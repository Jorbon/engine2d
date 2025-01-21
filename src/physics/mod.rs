

use crate::*;

fn get_force(entity: &Entity) -> Vec3<f64> { // MARK: get_force
	
	// pub fn movement(&mut self, mut input: Vec3<f64>) {
	// 	if input.is_zero() { return }
		
	// 	let (acceleration, speed) = self.get_acceleration_speed();
		
	// 	let input_length = input.length();
	// 	let input_direction = input.normalize();
		
	// 	let wish_acceleration = acceleration * input_length * dt;
	// 	let target_velocity = speed * input_length;
	// 	let current_velocity = self.velocity.dot(input_direction);
		
	// 	let (acceleration_parameter, deceleration_parameter) = match self.status {
	// 		EntityStatus::Grounded(_) => (2.0, 0.0),
	// 		EntityStatus::Falling => (1.0, 1.0),
	// 		EntityStatus::Swimming => (1.5, 0.5),
	// 	};
		
	// 	if current_velocity < -target_velocity { // reduced so that it doesn't act like 2x friction
	// 		self.velocity += input_direction * f64::min(deceleration_parameter * wish_acceleration, target_velocity - current_velocity);
	// 	} else if current_velocity < target_velocity { // 2x because 1x counters friction
	// 		self.velocity += input_direction * f64::min(acceleration_parameter * wish_acceleration, target_velocity - current_velocity);
	// 	}
		
	// 		 if input.y() < -input.x().abs() { self.direction = FacingDirection::Up; }
	// 	else if input.y() >  input.x().abs() { self.direction = FacingDirection::Down; }
	// 	else if input.x() < -input.y().abs() { self.direction = FacingDirection::Left; }
	// 	else if input.x() >  input.y().abs() { self.direction = FacingDirection::Right; }
	// }
	
	entity.movement_input * 5.0 + Vec3(0.0, 0.0, -9.8)
}



const SURFACE_MARGIN: f64 = 1e-4;

const MIN_V_BOUNCE: f64 = 0.1;


pub fn physics_step(entity: &mut Entity, cells: &HashMap<Vec3<isize>, Cell>, dt: f64, _debug: bool) { // MARK: Physics Step
	
	entity.velocity += get_force(entity) * dt;
	
		 if entity.movement_input.y() < -entity.movement_input.x().abs() { entity.direction = FacingDirection::Up; }
	else if entity.movement_input.y() >  entity.movement_input.x().abs() { entity.direction = FacingDirection::Down; }
	else if entity.movement_input.x() < -entity.movement_input.y().abs() { entity.direction = FacingDirection::Left; }
	else if entity.movement_input.x() >  entity.movement_input.y().abs() { entity.direction = FacingDirection::Right; }
	
	
	let mut l = entity.position + entity.size.scale(LOW_CORNER);
	let mut h = entity.position + entity.size.scale(HIGH_CORNER);
	
	let contacts = detect_contacts(cells, l, h);
	
	// todo: jump direction evaluation
	if entity.jump_input {
		for (normal, _) in &contacts {
			entity.velocity += *normal * 5.0;
			break
		}
	}
	
	let mut contacts_iter = std::iter::once(contacts);
	let mut dt_remaining = dt;
	
	loop {
		
		let contacts = match contacts_iter.next() {
			Some(contacts) => contacts,
			None => {
				l = entity.position + entity.size.scale(LOW_CORNER);
				h = entity.position + entity.size.scale(HIGH_CORNER);
				detect_contacts(cells, l, h)
			}
		};
		
		// todo: resolve displacements smarter
		// if _debug { dbg!(&contacts); }
		for (normal, displacement) in &contacts {
			// entity.position += *normal * displacement;
		}
		
		
		entity.velocity = constrain_velocity_with_contacts(entity.velocity, contacts.iter().map(|(normal, _)| *normal));
		
		
		if let Some((t, normal)) = detect_next_collision(entity, cells, l, h, dt_remaining) {
			entity.position += entity.velocity * t;
			
			// if _debug { dbg!(t, normal, entity.velocity); }
			
			let bounce = 0.0;
			
			let v_projected = entity.velocity.dot(normal);
			entity.velocity -= normal * v_projected * if v_projected < -MIN_V_BOUNCE {1.0 + bounce} else {1.0};
			
			dt_remaining -= t;
			
			// if _debug { dbg!(entity.velocity); }
			
			continue
		} else {
			entity.position += entity.velocity * dt_remaining;
			break
		}
	}
}


