use crate::Vec3;

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
	Ramp(Material, u16, u8),
	HTrack,
	VTrack,
}

pub use Material::*;
pub use Tile::*;

impl Tile {
	pub fn get_uv(&self) -> (u16, u16) {
		match self {
			Air    => (0, 0),
			Water  => (5, 0),
			Block(material) => match material {
				Grass  => (1, 0),
				Mud    => (2, 0),
				Dirt   => (3, 0),
				Stone  => (4, 0),
				Wood   => (0, 1),
				Brick  => (1, 1),
				Tiles   => (2, 1),
			}
			Ramp(_material, _direction, _level) => {
				(10, 10)
			}
			HTrack => (0, 2),
			VTrack => (1, 2),
		}
	}
}


pub const CELL_WIDTH_BITS: u16 = 8;
pub const CELL_HEIGHT_BITS: u16 = 4;
pub const CELL_WIDTH: usize = 1 << CELL_WIDTH_BITS;
pub const CELL_HEIGHT: usize = 1 << CELL_HEIGHT_BITS;
pub const CELL_XY_MASK: isize = CELL_WIDTH as isize - 1;
pub const CELL_Z_MASK: isize = CELL_HEIGHT as isize - 1;

pub const CELL_SIZE_BITS: Vec3<u16> = Vec3(CELL_WIDTH_BITS, CELL_WIDTH_BITS, CELL_HEIGHT_BITS);
pub const CELL_SIZE: Vec3<isize> = Vec3(CELL_WIDTH as isize, CELL_WIDTH as isize, CELL_HEIGHT as isize);
pub const CELL_MASK: Vec3<isize> = Vec3(CELL_XY_MASK, CELL_XY_MASK, CELL_Z_MASK);

pub struct Cell {
	pub tiles: Box<[[[Tile; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT]>,
}

impl Cell {
	pub fn load(_location: Vec3<isize>) -> Self {
		let mut tiles = {
			let ptr = Box::into_raw(vec![[[Air; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT].into_boxed_slice()) as *mut [[[Tile; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT];
			unsafe { Box::from_raw(ptr) }
		};
		
		for y in 0..CELL_WIDTH {
			for x in 0..CELL_WIDTH {
				tiles[0][y][x] = Block(Stone);
				tiles[1][y][x] = if (x + y) % 2 == 0 { Block(Mud) } else { Air };
				tiles[2][y][x] = if (x + y) % 4 == 0 { Block(Dirt) } else { Air };
				tiles[3][y][x] = if (x + y) % 8 == 0 { Block(Grass) } else { Air };
				tiles[4][y][x] = Air;
			}
		}
		
		tiles[9][0][0] = Block(Stone);
		
		Self {
			tiles,
		}
	}
}
