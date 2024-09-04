use num_traits::ConstZero;

use crate::*;

fn int_hash(mut x: u64, seed: u64) -> u64 {
	x ^= seed;
	x = (x ^ (x >> 30)).overflowing_mul(0xbf58476d1ce4e5b9).0;
	x = (x ^ (x >> 27)).overflowing_mul(0x94d049bb133111eb).0;
	x = x ^ (x >> 31);
	return x;
}

fn gradient(v: Vec2<i32>, seed: u64) -> Vec2<f64> {
	let h = int_hash((v.x() as u64) | ((v.y() as u64) << 32), seed);
	Vec2((h >> 32) as f64 / 0x80000000u32 as f64 - 1.0, (h & 0xffffffff) as f64 / 0x80000000u32 as f64 - 1.0)
}

pub fn perlin_noise(position: Vec2<f64>, seed: u64) -> f64 {
	let Vec2(lx, ly) = position.floor_to::<i32>();
	let Vec2(hx, hy) = position.ceil_to::<i32>();
	let Vec2(px, py) = position.modulo(1.0);
	let Vec2(nx, ny) = Vec2(px - 1.0, py - 1.0);
	
	let px2 = px*px;
	let py2 = py*py;
	let nx2 = nx*nx;
	let ny2 = ny*ny;
	
	let mut lld = (1.0 - (px2 + py2)).max(0.0); lld *= lld; lld *= lld;
	let mut hld = (1.0 - (nx2 + py2)).max(0.0); hld *= hld; hld *= hld;
	let mut lhd = (1.0 - (px2 + ny2)).max(0.0); lhd *= lhd; lhd *= lhd;
	let mut hhd = (1.0 - (nx2 + ny2)).max(0.0); hhd *= hhd; hhd *= hhd;
	
	(
		gradient(Vec2(lx, ly), seed).dot(Vec2(px, py)) * lld + 
		gradient(Vec2(hx, ly), seed).dot(Vec2(nx, py)) * hld + 
		gradient(Vec2(lx, hy), seed).dot(Vec2(px, ny)) * lhd + 
		gradient(Vec2(hx, hy), seed).dot(Vec2(nx, ny)) * hhd
	) * 128.0 / 81.0
	
	// gradient(Vec2(lx, ly), seed).dot(Vec2(px, py)) * lld
}

pub fn perlin_noise2(position: Vec2<f64>, seed: u64) -> f64 {
	let Vec2(lx, ly) = position.floor_to::<i32>();
	let Vec2(hx, hy) = position.ceil_to::<i32>();
	let Vec2(px, py) = position.modulo(1.0);
	let tx = ((6.0 * px - 15.0) * px + 10.0) * px * (px * px);
	let ty = ((6.0 * py - 15.0) * py + 10.0) * py * (py * py);
	(
		gradient(Vec2(lx, ly), seed).dot(Vec2(px, py)) * (1.0 - tx) + 
		gradient(Vec2(hx, ly), seed).dot(Vec2(px - 1.0, py)) * tx
	) * (1.0 - ty) + 
	(
		gradient(Vec2(lx, hy), seed).dot(Vec2(px, py - 1.0)) * (1.0 - tx) + 
		gradient(Vec2(hx, hy), seed).dot(Vec2(px - 1.0, py - 1.0)) * tx
	) * ty
}



struct GeneratorSettings {
	pub seed: u64,
	pub large_size: f64,
	pub small_size: f64,
	pub octave_size: f64,
	pub octave_weight: f64,
	pub height_scale: f64,
	pub center: f64,
	
}



pub const CELL_WIDTH_BITS: u16 = 6;
pub const CELL_HEIGHT_BITS: u16 = 6;
pub const CELL_WIDTH: usize = 1 << CELL_WIDTH_BITS;
pub const CELL_HEIGHT: usize = 1 << CELL_HEIGHT_BITS;
pub const CELL_XY_MASK: isize = CELL_WIDTH as isize - 1;
pub const CELL_Z_MASK: isize = CELL_HEIGHT as isize - 1;