// MARK: Constrain Contacts

fn constrain_velocity_with_contacts(velocity: Vec3<f64>, contacts: impl Iterator<Item = Vec3<f64>>) -> Vec3<f64> {
	let mut opposing: Vec<Vec3<f64>> = vec![];
	let mut remainder = vec![];
	for normal in contacts {
		let list = if velocity.dot(normal) < 0.0 { &mut opposing } else { &mut remainder };
		
		let mut unique = true;
		for other_normal in list.iter() {
			if (normal.x() - other_normal.x()).abs() < 1e-7
			&& (normal.y() - other_normal.y()).abs() < 1e-7
			&& (normal.z() - other_normal.z()).abs() < 1e-7 {
				unique = false;
				break
			}
		}
		
		if unique {
			list.push(normal);
		}
	}
	
	match opposing.len() {
		0 => velocity,
		1 => {
			let normal1 = opposing[0];
			let new_velocity = velocity - normal1 * velocity.dot(normal1);
			
			opposing = vec![];
			let mut remainder2 = vec![];
			for normal in remainder {
				if new_velocity.dot(normal) < 0.0 {
					opposing.push(normal);
				} else {
					remainder2.push(normal);
				}
			}
			
			match opposing.len() {
				0 => new_velocity,
				1 => {
					let mut new_velocity_direction = normal1.cross(opposing[0]);
					if velocity.dot(new_velocity_direction) < 0.0 {
						new_velocity_direction = -new_velocity_direction;
					}
					
					for normal in remainder2 {
						if new_velocity_direction.dot(normal) < 0.0 {
							return Vec3::ZERO
						}
					}
					
					new_velocity_direction * velocity.dot(new_velocity_direction) / new_velocity_direction.length_squared()
				}
				n => {
					for i in 0..n {
						let mut new_velocity_direction = normal1.cross(opposing[i]);
						if velocity.dot(new_velocity_direction) < 0.0 {
							new_velocity_direction = -new_velocity_direction;
						}
						
						let mut valid = true;
						for normal in opposing.iter().chain(remainder2.iter()) {
							if normal == &opposing[i] { continue }
							if new_velocity_direction.dot(*normal) < 0.0 {
								valid = false;
								break
							}
						}
						
						if valid {
							return new_velocity_direction * velocity.dot(new_velocity_direction) / new_velocity_direction.length_squared()
						}
					}
					
					Vec3::ZERO
				}
			}
			
		}
		2 => {
			let mut new_velocity_direction = opposing[0].cross(opposing[1]);
			if velocity.dot(new_velocity_direction) < 0.0 {
				new_velocity_direction = -new_velocity_direction;
			}
			
			for normal in remainder {
				if new_velocity_direction.dot(normal) < 0.0 {
					return Vec3::ZERO
				}
			}
			
			new_velocity_direction * velocity.dot(new_velocity_direction) / new_velocity_direction.length_squared()
		}
		n => {
			for i in 0..n {
				for j in ((i+1)..n).rev() {
					let mut new_velocity_direction = opposing[i].cross(opposing[j]);
					if velocity.dot(new_velocity_direction) < 0.0 {
						new_velocity_direction = -new_velocity_direction;
					}
					
					let mut valid = true;
					for normal in opposing.iter().chain(remainder.iter()) {
						if normal == &opposing[i] || normal == &opposing[j] { continue }
						if new_velocity_direction.dot(*normal) < 0.0 {
							valid = false;
							break
						}
					}
					
					if valid {
						return new_velocity_direction * velocity.dot(new_velocity_direction) / new_velocity_direction.length_squared()
					}
				}
			}
			
			Vec3::ZERO
		}
	}
}



// MARK: Detect Contacts

