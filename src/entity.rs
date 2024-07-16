use glium::texture::SrgbTexture2d;

use crate::vec::Vec3;




pub enum Direction { Up, Down, Left, Right }

pub struct Entity {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: Vec3,
    pub direction: Direction,
    pub speed: f32,
	pub acceleration: f32,
    pub textures: Vec<SrgbTexture2d>,
	on_ground: bool,
}

impl Entity {
	pub fn new(position: Vec3, size: Vec3, direction: Direction, speed: f32, acceleration: f32, textures: Vec<SrgbTexture2d>) -> Self {
		Self {
			position,
			velocity: Vec3(0.0, 0.0, 0.0),
			size,
			direction,
			speed,
			acceleration,
			textures,
			on_ground: false,
		}
	}
	pub fn input_move(&mut self, input: Vec3, dt: f32) {
		if input.is_zero() { return }
		
		let input_length = input.length();
		let input_direction = input.normalize();
		
		let wish_acceleration = self.acceleration * input_length * dt;
		let target_velocity = self.speed * input_length;
		let current_velocity = self.velocity.dot(input_direction);
		
		if current_velocity < -target_velocity { // reduced so that it doesn't act like 2x friction
			self.velocity += input_direction * f32::min(0.3 * wish_acceleration, target_velocity - current_velocity);
		} else if current_velocity < target_velocity { // 2x because 1x counters friction
			self.velocity += input_direction * f32::min(2.0 * wish_acceleration, target_velocity - current_velocity);
		}
		
			 if input.y() < -input.x().abs() { self.direction = Direction::Up;    }
		else if input.y() >  input.x().abs() { self.direction = Direction::Down;  }
		else if input.x() < -input.y().abs() { self.direction = Direction::Left;  }
		else if input.x() >  input.y().abs() { self.direction = Direction::Right; }
	}
	pub fn physics_step(&mut self, dt: f32) {
		if !self.velocity.is_zero() {
			self.velocity = self.velocity.normalize() * f32::max(self.velocity.length() - self.acceleration * dt, 0.0);
		}
		
		let gravity = 9.8;
		
		self.velocity.2 -= gravity * dt;
		
		self.position += self.velocity * dt;
	}
}