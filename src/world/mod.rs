use crate::*;

pub mod perlin;
pub mod generator;
pub mod mesh;
// pub use perlin::*;
pub use generator::*;
pub use mesh::*;




pub const CELL_WIDTH_BITS: u16 = 5;
pub const CELL_HEIGHT_BITS: u16 = 5;
pub const CELL_WIDTH: usize = 1 << CELL_WIDTH_BITS;
pub const CELL_HEIGHT: usize = 1 << CELL_HEIGHT_BITS;
pub const CELL_XY_MASK: isize = CELL_WIDTH as isize - 1;
pub const CELL_Z_MASK: isize = CELL_HEIGHT as isize - 1;

pub const CELL_SIZE_BITS: Vec3<u16> = Vec3(CELL_WIDTH_BITS, CELL_WIDTH_BITS, CELL_HEIGHT_BITS);
pub const CELL_SIZE: Vec3<isize> = Vec3(CELL_WIDTH as isize, CELL_WIDTH as isize, CELL_HEIGHT as isize);
pub const CELL_MASK: Vec3<isize> = Vec3(CELL_XY_MASK, CELL_XY_MASK, CELL_Z_MASK);

pub type CellTiles = [[[Tile; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT];


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
	pub vertices: Vec<ModelVertex>,
	pub indices: Vec<ModelIndex>,
	pub mesh_buffers: Option<(VertexBuffer<ModelVertex>, IndexBuffer<ModelIndex>)>,
	pub update_mesh_buffers: bool,
	pub unload: bool,
}



pub struct World {
	pub cells: HashMap<Vec3<isize>, Cell>,
	pub entities: Vec<Entity>,
	generator_settings: GeneratorSettings,
}

impl World {
	pub fn new() -> Self {
		Self {
			cells: HashMap::new(),
			entities: vec![],
			generator_settings: GeneratorSettings {
				seed: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() & (u64::MAX as u128)) as u64,
				large_size: 64.0,
				small_size: 4.0,
				octave_size: 2.0,
				octave_weight: 2.0,
				height_scale: 16.0,
				center: 16.0,
			}
		}
	}
	
	pub fn get_or_load_cell(&mut self, location: Vec3<isize>) -> &Cell {
		if !self.cells.contains_key(&location) { self.load(location); }
		self.cells.get(&location).unwrap()
	}
	
	pub fn get_or_load_cell_mut(&mut self, location: Vec3<isize>) -> &mut Cell {
		if !self.cells.contains_key(&location) { self.load(location); }
		self.cells.get_mut(&location).unwrap()
	}
	
	pub fn get_block(&mut self, position: Vec3<isize>) -> Tile {
		self.get_or_load_cell(position >> CELL_SIZE_BITS).tiles[(position & CELL_MASK).as_type()]
	}
	
	pub fn load(&mut self, location: Vec3<isize>) {
		if self.cells.contains_key(&location) { return }
		
		let mut tiles = {
			let ptr = Box::into_raw(vec![[[Tile::default(); CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT].into_boxed_slice()) as *mut CellTiles;
			unsafe { Box::from_raw(ptr) }
		};
		
		generate_cell(&mut tiles, location, &self.generator_settings);
		
		let mut cell = Cell {
			tiles,
			vertices: vec![],
			indices: vec![],
			mesh_buffers: None,
			update_mesh_buffers: false,
			unload: false,
		};
		
		build_cell_mesh(&mut cell, location, &mut self.cells);
		
		self.cells.insert(location, cell);
	}
	
	pub fn unload_flagged(&mut self) {
		self.cells.retain(|_pos, cell| !cell.unload);
	}
	
	pub fn place_player(&mut self, position: Vec3<f64>) -> Vec3<f64> {
		let tile_pos = position.floor_to::<isize>();
		let cell_location = tile_pos >> CELL_SIZE_BITS;
		let pos_in_cell = (tile_pos & CELL_MASK).as_type::<usize>();
		let cell = self.get_or_load_cell(cell_location.with_z(0));
		
		for z in (0..CELL_HEIGHT).rev() {
			if cell.tiles[pos_in_cell.with_z(z)] != Tile::empty(Air) {
				return position.with_z(z as f64 + 1.0)
			}
		}
		
		position.with_z(0.0)
	}
	
	pub fn update_mesh_buffers(&mut self, display: &Display) {
		for (_location, cell) in &mut self.cells {
			if cell.update_mesh_buffers {
				cell.mesh_buffers = Some((
					VertexBuffer::new(display, &cell.vertices).unwrap(),
					IndexBuffer::new(display, PrimitiveType::TrianglesList, &cell.indices).unwrap(),
				));
				cell.update_mesh_buffers = false;
			}
		}
	}
	
	
}