// Decide what surfaces the hitbox is in contact with
fn detect_contacts(cells: &HashMap<Vec3<isize>, Cell>, l: Vec3<f64>, h: Vec3<f64>) -> Vec<(Vec3<f64>, f64)> {
	Vec3Range::<isize, ZYX>::inclusive(
		(l - Vec3::all(SURFACE_MARGIN)).floor_to(),
		(h + Vec3::all(SURFACE_MARGIN)).floor_to()
	).map(|tile_pos| test_contact(l, h, cells, tile_pos)).flatten().collect::<Vec<_>>()
}


fn test_contact(l: Vec3<f64>, h: Vec3<f64>, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>) -> Vec<(Vec3<f64>, f64)> {
	let cell_pos = tile_pos >> CELL_SIZE_BITS;
	if let Some(cell) = cells.get(&cell_pos) {
		let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
		
		match tile.state() {
			TileState::Empty => vec![],
			TileState::Full => match test_contact_full_block(l, h, tile_pos) {
				Some((direction, displacement)) => vec![(Vec3::<f64>::unit(direction), displacement)],
				None => vec![]
			}
			TileState::Partial => match test_contact_slope(l, h, tile_pos, tile.direction, tile.level) {
				Some(contact) => vec![contact],
				None => vec![]
			}
		}
	} else { vec![] }
}

fn test_contact_full_block(l: Vec3<f64>, h: Vec3<f64>, tile_pos: Vec3<isize>) -> Option<(Direction, f64)> {	
	let h_inset = h - tile_pos.as_type::<f64>();
	let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
	
	for a in [Z, Y, X] {
		if h_inset[a.l()] > 0.0 && l_inset[a.l()] > 0.0
		&& h_inset[a.r()] > 0.0 && l_inset[a.r()] > 0.0 {
			if h_inset[a].abs() < SURFACE_MARGIN {
				return Some((a.n(), h_inset[a]));
			}
			if l_inset[a].abs() < SURFACE_MARGIN {
				return Some((a.p(), l_inset[a]));
			}
		}
	}
	
	None
}

fn test_contact_slope(l: Vec3<f64>, h: Vec3<f64>, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8) -> Option<(Vec3<f64>, f64)> {
	
	{ // Decide if we even need to run this at all
		let mut positive_sum = 0;
		let mut negative_sum = 0;
		direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
		
		if level <= negative_sum { return None }
		if level >= positive_sum { return test_contact_full_block(l, h, tile_pos).map(|(d, displacement)| (Vec3::<f64>::unit(d), displacement)) }
	}
	
	
	let near_corner = Vec3::by_axis(|a| if direction[a] >= 0 {l[a]} else {h[a]});
	
	let slope_normal = direction.as_type::<f64>();
	let slope_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
	
	// let h_inset = h - tile_pos.as_type::<f64>();
	// let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
	
	
	// Main slope face
	for _ in std::iter::once(()) {
		let current_s = near_corner.dot(slope_normal);
		let s_inset = (slope_s - current_s) / slope_normal.length();
		
		if s_inset.abs() < SURFACE_MARGIN
		&& near_corner.x() >= tile_pos.x() as f64 && near_corner.x() <= tile_pos.x() as f64 + 1.0
		&& near_corner.y() >= tile_pos.y() as f64 && near_corner.y() <= tile_pos.y() as f64 + 1.0
		&& near_corner.z() >= tile_pos.z() as f64 && near_corner.z() <= tile_pos.z() as f64 + 1.0 {
			return Some((slope_normal.normalize(), s_inset))
		}
	}
	
	// Acute edges
	for a in [X, Y, Z] {
		if level >= direction[a].min(0) + direction[a.l()].max(0) + direction[a.r()].max(0) { continue }
		
		let edge_normal = slope_normal.get_plane(a);
		
		let plane_tile_pos = tile_pos[a];
		let tile_pos = tile_pos.get_plane(a);
		let near_edge = near_corner.get_plane(a);
		
		let edge_s = (tile_pos.dot(direction.get_plane(a).as_type::<isize>()) + (level - direction[a].min(0)) as isize) as f64;
		let current_s = near_edge.dot(edge_normal);
		let s_inset = (edge_s - current_s) / edge_normal.length();
		
		let plane_relative_position = if direction[a] >= 0 { plane_tile_pos } else { plane_tile_pos + 1 } as f64;
		
		if s_inset.abs() < SURFACE_MARGIN
		&& near_edge.x() >= tile_pos.x() as f64 && near_edge.x() <= tile_pos.x() as f64 + 1.0
		&& near_edge.y() >= tile_pos.y() as f64 && near_edge.y() <= tile_pos.y() as f64 + 1.0
		&& plane_relative_position >= l[a] && plane_relative_position <= h[a] {
			return Some((edge_normal.normalize().vec3(a), s_inset))
		}
	}
	
	// Acute corners
	for a in [X, Y, Z] {
		if level > direction[a].max(0) + direction[a.l()].min(0) + direction[a.r()].min(0) { continue }
		
		let corner_s = (tile_pos[a] * direction[a] as isize + (level - direction[a.l()].min(0) - direction[a.r()].min(0)) as isize) as f64;
		let current_s = near_corner[a] * slope_normal[a];
		let s_inset = (corner_s - current_s) / slope_normal[a];
		
		let corner_relative_position = Vec2(
			if direction[a.l()] >= 0 { tile_pos[a.l()] } else { tile_pos[a.l()] + 1 } as f64,
			if direction[a.r()] >= 0 { tile_pos[a.r()] } else { tile_pos[a.r()] + 1 } as f64,
		);
		
		if s_inset.abs() < SURFACE_MARGIN
		&& corner_relative_position.x() >= l[a.l()] && corner_relative_position.x() <= h[a.l()]
		&& corner_relative_position.y() >= l[a.r()] && corner_relative_position.y() <= h[a.r()] {
			return Some((Vec3::unit(match direction[a] >= 0 { true => a.p(), false => a.n() }), s_inset))
		}
	}
	
	// Regular face contact
	let (d, displacement) = test_contact_full_block(l, h, tile_pos)?;
	let a = d.axis();
	let contacting_corner = near_corner.with(a, if d.is_positive() {l[a]} else {h[a]});
	
	if contacting_corner.dot(slope_normal) < slope_s {
		Some((Vec3::unit(d), displacement))
	} else {
		None
	}
}


