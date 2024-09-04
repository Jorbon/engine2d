use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RampDirection {
	North(NorthSouthAdjacent),
	South(NorthSouthAdjacent),
	East(EastWestAdjacent),
	West(EastWestAdjacent),
	Up(UpDownAdjacent),
	Down(UpDownAdjacent),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)] pub enum NorthSouthAdjacent { East, West, Up, Down }
#[derive(Copy, Clone, Debug, Eq, PartialEq)] pub enum EastWestAdjacent { North, South, Up, Down }
#[derive(Copy, Clone, Debug, Eq, PartialEq)] pub enum UpDownAdjacent { North, South, East, West }


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
pub enum Tile {
	Air,
	Water,
	Block(Material),
	Ramp(Material, u16, i8),
}

pub use Material::*;
pub use Tile::*;

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

pub fn decode_ramp_direction(d: u16) -> Vec3<i8> {
	Vec3(
		(((d & 0b0000000000011111) << 3) as i8) >> 3,
		(((d & 0b0000001111100000) >> 2) as i8) >> 3,
		(((d & 0b1111110000000000) >> 8) as i8) >> 2,
	)
}

pub fn encode_ramp_direction(d: Vec3<i8>) -> u16 {
	((d.x() & 0b00011111) as u16) | (((d.y() & 0b00011111) as u16) << 5) | (((d.z() & 0b00111111) as u16) << 10)
}










