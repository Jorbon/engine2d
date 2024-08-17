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

#[derive(PartialEq)]
pub enum EntityStatus {
	Grounded(Vec<Vec3<f64>>),
	Falling,
	Swimming,
}

pub enum SpriteSet {
	Static(SrgbTexture2d),
	Directional([SrgbTexture2d; 4])
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
	
	pub fn get_force_direction(&self) -> Vec3<f64> {
		Vec3(0.0, 0.0, -1.0)
	}
	
	pub fn get_force_magnitude(&self) -> f64 {
		9.8
	}
	
	pub fn physics_step(&mut self, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) {
		let force_direction = self.get_force_direction();
		
		let mut surfaces = vec![];
		
		let l = self.position - self.size.scale(Vec3(0.5, 0.5, 0.0));
		let h = self.position + self.size.scale(Vec3(0.5, 0.5, 1.0));
		
		for x in ((l.x() - SURFACE_MARGIN).floor() as isize)..((h.x() + SURFACE_MARGIN).floor() as isize) {
			for y in ((l.y() - SURFACE_MARGIN).floor() as isize)..((h.y() + SURFACE_MARGIN).floor() as isize) {
				for z in ((l.z() - SURFACE_MARGIN).floor() as isize)..((h.z() + SURFACE_MARGIN).floor() as isize) {
					let tile_pos = Vec3(x, y, z);
					let cell_pos = tile_pos >> CELL_SIZE_BITS;
					if let Some(cell) = cells.get(&cell_pos) {
						match cell.tiles[(z & CELL_Z_MASK) as usize][(y & CELL_XY_MASK) as usize][(x & CELL_XY_MASK) as usize] {
							Tile::Air | Tile::Water | Tile::HTrack | Tile::VTrack => (),
							Tile::Block(_material) => {
								let h_inset = h - tile_pos.as_type::<f64>();
								let l_inset = (tile_pos + Vec3(1, 1, 1)).as_type::<f64>() - l;
								
								if h_inset.x() > SURFACE_MARGIN && l_inset.x() > SURFACE_MARGIN
								&& h_inset.y() > SURFACE_MARGIN && l_inset.y() > SURFACE_MARGIN {
									if h_inset.z().abs() < SURFACE_MARGIN {
										self.position.2 = z as f64 - self.size.z();
										surfaces.push(Vec3(0.0, 0.0, -1.0));
									}
									if l_inset.z().abs() < SURFACE_MARGIN {
										self.position.2 = (z + 1) as f64;
										surfaces.push(Vec3(0.0, 0.0, 1.0));
									}
								}
								
								if h_inset.x() > SURFACE_MARGIN && l_inset.x() > SURFACE_MARGIN
								&& h_inset.z() > SURFACE_MARGIN && l_inset.z() > SURFACE_MARGIN {
									if h_inset.y().abs() < SURFACE_MARGIN {
										self.position.1 = y as f64 - self.size.y() * 0.5;
										surfaces.push(Vec3(0.0, -1.0, 0.0));
									}
									if l_inset.y().abs() < SURFACE_MARGIN {
										self.position.1 = (y + 1) as f64 + self.size.y() * 0.5;
										surfaces.push(Vec3(0.0, 1.0, 0.0));
									}
								}
								
								if h_inset.y() > SURFACE_MARGIN && l_inset.y() > SURFACE_MARGIN
								&& h_inset.z() > SURFACE_MARGIN && l_inset.z() > SURFACE_MARGIN {
									if h_inset.x().abs() < SURFACE_MARGIN {
										self.position.0 = x as f64 - self.size.x() * 0.5;
										surfaces.push(Vec3(-1.0, 0.0, 0.0));
									}
									if l_inset.x().abs() < SURFACE_MARGIN {
										self.position.0 = (x + 1) as f64 + self.size.x() * 0.5;
										surfaces.push(Vec3(1.0, 0.0, 0.0));
									}
								}
							}
							Tile::Ramp(_material, direction, level) => {
								
							}
						}
					}
				}
			}
		}
		
		
		if self.jump_input && surfaces.len() > 0 {
			self.velocity += self.get_force_direction() * 2.0;
		}
		
		self.velocity += self.movement_input * dt * 5.0;
		
		self.position += self.velocity * dt;
		
		// match self.status {
		// 	EntityStatus::Grounded(_normal) => {
		// 		if !self.velocity.is_zero() {
		// 			self.velocity = self.velocity.normalize() * f64::max(self.velocity.length() - self.ground_acceleration * dt, 0.0);
		// 			self.velocity *= 1.0 - self.velocity.length() * self.air_resistance * dt;
		// 		}
		// 	}
		// 	EntityStatus::Falling => {
		// 		self.velocity += force * dt;
		// 		self.velocity *= 1.0 - self.velocity.length() * self.air_resistance * dt;
		// 	}
		// 	EntityStatus::Swimming => {
		// 		self.velocity *= (0.5f64).powf(dt);
		// 	}
		// }
		
		// let mut first_collision = None;
		// let mut first_collision_t = 1.0;
		
		// for corner in [
		// 	Vec3(-0.5, -0.5, -0.0f64),
		// 	Vec3( 0.5, -0.5, -0.0),
		// 	Vec3(-0.5,  0.5, -0.0),
		// 	Vec3( 0.5,  0.5, -0.0),
		// 	Vec3(-0.5, -0.5,  1.0),
		// 	Vec3( 0.5, -0.5,  1.0),
		// 	Vec3(-0.5,  0.5,  1.0),
		// 	Vec3( 0.5,  0.5,  1.0),
		// ] {
		// 	let current = self.position + self.size.scale(corner);
		// 	let next = current + self.velocity * dt;
			
		// 	if let Some(collision) = raycast(cells, current, next, first_collision_t) {
		// 		first_collision = Some(collision);
		// 		first_collision_t = collision.0;
		// 	}
		// }
		
		
		// if let Some((t, normal)) = first_collision {
		// 	if self.velocity.dot(normal) > 0.0 { println!("collided backwards? {:?}, {:?}", self.velocity, normal); }
			
		// 	self.position += self.velocity * t * dt;
		// 	self.velocity -= normal * self.velocity.dot(normal);
		// 	self.position += self.velocity * (1.0 - t) * dt;
			
		// } else {
		// 	self.position += self.velocity * dt;
		// }
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
}