// MARK: Detect Next Collision

fn detect_next_collision(entity: &Entity, cells: &HashMap<Vec3<isize>, Cell>, l: Vec3<f64>, h: Vec3<f64>, dt_remaining: f64) -> Option<(f64, Vec3<f64>)> {
	let mut first_collision = None;
	let mut first_collision_t = dt_remaining;
	
	let reversed = entity.velocity.map(|v| v < 0.0);
	let step = reversed.map(|r| match r { false => 1, true => -1 });
	
	let main_corner = entity.position + entity.size.scale(Vec3::by_axis(|a| match reversed[a] { false => HIGH_CORNER[a], true => LOW_CORNER[a] }));
	let far_corner = entity.position + entity.size.scale(Vec3::by_axis(|a| match reversed[a] { false => LOW_CORNER[a], true => HIGH_CORNER[a] }));
	
	let mut main_tile = Vec3::by_axis(|a| match reversed[a] { false => main_corner[a].ceil() - 1.0, true => main_corner[a].floor() } as isize);
	let far_tile = Vec3::by_axis(|a| match reversed[a] { false => far_corner[a].floor(), true => far_corner[a].ceil() - 1.0 } as isize);
	
	// Check tiles that the hitbox is already inside for faces in direction of movement
	for (axis, check_axis) in [(Z, None), (Y, Some(Z)), (X, Some(Y))] {
		if let Some(ca) = check_axis {
			if main_tile[ca] == far_tile[ca] {
				break
			} else {
				main_tile[ca] -= step[ca];
			}
		}
		
		// dbg!(self.position, main_tile, far_tile);
		for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile.with(axis, main_tile[axis])) {
			if let Some(collision) = test_collision(l, h, entity.velocity, cells, tile_pos, first_collision_t) {
				first_collision = Some(collision);
				first_collision_t = collision.0;
			}
		}
	}
	
	
	// Next, visit each tile boundary encounter in chronological order
	
	let mut current_tile = main_corner.floor_to::<isize>();
	let mut next_tile_boundary = current_tile + reversed.map(|r| if r {0} else {1});
	
	while first_collision.is_none() {
		let t_next = Vec3::by_axis(|a| prel(main_corner[a], main_corner[a] + entity.velocity[a], next_tile_boundary[a] as f64)).map(|v| if v < 0.0 {f64::INFINITY} else {v});
		let a = match (t_next.x() < t_next.y(), t_next.x() < t_next.z(), t_next.y() < t_next.z()) {
			(true, true, _) => X,
			(false, _, true) => Y,
			(_, false, false) => Z,
			_ => unreachable!()
		};
		
		if t_next[a] > dt_remaining { break }
		
		current_tile += step.component(a);
		next_tile_boundary += step.component(a);
		
		let current_t = t_next[a];
		let current_main_pos = main_corner + entity.velocity * current_t;
		let current_far_pos = far_corner + entity.velocity * current_t;
		let mut main_tile = Vec3::by_axis(|a| match reversed[a] { false => current_main_pos[a].ceil() - 1.0, true => current_main_pos[a].floor() } as isize).with(a, current_tile[a]);
		let far_tile = Vec3::by_axis(|a| match reversed[a] { false => current_far_pos[a].floor(), true => current_far_pos[a].ceil() - 1.0 } as isize).with(a, current_tile[a]);
		
		// Edge case will make current_tile farther than main_tile, use current_tile coord if it is moving into in that direction
		// main_tile is less inclusive and allows smooth wall sliding (no velocity into wall)
		// current_tile is more inclusive and fixes corner clip (velocity into tile)
		if entity.velocity[a.l()] != 0.0 { main_tile[a.l()] = current_tile[a.l()] }
		if entity.velocity[a.r()] != 0.0 { main_tile[a.r()] = current_tile[a.r()] }
		
		for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile) {
			if let Some(collision) = test_collision(l, h, entity.velocity, cells, tile_pos, first_collision_t) {
				first_collision = Some(collision);
				first_collision_t = collision.0;
			}
		}
	}
	
	first_collision
}



