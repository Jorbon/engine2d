use crate::*;


#[derive(Copy, Clone, Debug)]
pub struct Collision {
	pub normal: Vec3<f64>,
	pub material: Material,
	pub dt: f64,
}


// MARK: Detect Next Collision

// Very similar to raycast algorithm, but different enough to not use raycast since it has to cover a whole volume

pub fn detect_next_collision(entity: &Entity, cells: &HashMap<Vec3<isize>, Cell>, l: Vec3<f64>, h: Vec3<f64>, dt_remaining: f64) -> Option<Collision> {
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
		
		for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile.with(axis, main_tile[axis])) {
			if let Some(collision) = test_collision(l, h, entity.velocity, cells, tile_pos, first_collision_t) {
				first_collision = Some(collision);
				first_collision_t = collision.dt;
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
				first_collision_t = collision.dt;
			}
		}
	}
	
	first_collision
}



// MARK: Test Collision

fn test_collision(l: Vec3<f64>, h: Vec3<f64>, velocity: Vec3<f64>, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>, max_t: f64) -> Option<Collision> {
	let cell_pos = tile_pos >> CELL_SIZE_BITS;
	if let Some(cell) = cells.get(&cell_pos) {
		let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
		
		match tile.state() {
			TileState::Empty => None,
			TileState::Full => test_collision_full_block(l, h, velocity, tile_pos, max_t).map(|(dt, direction)| Collision {
				normal: Vec3::unit(direction),
				material: tile.material,
				dt,
			}),
			TileState::Partial => test_collision_slope(l, h, velocity, tile_pos, tile.direction, tile.level, max_t).map(|(dt, normal)| Collision {
				normal,
				material: tile.material,
				dt,
			}),
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
		if near_corner_pos.x() + 1e-10 >= tile_pos.x() as f64 && near_corner_pos.x() <= tile_pos.x() as f64 + 1.0 + 1e-10
		&& near_corner_pos.y() + 1e-10 >= tile_pos.y() as f64 && near_corner_pos.y() <= tile_pos.y() as f64 + 1.0 + 1e-10
		&& near_corner_pos.z() + 1e-10 >= tile_pos.z() as f64 && near_corner_pos.z() <= tile_pos.z() as f64 + 1.0 + 1e-10 {
			return Some((t, slope_normal.normalize()))
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
		
		if near_edge_pos.x() + 1e-10 >= tile_pos.x() as f64 && near_edge_pos.x() <= tile_pos.x() as f64 + 1.0 + 1e-10
		&& near_edge_pos.y() + 1e-10 >= tile_pos.y() as f64 && near_edge_pos.y() <= tile_pos.y() as f64 + 1.0 + 1e-10
		&& plane_relative_position > l[a] && plane_relative_position < h[a] {
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
		&&*/ corner_relative_position.x() > l[a.l()] && corner_relative_position.x() < h[a.l()]
		&& corner_relative_position.y() > l[a.r()] && corner_relative_position.y() < h[a.r()] {
			return Some((t, Vec3::unit(match direction[a] >= 0 { true => a.p(), false => a.n() })))
		}
	}
	
	// Regular face collisions
	let (t, d) = test_collision_full_block(l, h, velocity, tile_pos, max_t)?;
	let a = d.axis();
	let colliding_corner = near_corner.with(a, if d.is_positive() {l[a]} else {h[a]}) + velocity * t;
	let colliding_point = Vec3::by_axis(|a| colliding_corner[a].clamp(tile_pos[a] as f64, tile_pos[a] as f64 + 1.0));
	
	if colliding_point.dot(slope_normal) + 1e-10 < slope_s {
		Some((t, Vec3::unit(d)))
	} else {
		None
	}
}


