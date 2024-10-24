use crate::*;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Material {
	Grass,
	Mud,
	Dirt,
	Stone,
	Wood,
	Brick,
	Tiles,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fluid {
	Air,
	Water,
}

use num_traits::Zero;
pub use Material::*;
pub use Fluid::*;

impl Material {
	pub fn get_uv(&self) -> Vec2<u16> {
		match self {
			Grass => Vec2(1, 0),
			Mud   => Vec2(2, 0),
			Dirt  => Vec2(3, 0),
			Stone => Vec2(4, 0),
			Wood  => Vec2(0, 1),
			Brick => Vec2(1, 1),
			Tiles => Vec2(2, 1),
		}
	}
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile {
	pub material: Material,
	pub fluid: Fluid,
	pub level: i8,
	pub direction: Vec3<i8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileState { Empty, Full, Partial }

impl Tile {
	pub fn full(material: Material) -> Self {
		Self {
			material,
			fluid: Air,
			level: 1,
			direction: Vec3::ZERO,
		}
	}
	pub fn empty(fluid: Fluid) -> Self {
		Self {
			material: Grass,
			fluid,
			level: 0,
			direction: Vec3::ZERO,
		}
	}
	pub fn state(&self) -> TileState {
		if self.direction.is_zero() {
			if self.level == 0 {TileState::Empty} else {TileState::Full}
		} else {TileState::Partial}
	}
	pub fn is_empty(&self) -> bool {
		self.direction.is_zero() && self.level == 0
	}
	pub fn is_full(&self) -> bool {
		self.direction.is_zero() && self.level != 0
	}
	
	 // Invalid for empty tiles
	pub fn includes_corner(&self, corner: Vec3<i8>) -> bool {
		self.direction.dot(corner) <= self.level
	}
}


impl Default for Tile {
	fn default() -> Self {
		Self {
			material: Grass,
			fluid: Air,
			level: 0,
			direction: Vec3::ZERO,
		}
	}
}



