use num_traits::Zero;
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

type CellTiles = [[[Tile; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT];

impl std::ops::Index<Vec3<usize>> for CellTiles {
	type Output = Tile;
	fn index(&self, index: Vec3<usize>) -> &Self::Output {
		&self[index.2][index.1][index.0]
	}
}

impl std::ops::IndexMut<Vec3<usize>> for CellTiles {
	fn index_mut(&mut self, index: Vec3<usize>) -> &mut Self::Output {
		&mut self[index.2][index.1][index.0]
	}
}

pub struct Cell {
	pub tiles: Box<CellTiles>,
}

impl Cell {
	pub fn load(_location: Vec3<isize>) -> Self {
		let mut tiles = {
			let ptr = Box::into_raw(vec![[[Air; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT].into_boxed_slice()) as *mut CellTiles;
			unsafe { Box::from_raw(ptr) }
		};
		
		for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::zero(), Vec3(CELL_WIDTH, CELL_WIDTH, 1)) {
			tiles[pos] = Block(Stone);
			tiles[pos + Vec3::<usize>::Z] = if (pos.x() + pos.y()) % 2 == 0 { Block(Mud) } else { Air };
			tiles[pos + Vec3::<usize>::Z * 2] = if (pos.x() + pos.y()) % 4 == 0 { Block(Dirt) } else { Air };
			tiles[pos + Vec3::<usize>::Z * 3] = if (pos.x() + pos.y()) % 8 == 0 { Block(Grass) } else { Air };
		}
		
		tiles[Vec3(0, 0, 9)] = Block(Stone);
		
		Self {
			tiles,
		}
	}
}


