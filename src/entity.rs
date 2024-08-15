use std::path::Path;

use glium::{glutin::surface::WindowSurface, texture::SrgbTexture2d, Display};

use crate::{load_texture, raycast, vec::*, Cell};




pub enum FacingDirection { Up, Down, Left, Right }

pub struct Entity {
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub size: Vec3<f32>,
    pub direction: FacingDirection,
    pub ground_speed: f32,
	pub air_speed: f32,
	pub water_speed: f32,
	pub ground_acceleration: f32,
	pub air_acceleration: f32,
	pub water_acceleration: f32,
	pub air_resistance: f32,
    pub sprites: SpriteSet,
	pub status: EntityStatus,
}

#[derive(PartialEq)]
pub enum EntityStatus {
	Grounded(Vec3<f32>),
	Falling,
	Swimming,
}

pub enum SpriteSet {
	Static(SrgbTexture2d),
	Directional([SrgbTexture2d; 4])
}

impl SpriteSet {
	pub fn load(display: &Display<WindowSurface>, path: &str) -> Self {
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




impl Entity {
	pub fn new(position: Vec3<f32>, size: Vec3<f32>, sprites: SpriteSet) -> Self {
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
			status: EntityStatus::Grounded(Vec3(0.0, 0.0, 1.0)),
		}
	}
	
	pub fn get_acceleration_speed(&self) -> (f32, f32) {
		match self.status {
			EntityStatus::Grounded(_) => (self.ground_acceleration, self.ground_speed),
			EntityStatus::Falling => (self.air_acceleration, self.air_speed),
			EntityStatus::Swimming => (self.water_acceleration, self.water_speed),
		}
	}
	
	pub fn input_move(&mut self, mut input: Vec3<f32>, dt: f32) {
		if let EntityStatus::Grounded(_) = self.status {
			input.2 = 0.0;
		}
		if input.is_zero() { return }
		
		let (acceleration, speed) = self.get_acceleration_speed();
		
		let input_length = input.length();
		let input_direction = input.normalize();
		
		let wish_acceleration = acceleration * input_length * dt;
		let target_velocity = speed * input_length;
		let current_velocity = self.velocity.dot(input_direction);
		
		let (acceleration_parameter, deceleration_parameter) = match self.status {
			EntityStatus::Grounded(_) => (2.0, 0.0),
			EntityStatus::Falling => (1.0, 1.0),
			EntityStatus::Swimming => (1.5, 0.5),
		};
		
		if current_velocity < -target_velocity { // reduced so that it doesn't act like 2x friction
			self.velocity += input_direction * f32::min(deceleration_parameter * wish_acceleration, target_velocity - current_velocity);
		} else if current_velocity < target_velocity { // 2x because 1x counters friction
			self.velocity += input_direction * f32::min(acceleration_parameter * wish_acceleration, target_velocity - current_velocity);
		}
		
			 if input.y() < -input.x().abs() { self.direction = FacingDirection::Up; }
		else if input.y() >  input.x().abs() { self.direction = FacingDirection::Down; }
		else if input.x() < -input.y().abs() { self.direction = FacingDirection::Left; }
		else if input.x() >  input.y().abs() { self.direction = FacingDirection::Right; }
	}
	
	pub fn jump(&mut self) {
		if let EntityStatus::Grounded(normal) = self.status {
			self.velocity += normal * 2.0;
			self.status = EntityStatus::Falling;
		}
	}
	
	pub fn physics_step(&mut self, cells: &Vec<(Vec3<isize>, Cell)>, dt: f32) {
		let gravity = 9.8;
		let force = Vec3(0.0, 0.0, -gravity);
		
		if let EntityStatus::Grounded(normal) = self.status {
			if force.dot(normal) >= 0.0 {
				self.status = EntityStatus::Falling;
			}
		}
		
		match self.status {
			EntityStatus::Grounded(_normal) => {
				if !self.velocity.is_zero() {
					self.velocity = self.velocity.normalize() * f32::max(self.velocity.length() - self.ground_acceleration * dt, 0.0);
					self.velocity *= 1.0 - self.velocity.length() * self.air_resistance * dt;
				}
			}
			EntityStatus::Falling => {
				self.velocity += force * dt;
				self.velocity *= 1.0 - self.velocity.length() * self.air_resistance * dt;
			}
			EntityStatus::Swimming => {
				self.velocity *= (0.5f32).powf(dt);
			}
		}
		
		let mut first_collision = None;
		let mut first_collision_t = 1.0;
		
		for corner in [
			Vec3(-0.5, -0.5, -0.0f32),
			Vec3( 0.5, -0.5, -0.0),
			Vec3(-0.5,  0.5, -0.0),
			Vec3( 0.5,  0.5, -0.0),
			Vec3(-0.5, -0.5,  1.0),
			Vec3( 0.5, -0.5,  1.0),
			Vec3(-0.5,  0.5,  1.0),
			Vec3( 0.5,  0.5,  1.0),
		] {
			let current = self.position + self.size.scale(corner);
			let next = current + self.velocity * dt;
			
			if let Some(collision) = raycast(cells, current, next, first_collision_t) {
				first_collision = Some(collision);
				first_collision_t = collision.0;
			}
		}
		
		
		if let Some((t, normal)) = first_collision {
			if self.velocity.dot(normal) > 0.0 { println!("collided backwards? {:?}, {:?}", self.velocity, normal); }
			
			self.position += self.velocity * t * dt;
			self.velocity -= normal * self.velocity.dot(normal);
			self.status = EntityStatus::Grounded(normal);
			self.position += self.velocity * (1.0 - t) * dt;
			
		} else {
			self.position += self.velocity * dt;
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
}
