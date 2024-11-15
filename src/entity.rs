use std::{collections::HashMap, path::Path};
use glium::{texture::SrgbTexture2d, Display};

use crate::*;



pub enum FacingDirection { Up, Down, Left, Right }

pub struct Entity {
    pub position: Vec3<f64>,
    pub velocity: Vec3<f64>,
    pub size: Vec3<f64>,
    pub direction: FacingDirection,
    pub ground_speed: f64,
	pub air_speed: f64,
	pub water_speed: f64,
	pub ground_acceleration: f64,
	pub air_acceleration: f64,
	pub water_acceleration: f64,
	pub air_resistance: f64,
    pub sprites: SpriteSet,
	pub movement_input: Vec3<f64>,
	pub jump_input: bool,
	pub mesh_buffers: Option<(VertexBuffer<ModelVertex>, IndexBuffer<ModelIndex>)>,
}

pub enum SpriteSet {
	Static(SrgbTexture2d),
	Directional([SrgbTexture2d; 4]),
}

impl SpriteSet {
	pub fn load(display: &Display, path: &str) -> Self {
		let full_path = format!("assets/textures/{path}.png");
		if Path::new(&full_path).is_file() {
			SpriteSet::Static(load_texture(display, path))
		} else {
			SpriteSet::Directional([
				load_texture(display, &format!("{path}/up")),
				load_texture(display, &format!("{path}/down")),
				load_texture(display, &format!("{path}/left")),
				load_texture(display, &format!("{path}/right")),
			])
		}
	}
}



pub const LOW_CORNER: Vec3<f64> = Vec3(-0.5, -0.5, 0.0);
pub const HIGH_CORNER: Vec3<f64> = Vec3(0.5, 0.5, 1.0);

const SURFACE_MARGIN: f64 = 1e-4;


impl Entity {
	pub fn new(position: Vec3<f64>, size: Vec3<f64>, sprites: SpriteSet) -> Self {
		Self {
			position,
			velocity: Vec3(0.0, 0.0, 0.0),
			size,
			direction: FacingDirection::Down,
			ground_speed: 5.0,
			air_speed: 12.0,
			water_speed: 3.0,
			ground_acceleration: 60.0,
			air_acceleration: 4.0,
			water_acceleration: 25.0,
			air_resistance: 0.001,
			sprites,
			movement_input: Vec3(0.0, 0.0, 0.0),
			jump_input: false,
			mesh_buffers: None,
		}
	}
	
	pub fn current_sprite(&self) -> &SrgbTexture2d {
		match &self.sprites {
			SpriteSet::Static(sprite) => &sprite,
			SpriteSet::Directional([up, down, left, right]) => match self.direction {
				FacingDirection::Up => &up,
				FacingDirection::Down => &down,
				FacingDirection::Left => &left,
				FacingDirection::Right => &right,
			}
		}
	}
	