// MARK: Test Collision

fn test_collision(l: Vec3<f64>, h: Vec3<f64>, velocity: Vec3<f64>, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Vec3<f64>)> {
	let cell_pos = tile_pos >> CELL_SIZE_BITS;
	if let Some(cell) = cells.get(&cell_pos) {
		let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
		
		match tile.state() {
			TileState::Empty => None,
			TileState::Full => test_collision_full_block(l, h, velocity, tile_pos, max_t).map(|c| (c.0, Vec3::unit(c.1))),
			TileState::Partial => test_collision_slope(l, h, velocity, tile_pos, tile.direction, tile.level, max_t),
		}
	} else { None }
}

fn test_collision_full_block(l: Vec3<f64>, h: Vec3<f64>, velocity: Vec3<f64>, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Direction)> {
	for a in [Z, Y, X] {
		if velocity[a] < 0.0 {
			let t = prel(l[a], l[a] + velocity[a], tile_pos[a] as f64 + 1.0);
			if t >= 0.0 && t <= max_t {
				return Some((t, a.p()))
			}
		} else if velocity[a] > 0.0 {
			let t = prel(h[a], h[a] + velocity[a], tile_pos[a] as f64);
			if t >= 0.0 && t <= max_t {
				return Some((t, a.n()))
			}
		}
	}
	
	None
}


