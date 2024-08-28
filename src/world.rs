use num_traits::Zero;

use crate::*;


pub const CELL_WIDTH_BITS: u16 = 3;
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
	pub unload: bool,
}

impl Cell {
	
}



pub struct World {
	pub cells: HashMap<Vec3<isize>, Cell>,
	pub entities: Vec<Entity>,
}

impl World {
	pub fn new() -> Self {
		Self {
			cells: HashMap::new(),
			entities: vec![],
		}
	}
	
	pub fn load(&mut self, location: Vec3<isize>) -> &Cell {
		if !self.cells.contains_key(&location) {
			self.cells.insert(location, self.generate(location));
		}
		self.cells.get(&location).unwrap()
	}
	
	pub fn load_mut(&mut self, location: Vec3<isize>) -> &mut Cell {
		if !self.cells.contains_key(&location) {
			self.cells.insert(location, self.generate(location));
		}
		self.cells.get_mut(&location).unwrap()
	}
	
	fn generate(&self, _location: Vec3<isize>) -> Cell {
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
		tiles[Vec3(1, 0, 8)] = Ramp(Brick, encode_ramp_direction(Vec3( 1,  1, 2)), 2);
		tiles[Vec3(2, 0, 8)] = Ramp(Brick, encode_ramp_direction(Vec3(-1,  1, 2)), 1);
		tiles[Vec3(1, 1, 8)] = Ramp(Brick, encode_ramp_direction(Vec3( 1, -1, 2)), 1);
		tiles[Vec3(2, 1, 8)] = Ramp(Brick, encode_ramp_direction(Vec3(-1, -1, 2)), 0);
		
		Cell {
			tiles,
			unload: false,
		}
	}
	
	pub fn unload_flagged(&mut self) {
		self.cells.retain(|_pos, cell| !cell.unload);
	}
	
	pub fn place_player(&mut self, position: Vec3<f64>) -> Vec3<f64> {
		let tile_pos = position.floor_to::<isize>();
		let cell_location = tile_pos >> CELL_SIZE_BITS;
		let pos_in_cell = (tile_pos & CELL_MASK).as_type::<usize>();
		let cell = self.load(cell_location.with_z(0));
		
		for z in (0..CELL_HEIGHT).rev() {
			match cell.tiles[pos_in_cell.with_z(z)] {
				Air => continue,
				_ => return position.with_z(z as f64 + 1.0)
			}
		}
		
		position.with_z(0.0)
	}
}