	pub fn build_mesh_buffers(&mut self, display: &Display) {
		let l = self.size.scale(LOW_CORNER).as_type();
		let h = self.size.scale(HIGH_CORNER).as_type();
		
		self.mesh_buffers = Some((
			VertexBuffer::new(display, &[
				ModelVertex { position: Vec3(l.x(), l.y(), l.z() + 0.01), normal: Vec3::Z, uv: Vec2(0.0, 0.0) },
				ModelVertex { position: Vec3(l.x(), h.y(), l.z() + 0.01), normal: Vec3::Z, uv: Vec2(0.0, 1.0) },
				ModelVertex { position: Vec3(h.x(), h.y(), l.z() + 0.01), normal: Vec3::Z, uv: Vec2(1.0, 1.0) },
				ModelVertex { position: Vec3(h.x(), l.y(), l.z() + 0.01), normal: Vec3::Z, uv: Vec2(1.0, 0.0) },
			]).unwrap(),
			IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 2, 3]).unwrap(),
		));
	}
	
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
	
	pub fn get_force(&self) -> Vec3<f64> {
		self.movement_input * 5.0 + Vec3(0.0, 0.0, -9.8)
	}
	
	pub fn physics_step(&mut self, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) {
		
		// First decide what surfaces the hitbox is in contact with
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		
		let contacts = Vec3Range::<isize, ZYX>::inclusive((l - Vec3::all(SURFACE_MARGIN)).floor_to(), (h + Vec3::all(SURFACE_MARGIN)).floor_to()).map(|tile_pos| self.test_contact(cells, tile_pos)).flatten().collect::<Vec<_>>();
		
		// todo: resolve displacements
		
		self.velocity += self.get_force() * dt;
		
		     if self.movement_input.y() < -self.movement_input.x().abs() { self.direction = FacingDirection::Up; }
		else if self.movement_input.y() >  self.movement_input.x().abs() { self.direction = FacingDirection::Down; }
		else if self.movement_input.x() < -self.movement_input.y().abs() { self.direction = FacingDirection::Left; }
		else if self.movement_input.x() >  self.movement_input.y().abs() { self.direction = FacingDirection::Right; }
		
		// todo: directional jumps
		if self.jump_input {
			for (normal, _) in &contacts {
				self.velocity += *normal * 5.0;
				break;
			}
		}
		
		for (normal, _) in contacts {
			if self.velocity.dot(normal) < 0.0 {
				self.velocity -= normal * self.velocity.dot(normal);
			}
		}
		
		
		
		// Main collider loop
		
		let mut dt_remaining = dt;
		
		loop {
			
			let mut first_collision = None;
			let mut first_collision_t = dt_remaining;
			
			let reversed = self.velocity.map(|v| v < 0.0);
			let step = reversed.map(|r| match r { false => 1, true => -1 });
			
			let main_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => HIGH_CORNER[a], true => LOW_CORNER[a] }));
			let far_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => LOW_CORNER[a], true => HIGH_CORNER[a] }));
			
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
					if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
						first_collision = Some(collision);
						first_collision_t = collision.0;
					}
				}
			}
			
			
			// Next, visit each tile boundary encounter in chronological order
			
			let mut current_tile = main_corner.floor_to::<isize>();
			let mut next_tile_boundary = current_tile + reversed.map(|r| if r {0} else {1});
			
			while first_collision.is_none() {
				let t_next = Vec3::by_axis(|a| prel(main_corner[a], main_corner[a] + self.velocity[a], next_tile_boundary[a] as f64)).map(|v| if v < 0.0 {f64::INFINITY} else {v});
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
				let current_main_pos = main_corner + self.velocity * current_t;
				let current_far_pos = far_corner + self.velocity * current_t;
				let mut main_tile = Vec3::by_axis(|a| match reversed[a] { false => current_main_pos[a].ceil() - 1.0, true => current_main_pos[a].floor() } as isize).with(a, current_tile[a]);
				let far_tile = Vec3::by_axis(|a| match reversed[a] { false => current_far_pos[a].floor(), true => current_far_pos[a].ceil() - 1.0 } as isize).with(a, current_tile[a]);
				
				// Edge case will make current_tile farther than main_tile, use current_tile coord if it is moving into in that direction
				// main_tile is less inclusive and allows smooth wall sliding (no velocity into wall)
				// current_tile is more inclusive and fixes corner clip (velocity into tile)
				if self.velocity[a.l()] != 0.0 { main_tile[a.l()] = current_tile[a.l()] }
				if self.velocity[a.r()] != 0.0 { main_tile[a.r()] = current_tile[a.r()] }
				
				for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile) {
					if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
						first_collision = Some(collision);
						first_collision_t = collision.0;
					}
				}
			}
			
			if let Some((t, normal)) = first_collision {
				self.position += self.velocity * t;
				self.velocity -= normal * self.velocity.dot(normal) * 1.0;
				dt_remaining -= t;
				// println!("{t}, {dt_remaining}, {normal:?}, {:?}", self.velocity);
				continue
			} else {
				self.position += self.velocity * dt_remaining;
				break
			}
		}
	}
	
	fn test_contact(&self, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>) -> Vec<(Vec3<f64>, f64)> {
		let cell_pos = tile_pos >> CELL_SIZE_BITS;
		if let Some(cell) = cells.get(&cell_pos) {
			let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
			
			match tile.state() {
				TileState::Empty => vec![],
				TileState::Full => match self.test_contact_full_block(tile_pos) {
					Some((direction, displacement)) => vec![(Vec3::<f64>::unit(direction), displacement)],
					None => vec![]
				}
				TileState::Partial => match self.test_contact_slope(tile_pos, tile.direction, tile.level) {
					Some(contact) => vec![contact],
					None => vec![]
				}
			}
		} else { vec![] }
	}
	
	fn test_contact_full_block(&self, tile_pos: Vec3<isize>) -> Option<(Direction, f64)> {
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		
		let h_inset = h - tile_pos.as_type::<f64>();
		let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
		
		for a in [Z, Y, X] {
			if h_inset[a.l()] > SURFACE_MARGIN && l_inset[a.l()] > SURFACE_MARGIN
			&& h_inset[a.r()] > SURFACE_MARGIN && l_inset[a.r()] > SURFACE_MARGIN {
				if h_inset[a].abs() < SURFACE_MARGIN {
					let displacement = h[a] - tile_pos[a] as f64;
					return Some((a.n(), displacement));
				}
				if l_inset[a].abs() < SURFACE_MARGIN {
					let displacement = (tile_pos[a] + 1) as f64 - l[a];
					return Some((a.p(), displacement));
				}
			}
		}
		
		None
	}
	
	fn test_contact_slope(&self, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8) -> Option<(Vec3<f64>, f64)> {
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		let near_corner = Vec3::by_axis(|a| if direction[a] >= 0 {l[a]} else {h[a]});
		
		let ramp_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
		let near_corner_s = near_corner.dot(direction.as_type::<f64>());
		if (near_corner_s - ramp_s).abs() < SURFACE_MARGIN * direction.as_type::<f64>().length()
		&& near_corner.x() >= tile_pos.x() as f64 && near_corner.x() <= tile_pos.x() as f64 + 1.0
		&& near_corner.y() >= tile_pos.y() as f64 && near_corner.y() <= tile_pos.y() as f64 + 1.0
		&& near_corner.z() >= tile_pos.z() as f64 && near_corner.z() <= tile_pos.z() as f64 + 1.0 {
			let displacement = (ramp_s - near_corner_s) / direction.as_type::<f64>().length();
			Some((direction.as_type::<f64>().normalize(), displacement))
		} else if near_corner_s < ramp_s {
			self.test_contact_full_block(tile_pos).map(|(d, displacement)| (Vec3::<f64>::unit(d), displacement))
		} else {
			None
		}
	}
	
	
	
	pub fn test_collision(&self, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Vec3<f64>)> {
		let cell_pos = tile_pos >> CELL_SIZE_BITS;
		if let Some(cell) = cells.get(&cell_pos) {
			let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
			
			match tile.state() {
				TileState::Empty => None,
				TileState::Full => self.test_collision_full_block(tile_pos, max_t).map(|c| (c.0, Vec3::unit(c.1))),
				TileState::Partial => self.test_collision_slope(tile_pos, tile.direction, tile.level, max_t),
			}
		} else { None }
	}
	
	fn test_collision_full_block(&self, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Direction)> {
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		for a in [Z, Y, X] {
			if self.velocity[a] < 0.0 {
				let t = prel(l[a], l[a] + self.velocity[a], tile_pos[a] as f64 + 1.0);
				if t >= 0.0 && t <= max_t {
					return Some((t, a.p()))
				}
			} else if self.velocity[a] > 0.0 {
				let t = prel(h[a], h[a] + self.velocity[a], tile_pos[a] as f64);
				if t >= 0.0 && t <= max_t {
					return Some((t, a.n()))
				}
			}
		}
		None
	}
	
	fn test_collision_slope(&self, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8, max_t: f64) -> Option<(f64, Vec3<f64>)> {
		
		{ // Decide if we even need to run this at all
			let mut positive_sum = 0;
			let mut negative_sum = 0;
			direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
			
			if level <= negative_sum { return None }
			if level >= positive_sum { return self.test_collision_full_block(tile_pos, max_t).map(|c| (c.0, Vec3::unit(c.1))) }
		}
		
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		
		let near_corner = Vec3::by_axis(|a| if direction[a] >= 0 {l[a]} else {h[a]});
		
		let slope_normal = direction.as_type::<f64>();
		let slope_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
		
		
		// Main slope face
		for _ in std::iter::once(()) {
			let s_velocity = self.velocity.dot(slope_normal);
			if s_velocity > -SURFACE_MARGIN { continue }
			
			let current_s = near_corner.dot(slope_normal);
			if current_s < slope_s { continue }
			
			let t = (slope_s - current_s) / s_velocity;
			if t > max_t { return None }
			
			let near_corner_pos = near_corner + self.velocity * t;
			if near_corner_pos.x() >= tile_pos.x() as f64 && near_corner_pos.x() <= tile_pos.x() as f64 + 1.0
			&& near_corner_pos.y() >= tile_pos.y() as f64 && near_corner_pos.y() <= tile_pos.y() as f64 + 1.0
			&& near_corner_pos.z() >= tile_pos.z() as f64 && near_corner_pos.z() <= tile_pos.z() as f64 + 1.0 {
				return Some((t, slope_normal.normalize()));
			}
		}
		
		// Concave edges
		for a in [X, Y, Z] {
			if level >= direction[a].min(0) + direction[a.l()].max(0) + direction[a.r()].max(0) { continue }
			
			let velocity = self.velocity.get_plane(a);
			let edge_normal = slope_normal.get_plane(a);
			
			let s_velocity = velocity.dot(edge_normal);
			if s_velocity > -SURFACE_MARGIN { continue }
			
			let plane_position = tile_pos[a];
			let tile_pos = tile_pos.get_plane(a);
			let near_edge = near_corner.get_plane(a);
			
			let edge_s = (tile_pos.dot(direction.get_plane(a).as_type::<isize>()) + (level - direction[a].min(0)) as isize) as f64;
			let current_s = near_edge.dot(edge_normal);
			if current_s < edge_s { continue }
			
			let t = (edge_s - current_s) / s_velocity;
			if t > max_t { return None }
			
			let plane_position = if direction[a] >= 0 { plane_position } else { plane_position + 1 } as f64 - self.velocity[a] * t;
			let near_edge_pos = near_edge + velocity * t;
			
			if near_edge_pos.x() >= tile_pos.x() as f64 && near_edge_pos.x() <= tile_pos.x() as f64 + 1.0
			&& near_edge_pos.y() >= tile_pos.y() as f64 && near_edge_pos.y() <= tile_pos.y() as f64 + 1.0
			&& plane_position >= l[a] && plane_position <= h[a] {
				return Some((t, edge_normal.normalize().vec3(a)))
			}
		}
		
		// Concave corners
		for a in [X, Y, Z] {
			if level > direction[a].max(0) + direction[a.l()].min(0) + direction[a.r()].min(0) { continue }
			
			let s_velocity = self.velocity[a] * slope_normal[a];
			if s_velocity >= 0.0 { continue }
			
			let corner_s = (tile_pos[a] * direction[a] as isize + (level - direction[a.l()].min(0) - direction[a.r()].min(0)) as isize) as f64;
			let current_s = near_corner[a] * slope_normal[a];
			if current_s < corner_s { continue }
			
			let t = (corner_s - current_s) / s_velocity;
			if t > max_t { return None }
			
			// let near_face_pos = near_corner[a] + self.velocity[a] * t;
			let corner_position = Vec2(
				if direction[a.l()] >= 0 { tile_pos[a.l()] } else { tile_pos[a.l()] + 1 } as f64 - self.velocity[a.l()] * t,
				if direction[a.r()] >= 0 { tile_pos[a.r()] } else { tile_pos[a.r()] + 1 } as f64 - self.velocity[a.r()] * t,
			);
			
			if /*near_face_pos >= tile_pos[a] as f64 && near_face_pos <= tile_pos[a] as f64 + 1.0
			&&*/ corner_position.x() >= l[a.l()] && corner_position.x() <= h[a.l()]
			&& corner_position.y() >= l[a.r()] && corner_position.y() <= h[a.r()] {
				return Some((t, Vec3::unit(match direction[a] >= 0 { true => a.p(), false => a.n() })))
			}
		}
		
		// Regular face collisions
		let (t, d) = self.test_collision_full_block(tile_pos, max_t)?;
		let a = d.axis();
		let colliding_corner = near_corner.with(a, if d.is_positive() { l[a] } else { h[a] }) + self.velocity * t;
		
		if colliding_corner.dot(slope_normal) < slope_s {
			Some((t, Vec3::unit(d)))
		} else {
			None
		}
	}
}