pub const CELL_SIZE_BITS: Vec3<u16> = Vec3(CELL_WIDTH_BITS, CELL_WIDTH_BITS, CELL_HEIGHT_BITS);
pub const CELL_SIZE: Vec3<isize> = Vec3(CELL_WIDTH as isize, CELL_WIDTH as isize, CELL_HEIGHT as isize);
pub const CELL_MASK: Vec3<isize> = Vec3(CELL_XY_MASK, CELL_XY_MASK, CELL_Z_MASK);

type CellTiles = [[[Tile; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT];

type ModelIndex = u16;


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
	pub buffers: Option<(VertexBuffer<ModelVertex>, IndexBuffer<ModelIndex>)>,
	pub update_mesh: bool,
	pub unload: bool,
}

impl Cell {
	fn generate(location: Vec3<isize>, gen: &GeneratorSettings) -> Cell {
		let mut tiles = {
			let ptr = Box::into_raw(vec![[[Air; CELL_WIDTH]; CELL_WIDTH]; CELL_HEIGHT].into_boxed_slice()) as *mut CellTiles;
			unsafe { Box::from_raw(ptr) }
		};
		
		for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, Vec3(CELL_WIDTH, CELL_WIDTH, 1)) {
			let tile_pos = (location << CELL_SIZE_BITS) + pos.as_type::<isize>();
			let mut h = 0.0;
			let mut size = 1.0 / gen.large_size;
			let mut weight = 1.0;
			while size <= 1.0 / gen.small_size {
				h += perlin_noise(tile_pos.xy().as_type::<f64>() * size, gen.seed) * weight;
				size *= gen.octave_size;
				weight /= gen.octave_weight;
			}
			
			let h = (h * gen.height_scale + gen.center).round() as usize;
			
			let materials = [
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Mud,
				Mud,
				Mud,
				Mud,
				Grass,
				Grass,
				Grass,
				Grass,
				Dirt,
				Dirt,
				Dirt,
				Dirt,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
				Stone,
			];
			
			for z in 0..h {
				tiles[pos + Vec3::<usize>::Z * z] = Block(materials[z]);
			}
			for z in h..gen.center.round() as usize {
				tiles[pos + Vec3::<usize>::Z * z] = Water;
			}
		}
		
		Cell {
			tiles,
			vertices: vec![],
			indices: vec![],
			buffers: None,
			update_mesh: false,
			unload: false,
		}
	}
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
				seed: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
				large_size: 128.0,
				small_size: 1.0,
				octave_size: 2.0,
				octave_weight: 2.0,
				height_scale: 16.0,
				center: 16.0,
			}
		}
	}
	
	pub fn get_or_load(&mut self, location: Vec3<isize>) -> &Cell {
		if !self.cells.contains_key(&location) { self.load(location); }
		self.cells.get(&location).unwrap()
	}
	
	pub fn get_or_load_mut(&mut self, location: Vec3<isize>) -> &mut Cell {
		if !self.cells.contains_key(&location) { self.load(location); }
		self.cells.get_mut(&location).unwrap()
	}
	
	pub fn load(&mut self, location: Vec3<isize>) {
		if self.cells.contains_key(&location) { return }
		
		let mut cell = Cell::generate(location, &self.generator_settings);
		
		for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type()) {
			match cell.tiles[pos] {
				Air | Water => (),
				Block(material) => {
					for (a, c) in [
						(X, (Vec3::<f32>::Z, Vec3::<f32>::ZERO, Vec3::<f32>::Y, Vec3::<f32>::Y + Vec3::<f32>::Z)),
						(Y, (Vec3::<f32>::X + Vec3::<f32>::Z, Vec3::X, Vec3::ZERO, Vec3::Z)),
						(Z, (Vec3::X, Vec3::<f32>::X + Vec3::<f32>::Y, Vec3::Y, Vec3::ZERO)),
					] {
						match
							if pos[a] > 0 {
								cell.tiles[pos - Vec3::<usize>::unit(a)]
							} else if let Some(other_cell) = self.cells.get(&(location - Vec3::<isize>::unit(a))) {
								other_cell.tiles[pos.with(a, CELL_SIZE[a] as usize - 1)]
							} else if a == Z {
								Air
							} else {
								continue
							}
						{
							Block(_) => (),
							Air | Water | Ramp(..) => {
								let index_base = cell.vertices.len() as ModelIndex;
								cell.indices.append(&mut [0, 1, 2, 0, 2, 3].iter().map(|i| i + index_base).collect());
								
								let corner = pos.as_type::<f32>();
								let uv_corner = material.get_uv().as_type::<f32>();
								cell.vertices.append(&mut vec![
									ModelVertex { position: corner + c.0, uv: uv_corner },
									ModelVertex { position: corner + c.1, uv: uv_corner.add_y(1.0) },
									ModelVertex { position: corner + c.2, uv: uv_corner.add_x(1.0).add_y(1.0) },
									ModelVertex { position: corner + c.3, uv: uv_corner.add_x(1.0) },
								]);
							}
						}
						
						match
							if pos[a] < CELL_SIZE[a] as usize - 1 {
								cell.tiles[pos + Vec3::<usize>::unit(a)]
							} else if let Some(other_cell) = self.cells.get(&(location + Vec3::<isize>::unit(a))) {
								other_cell.tiles[pos.with(a, 0)]
							} else if a == Z {
								Air
							} else {
								continue
							}
						{
							Block(_) => (),
							Air | Water | Ramp(..) => {
								let index_base = cell.vertices.len() as ModelIndex;
								cell.indices.append(&mut [0, 1, 2, 0, 2, 3].iter().map(|i| i + index_base).collect());
								
								let corner = pos.as_type::<f32>() + Vec3::<f32>::unit(a);
								let uv_corner = material.get_uv().as_type::<f32>();
								cell.vertices.append(&mut vec![
									ModelVertex { position: corner + c.3, uv: uv_corner },
									ModelVertex { position: corner + c.2, uv: uv_corner.add_y(1.0) },
									ModelVertex { position: corner + c.1, uv: uv_corner.add_x(1.0).add_y(1.0) },
									ModelVertex { position: corner + c.0, uv: uv_corner.add_x(1.0) },
								]);
							}
						}
					}
				}
				Ramp(_material, _direction, _level) => {
					
				}
			}
		}
		
		cell.update_mesh = true;
		self.cells.insert(location, cell);
	}
	
	pub fn unload_flagged(&mut self) {
		self.cells.retain(|_pos, cell| !cell.unload);
	}
	
	pub fn place_player(&mut self, position: Vec3<f64>) -> Vec3<f64> {
		let tile_pos = position.floor_to::<isize>();
		let cell_location = tile_pos >> CELL_SIZE_BITS;
		let pos_in_cell = (tile_pos & CELL_MASK).as_type::<usize>();
		let cell = self.get_or_load(cell_location.with_z(0));
		
		for z in (0..CELL_HEIGHT).rev() {
			match cell.tiles[pos_in_cell.with_z(z)] {
				Air => continue,
				_ => return position.with_z(z as f64 + 1.0)
			}
		}
		
		position.with_z(0.0)
	}
	
	pub fn update_buffers(&mut self, display: &Display) {
		for (_location, cell) in &mut self.cells {
			if cell.update_mesh {
				cell.buffers = Some((
					VertexBuffer::new(display, &cell.vertices).unwrap(),
					IndexBuffer::new(display, PrimitiveType::TrianglesList, &cell.indices).unwrap(),
				));
				cell.update_mesh = false;
			}
		}
	}
	
	
}


