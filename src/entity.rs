use std::path::Path;
use glium::{texture::SrgbTexture2d, Display};

use crate::*;



pub enum FacingDirection { Up, Down, Left, Right }

pub struct Entity {
    pub position: Vec3<f64>,
    pub velocity: Vec3<f64>,
    pub size: Vec3<f64>,
	pub mass: f64,
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
	pub show: bool,
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


impl Entity {
	pub fn new(position: Vec3<f64>, size: Vec3<f64>, sprites: SpriteSet) -> Self {
		Self {
			position,
			velocity: Vec3(0.0, 0.0, 0.0),
			size,
			mass: 70.0,
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
			show: true,
			mesh_buffers: None,
		}
	}
	
	pub fn update_sprite_status(&mut self) {
		     if self.movement_input.y() < -self.movement_input.x().abs() { self.direction = FacingDirection::Up; }
		else if self.movement_input.y() >  self.movement_input.x().abs() { self.direction = FacingDirection::Down; }
		else if self.movement_input.x() < -self.movement_input.y().abs() { self.direction = FacingDirection::Left; }
		else if self.movement_input.x() >  self.movement_input.y().abs() { self.direction = FacingDirection::Right; }
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
	
	pub fn load_mesh_buffers(&mut self, display: &Display) {
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
	
}
