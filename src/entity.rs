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
		Vec3(0.0, 0.0, -9.8)
	}
	
	pub fn physics_step(&mut self, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) {
		let mut normals = vec![];
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		
		for tile_pos in Vec3Range::<isize, ZYX>::inclusive((l - Vec3::all(SURFACE_MARGIN)).floor_to(), (h + Vec3::all(SURFACE_MARGIN)).floor_to()) {
			let cell_pos = tile_pos >> CELL_SIZE_BITS;
			if let Some(cell) = cells.get(&cell_pos) {
				let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
				
				match tile.state() {
					TileState::Empty => (),
					TileState::Full => self.block_touching(&mut normals, tile_pos, l, h),
					TileState::Partial => {
						let near_corner = Vec3::by_axis(|a| if tile.direction[a] >= 0 {l[a]} else {h[a]});
						
						let ramp_s = (tile_pos.dot(tile.direction.as_type::<isize>()) + tile.level as isize) as f64;
						let near_corner_s = near_corner.dot(tile.direction.as_type::<f64>());
						if (near_corner_s - ramp_s).abs() < SURFACE_MARGIN * tile.direction.as_type::<f64>().length()
						&& near_corner.x() >= tile_pos.x() as f64 && near_corner.x() <= tile_pos.x() as f64 + 1.0
						&& near_corner.y() >= tile_pos.y() as f64 && near_corner.y() <= tile_pos.y() as f64 + 1.0
						&& near_corner.z() >= tile_pos.z() as f64 && near_corner.z() <= tile_pos.z() as f64 + 1.0 {
							self.position += tile.direction.as_type::<f64>() * (ramp_s - near_corner_s) / tile.direction.as_type::<f64>().length_squared();
							normals.push(tile.direction.as_type::<f64>().normalize());
						} else if near_corner_s < ramp_s {
							self.block_touching(&mut normals, tile_pos, l, h);
						}
					}
				}
			}
		}
		
		
		self.velocity += self.get_force() * dt;
		
		self.velocity += self.movement_input * dt * 5.0;
		
		     if self.movement_input.y() < -self.movement_input.x().abs() { self.direction = FacingDirection::Up; }
		else if self.movement_input.y() >  self.movement_input.x().abs() { self.direction = FacingDirection::Down; }
		else if self.movement_input.x() < -self.movement_input.y().abs() { self.direction = FacingDirection::Left; }
		else if self.movement_input.x() >  self.movement_input.y().abs() { self.direction = FacingDirection::Right; }
		
		if self.jump_input {
			for normal in &normals {
				if normal.z() > 0.0 {
					self.velocity -= self.get_force().normalize_or_zero() * 5.0;
					break;
				}
			}
		}
		
		for normal in normals {
			if self.velocity.dot(normal) < 0.0 {
				self.velocity -= normal * self.velocity.dot(normal);
			}
		}
		
		
		
		let mut dt_remaining = dt;
		
		loop {
			
			let mut first_collision = None;
			let mut first_collision_t = dt_remaining;
			
			let reversed = self.velocity.map(|v| v < 0.0);
			let step = reversed.map(|r| match r { false => 1, true => -1 });
			
			let main_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => HIGH_CORNER[a], true => LOW_CORNER[a] }));
			let far_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => LOW_CORNER[a], true => HIGH_CORNER[a] }));
			
			let mut main_tile = Vec3::by_axis(|a| if reversed[a] { main_corner[a].floor() } else { main_corner[a].ceil() - 1.0 } as isize);
			let far_tile = Vec3::by_axis(|a| if reversed[a] { far_corner[a].ceil() - 1.0 } else { far_corner[a].floor() } as isize);
			
			for (axis, check_axis) in [(Z, None), (Y, Some(Z)), (X, Some(Y))] {
				if let Some(ca) = check_axis {
					if main_tile[ca] == far_tile[ca] {
						break
					} else {
						main_tile[ca] -= step[ca];
					}
				}
				
				for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile.with(axis, main_tile[axis])) {
					if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
						first_collision = Some(collision);
						first_collision_t = collision.0;
					}
				}
			}
			
			if first_collision.is_none() {
				let mut current_tile = main_corner.floor_to::<isize>();
				let mut next_tile_boundary = current_tile + reversed.map(|r| if r {0} else {1});
				
				loop {
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
					let main_tile = Vec3::by_axis(|a| if reversed[a] { current_main_pos[a].floor() } else { current_main_pos[a].ceil() - 1.0 } as isize).with(a, current_tile[a]);
					let far_tile = Vec3::by_axis(|a| if reversed[a] { current_far_pos[a].ceil() - 1.0 } else { current_far_pos[a].floor() } as isize).with(a, current_tile[a]);
					for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile) {
						if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
							first_collision = Some(collision);
							first_collision_t = collision.0;
						}
					}
				}
			}
			
			if let Some((t, normal)) = first_collision {
				self.position += self.velocity * t;
				self.velocity -= normal * self.velocity.dot(normal);
				dt_remaining -= t;
				// println!("{t}, {dt_remaining}, {normal:?}, {:?}", self.velocity);
				continue
			} else {
				self.position += self.velocity * dt_remaining;
				break
			}
			
		}
	}
	
	pub fn test_collision(&self, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Vec3<f64>)> {
		let cell_pos = tile_pos >> CELL_SIZE_BITS;
		if let Some(cell) = cells.get(&cell_pos) {
			let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
			
			match tile.state() {
				TileState::Empty => None,
				TileState::Full => self.block_collision(tile_pos, max_t),
				TileState::Partial => {
					let l = self.position + self.size.scale(LOW_CORNER);
					let h = self.position + self.size.scale(HIGH_CORNER);
					
					if self.velocity.dot(tile.direction.as_type::<f64>()) < -SURFACE_MARGIN {
						let mut positive_sum = 0;
						let mut negative_sum = 0;
						tile.direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
						
						if tile.level <= negative_sum { return None }
						if tile.level >= positive_sum { return self.block_collision(tile_pos, max_t) }
						
						
						
						let near_corner = Vec3::by_axis(|a| if tile.direction[a] >= 0 {l[a]} else {h[a]});
						let ramp_s = (tile_pos.dot(tile.direction.as_type::<isize>()) + tile.level as isize) as f64;
						let current_s = near_corner.dot(tile.direction.as_type::<f64>());
						let next_s = (near_corner + self.velocity).dot(tile.direction.as_type::<f64>());
						let t = prel(current_s, next_s, ramp_s);
						
						let near_corner_pos = near_corner + self.velocity * t;
						if t >= 0.0
						&& near_corner_pos.x() >= tile_pos.x() as f64 && near_corner_pos.x() <= tile_pos.x() as f64 + 1.0
						&& near_corner_pos.y() >= tile_pos.y() as f64 && near_corner_pos.y() <= tile_pos.y() as f64 + 1.0
						&& near_corner_pos.z() >= tile_pos.z() as f64 && near_corner_pos.z() <= tile_pos.z() as f64 + 1.0 {
							if t <= max_t {
								return Some((t, tile.direction.as_type::<f64>().normalize()))
							} else {
								return None
							}
						} else {
							return self.block_collision(tile_pos, max_t)
						}
					}
					
					None
				}
			}
		} else {
			None
		}
	}
	
	fn block_touching(&mut self, normals: &mut Vec<Vec3<f64>>, tile_pos: Vec3<isize>, l: Vec3<f64>, h: Vec3<f64>) {
		let h_inset = h - tile_pos.as_type::<f64>();
		let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
		
		for a in [Z, Y, X] {
			if h_inset[a.l()] > SURFACE_MARGIN && l_inset[a.l()] > SURFACE_MARGIN
			&& h_inset[a.r()] > SURFACE_MARGIN && l_inset[a.r()] > SURFACE_MARGIN {
				if h_inset[a].abs() < SURFACE_MARGIN {
					self.position[a] = tile_pos[a] as f64 - self.size[a] * HIGH_CORNER[a];
					normals.push(Vec3::<f64>::unit(a.n()));
				}
				if l_inset[a].abs() < SURFACE_MARGIN {
					self.position[a] = (tile_pos[a] + 1) as f64 - self.size[a] * LOW_CORNER[a];
					normals.push(Vec3::unit(a.p()));
				}
			}
		}
	}
	
	fn block_collision(&self, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Vec3<f64>)> {
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		for a in [Z, Y, X] {
			if self.velocity[a] < 0.0 {
				let t = prel(l[a], l[a] + self.velocity[a], tile_pos[a] as f64 + 1.0);
				if t > 0.0 && t <= max_t {
					return Some((t, Vec3::unit(a.p())))
				}
			} else if self.velocity[a] > 0.0 {
				let t = prel(h[a], h[a] + self.velocity[a], tile_pos[a] as f64);
				if t > 0.0 && t <= max_t {
					return Some((t, Vec3::<f64>::unit(a.n())))
				}
			}
		}
		None
	}
}