fn test_collision_slope(l: Vec3<f64>, h: Vec3<f64>, velocity: Vec3<f64>, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8, max_t: f64) -> Option<(f64, Vec3<f64>)> {
	
	{ // Decide if we even need to run this at all
		let mut positive_sum = 0;
		let mut negative_sum = 0;
		direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
		
		if level <= negative_sum { return None }
		if level >= positive_sum { return test_collision_full_block(l, h, velocity, tile_pos, max_t).map(|c| (c.0, Vec3::unit(c.1))) }
	}
	
	
	let near_corner = Vec3::by_axis(|a| if direction[a] >= 0 {l[a]} else {h[a]});
	
	let slope_normal = direction.as_type::<f64>();
	let slope_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
	
	
	// Main slope face
	for _ in std::iter::once(()) {
		let s_velocity = velocity.dot(slope_normal);
		if s_velocity > -SURFACE_MARGIN { continue }
		
		let current_s = near_corner.dot(slope_normal);
		if current_s < slope_s { continue }
		
		let t = (slope_s - current_s) / s_velocity;
		if t > max_t { return None }
		
		let near_corner_pos = near_corner + velocity * t;
		if near_corner_pos.x() >= tile_pos.x() as f64 && near_corner_pos.x() <= tile_pos.x() as f64 + 1.0
		&& near_corner_pos.y() >= tile_pos.y() as f64 && near_corner_pos.y() <= tile_pos.y() as f64 + 1.0
		&& near_corner_pos.z() >= tile_pos.z() as f64 && near_corner_pos.z() <= tile_pos.z() as f64 + 1.0 {
			return Some((t, slope_normal.normalize()));
		}
	}
	
	// Acute edges
	for a in [X, Y, Z] {
		if level >= direction[a].min(0) + direction[a.l()].max(0) + direction[a.r()].max(0) { continue }
		
		let plane_velocity = velocity.get_plane(a);
		let edge_normal = slope_normal.get_plane(a);
		
		let s_velocity = plane_velocity.dot(edge_normal);
		if s_velocity > -SURFACE_MARGIN { continue }
		
		let plane_tile_pos = tile_pos[a];
		let tile_pos = tile_pos.get_plane(a);
		let near_edge = near_corner.get_plane(a);
		
		let edge_s = (tile_pos.dot(direction.get_plane(a).as_type::<isize>()) + (level - direction[a].min(0)) as isize) as f64;
		let current_s = near_edge.dot(edge_normal);
		if current_s < edge_s { continue }
		
		let t = (edge_s - current_s) / s_velocity;
		if t > max_t { return None }
		
		let plane_relative_position = if direction[a] >= 0 { plane_tile_pos } else { plane_tile_pos + 1 } as f64 - velocity[a] * t;
		let near_edge_pos = near_edge + plane_velocity * t;
		
		if near_edge_pos.x() >= tile_pos.x() as f64 && near_edge_pos.x() <= tile_pos.x() as f64 + 1.0
		&& near_edge_pos.y() >= tile_pos.y() as f64 && near_edge_pos.y() <= tile_pos.y() as f64 + 1.0
		&& plane_relative_position >= l[a] && plane_relative_position <= h[a] {
			return Some((t, edge_normal.normalize().vec3(a)))
		}
	}
	
	// Acute corners
	for a in [X, Y, Z] {
		if level > direction[a].max(0) + direction[a.l()].min(0) + direction[a.r()].min(0) { continue }
		
		let s_velocity = velocity[a] * slope_normal[a];
		if s_velocity > 0.0 { continue }
		
		let corner_s = (tile_pos[a] * direction[a] as isize + (level - direction[a.l()].min(0) - direction[a.r()].min(0)) as isize) as f64;
		let current_s = near_corner[a] * slope_normal[a];
		if current_s < corner_s { continue }
		
		let t = (corner_s - current_s) / s_velocity;
		if t > max_t { return None }
		
		// let near_face_pos = near_corner[a] + self.velocity[a] * t;
		let corner_relative_position = Vec2(
			if direction[a.l()] >= 0 { tile_pos[a.l()] } else { tile_pos[a.l()] + 1 } as f64 - velocity[a.l()] * t,
			if direction[a.r()] >= 0 { tile_pos[a.r()] } else { tile_pos[a.r()] + 1 } as f64 - velocity[a.r()] * t,
		);
		
		if /*near_face_pos >= tile_pos[a] as f64 && near_face_pos <= tile_pos[a] as f64 + 1.0
		&&*/ corner_relative_position.x() >= l[a.l()] && corner_relative_position.x() <= h[a.l()]
		&& corner_relative_position.y() >= l[a.r()] && corner_relative_position.y() <= h[a.r()] {
			return Some((t, Vec3::unit(match direction[a] >= 0 { true => a.p(), false => a.n() })))
		}
	}
	
	// Regular face collisions
	let (t, d) = test_collision_full_block(l, h, velocity, tile_pos, max_t)?;
	let a = d.axis();
	let colliding_corner = near_corner.with(a, if d.is_positive() {l[a]} else {h[a]}) + velocity * t;
	
	if colliding_corner.dot(slope_normal) < slope_s {
		Some((t, Vec3::unit(d)))
	} else {
		None
	}
}


