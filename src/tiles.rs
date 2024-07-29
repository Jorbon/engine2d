use crate::Vec2;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
	Air,
	Grass,
	Mud,
	Dirt,
	Stone,
	Water,
	Wood,
	Brick,
	Tile,
	HTrack,
	VTrack,
}

impl Tile {
	pub fn get_uv(&self) -> (u16, u16) {
		match self {
			Tile::Air    => (0, 0),
			Tile::Grass  => (1, 0),
			Tile::Mud    => (2, 0),
			Tile::Dirt   => (3, 0),
			Tile::Stone  => (4, 0),
			Tile::Water  => (5, 0),
			Tile::Wood   => (0, 1),
			Tile::Brick  => (1, 1),
			Tile::Tile   => (2, 1),
			Tile::HTrack => (0, 2),
			Tile::VTrack => (1, 2),
		}
	}
}


pub const CELL_WIDTH: usize = 256;
pub const CELL_HEIGHT: usize = 16;

pub struct Cell {
	pub location: Vec2<isize>,
	pub tiles: Box<[[[Tile; CELL_HEIGHT]; CELL_WIDTH]; CELL_WIDTH]>,
}

impl Cell {
	pub fn new(location: Vec2<isize>) -> Self {
		let mut tiles = Box::new([[[Tile::Air; CELL_HEIGHT]; CELL_WIDTH]; CELL_WIDTH]);
		
		for y in 0..CELL_WIDTH {
			for x in 0..CELL_WIDTH {
				tiles[x][y][0] = Tile::Grass;
				tiles[x][y][1] = if (x + y) % 2 == 0 { Tile::Grass } else { Tile::Water };
				tiles[x][y][2] = if (x + y) % 4 == 0 { Tile::Grass } else { Tile::Water };
				tiles[x][y][3] = if (x + y) % 8 == 0 { Tile::Grass } else { Tile::Water };
				tiles[x][y][4] = Tile::Water;
			}
		}
		
		Self {
			location,
			tiles,
		}
	}
}
