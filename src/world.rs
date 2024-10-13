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

pub fn perlin_noise(position: Vec2<f64>, seed: u64) -> (f64, Vec2<f64>) {
	let Vec2(lx, ly) = position.floor_to::<i32>();
	let Vec2(hx, hy) = position.ceil_to::<i32>();
	let Vec2(px, py) = position.modulo(1.0);
	let Vec2(nx, ny) = Vec2(px - 1.0, py - 1.0);
	
	let px2 = px*px;
	let py2 = py*py;
	let nx2 = nx*nx;
	let ny2 = ny*ny;
	
	
	let distance_ll = (1.0 - (px2 + py2)).max(0.0); let distance_ll2 = distance_ll * distance_ll; let distance_ll4 = distance_ll2 * distance_ll2;
	let distance_hl = (1.0 - (nx2 + py2)).max(0.0); let distance_hl2 = distance_hl * distance_hl; let distance_hl4 = distance_hl2 * distance_hl2;
	let distance_lh = (1.0 - (px2 + ny2)).max(0.0); let distance_lh2 = distance_lh * distance_lh; let distance_lh4 = distance_lh2 * distance_lh2;
	let distance_hh = (1.0 - (nx2 + ny2)).max(0.0); let distance_hh2 = distance_hh * distance_hh; let distance_hh4 = distance_hh2 * distance_hh2;
	
	let gradient_ll = gradient(Vec2(lx, ly), seed);
	let gradient_hl = gradient(Vec2(hx, ly), seed);
	let gradient_lh = gradient(Vec2(lx, hy), seed);
	let gradient_hh = gradient(Vec2(hx, hy), seed);
	
	let gradient_dot_ll = gradient_ll.dot(Vec2(px, py));
	let gradient_dot_hl = gradient_hl.dot(Vec2(nx, py));
	let gradient_dot_lh = gradient_lh.dot(Vec2(px, ny));
	let gradient_dot_hh = gradient_hh.dot(Vec2(nx, ny));
	
	(
		(
			gradient_dot_ll * distance_ll4 + 
			gradient_dot_hl * distance_hl4 + 
			gradient_dot_lh * distance_lh4 + 
			gradient_dot_hh * distance_hh4
		) * 128.0 / 81.0,
		Vec2(
			if distance_ll > 0.0 {gradient_ll.x() * distance_ll4 - 8.0 * px * gradient_dot_ll * distance_ll2 * distance_ll} else {0.0} + 
			if distance_hl > 0.0 {gradient_hl.x() * distance_hl4 - 8.0 * nx * gradient_dot_hl * distance_hl2 * distance_hl} else {0.0} + 
			if distance_lh > 0.0 {gradient_lh.x() * distance_lh4 - 8.0 * px * gradient_dot_lh * distance_lh2 * distance_lh} else {0.0} + 
			if distance_hh > 0.0 {gradient_hh.x() * distance_hh4 - 8.0 * nx * gradient_dot_hh * distance_hh2 * distance_hh} else {0.0},
			if distance_ll > 0.0 {gradient_ll.y() * distance_ll4 - 8.0 * py * gradient_dot_ll * distance_ll2 * distance_ll} else {0.0} + 
			if distance_hl > 0.0 {gradient_hl.y() * distance_hl4 - 8.0 * py * gradient_dot_hl * distance_hl2 * distance_hl} else {0.0} + 
			if distance_lh > 0.0 {gradient_lh.y() * distance_lh4 - 8.0 * ny * gradient_dot_lh * distance_lh2 * distance_lh} else {0.0} + 
			if distance_hh > 0.0 {gradient_hh.y() * distance_hh4 - 8.0 * ny * gradient_dot_hh * distance_hh2 * distance_hh} else {0.0},
		) * 128.0 / 81.0
	)
}

// pub fn perlin_noise2(position: Vec2<f64>, seed: u64) -> f64 {
// 	let Vec2(lx, ly) = position.floor_to::<i32>();
// 	let Vec2(hx, hy) = position.ceil_to::<i32>();
// 	let Vec2(px, py) = position.modulo(1.0);
// 	let tx = ((6.0 * px - 15.0) * px + 10.0) * px * (px * px);
// 	let ty = ((6.0 * py - 15.0) * py + 10.0) * py * (py * py);
// 	(
// 		gradient(Vec2(lx, ly), seed).dot(Vec2(px, py)) * (1.0 - tx) + 
// 		gradient(Vec2(hx, ly), seed).dot(Vec2(px - 1.0, py)) * tx
// 	) * (1.0 - ty) + 
// 	(
// 		gradient(Vec2(lx, hy), seed).dot(Vec2(px, py - 1.0)) * (1.0 - tx) + 
// 		gradient(Vec2(hx, hy), seed).dot(Vec2(px - 1.0, py - 1.0)) * tx
// 	) * ty
// }



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
			let mut height = 0.0;
			let mut slope = Vec2::<f64>::ZERO;
			let mut inverse_size = 1.0 / gen.large_size;
			let mut weight = 1.0;
			while inverse_size <= 1.0 / gen.small_size {
				let (value, gradient) = perlin_noise(tile_pos.xy().as_type::<f64>() * inverse_size, gen.seed);
				height += value * weight;
				slope += gradient * weight * inverse_size;
				
				inverse_size *= gen.octave_size;
				weight /= gen.octave_weight;
			}
			
			let height = (height * gen.height_scale + gen.center).floor() as usize;
			let slope = slope * gen.height_scale;
			
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
			
			for z in 0..height {
				tiles[pos.with_z(z)] = Block(materials[z]);
			}
			let direction = (slope.with_z(-1.0) * -8.0).round_to();
			tiles[pos.with_z(height)] = Ramp(materials[height], encode_ramp_direction(direction), (direction.x() + direction.y() + direction.z()) / 2);
			// for z in height..gen.center.round() as usize {
			// 	tiles[pos.with_z(z)] = Water;
			// }
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


const FACE_MODELS_POSITIVE: Vec3<[Vec3<f32>; 4]> = Vec3(
	[
		Vec3(1.0, 1.0, 1.0),
		Vec3(1.0, 1.0, 0.0),
		Vec3(1.0, 0.0, 0.0),
		Vec3(1.0, 0.0, 1.0),
	], [
		Vec3(0.0, 1.0, 1.0),
		Vec3(0.0, 1.0, 0.0),
		Vec3(1.0, 1.0, 0.0),
		Vec3(1.0, 1.0, 1.0),
	], [
		Vec3(0.0, 0.0, 1.0),
		Vec3(0.0, 1.0, 1.0),
		Vec3(1.0, 1.0, 1.0),
		Vec3(1.0, 0.0, 1.0),
	],
);
const FACE_MODELS_NEGATIVE: Vec3<[Vec3<f32>; 4]> = Vec3(
	[
		Vec3(0.0, 0.0, 1.0),
		Vec3(0.0, 0.0, 0.0),
		Vec3(0.0, 1.0, 0.0),
		Vec3(0.0, 1.0, 1.0),
	], [
		Vec3(1.0, 0.0, 1.0),
		Vec3(1.0, 0.0, 0.0),
		Vec3(0.0, 0.0, 0.0),
		Vec3(0.0, 0.0, 1.0),
	], [
		Vec3(1.0, 0.0, 0.0),
		Vec3(1.0, 1.0, 0.0),
		Vec3(0.0, 1.0, 0.0),
		Vec3(0.0, 0.0, 0.0),
	],
);

const UV_MARGIN: f32 = 0.0001;

const FACE_UVS: [Vec2<f32>; 4] = [
	Vec2(UV_MARGIN, UV_MARGIN),
	Vec2(UV_MARGIN, 1.0 - UV_MARGIN),
	Vec2(1.0 - UV_MARGIN, 1.0 - UV_MARGIN),
	Vec2(1.0 - UV_MARGIN, UV_MARGIN),
];



fn add_face(vertices: &mut Vec<ModelVertex>, indices: &mut Vec<ModelIndex>, pos: Vec3<usize>, material: Material, d: Direction) {
	let index_base = vertices.len() as ModelIndex;
	indices.append(&mut [0, 1, 2, 0, 2, 3].iter().map(|i| i + index_base).collect());
	
	let pos = pos.as_type::<f32>();
	let uv_corner = material.get_uv().as_type::<f32>();
	
	vertices.append(&mut (0..4).map(|i| ModelVertex {
		position: pos + if d.is_positive() {FACE_MODELS_POSITIVE} else {FACE_MODELS_NEGATIVE}[d.axis()][i],
		normal: Vec3::<f32>::unit(d),
		uv: uv_corner + FACE_UVS[i],
	}).collect());
}

fn adjacent_tile_has_full_face(d: Direction, adjacent_tile: Tile) -> bool {
	match adjacent_tile {
		Block(_) => true,
		Air | Water => false,
		Ramp(_material, direction, level) => {
			let direction = decode_ramp_direction(direction);
			let a = d.axis();
			match d.is_positive() {
				true => direction[a] > 0 && level >= direction[a.l()].max(0) + direction[a.r()].max(0),
				false => direction[a] < 0 && level >= direction[a] + direction[a.l()] + direction[a.r()],
			}
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
		
		let mut cell = Cell::generate(location, &self.generator_settings);
		
		for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type()) {
			match cell.tiles[pos] {
				Air | Water => (),
				Block(material) => {
					for d in [PX, PY, PZ, NX, NY, NZ] {
						if !adjacent_tile_has_full_face(d,
							if match d.is_positive() {
								true => pos[d.axis()] < CELL_SIZE[d.axis()] as usize - 1,
								false => pos[d.axis()] > 0,
							} {
								cell.tiles[(pos.as_type::<isize>() + Vec3::<isize>::unit(d)).as_type::<usize>()]
							} else if let Some(other_cell) = self.cells.get(&(location + Vec3::<isize>::unit(d))) {
								other_cell.tiles[pos.with(d.axis(), if d.is_positive() {0} else {CELL_SIZE[d.axis()] as usize - 1})]
							} else if d.axis() == Z {
								Air
							} else {
								continue
							}
						) {
							add_face(&mut cell.vertices, &mut cell.indices, pos, material, d);
						}
					}
				}
				Ramp(material, direction, level) => {
					let direction = decode_ramp_direction(direction);
					
					let (mut min_level, mut max_level) = (0, 0);
					direction.map(|v| if v > 0 { max_level += v } else { min_level += v });
					
					if level <= min_level { println!("Empty ramp"); continue }
					if level >= max_level { println!("Full ramp"); continue }
					
					let direction_abs = direction.abs();
					let level_abs = level + (-direction.x()).max(0) + (-direction.y()).max(0) + (-direction.z()).max(0);
					let mut v = vec![];
					for a in [X, Y, Z] {
						if level_abs <= direction_abs[a] {
							v.push(Vec3::ZERO.with(a, level_abs as f32 / direction_abs[a] as f32));
						} else {
							if level_abs >= direction_abs[a] + direction_abs[a.l()] {
								v.push(Vec3::XYZ.with(a.r(), (level_abs - (direction_abs[a] + direction_abs[a.l()])) as f32 / direction_abs[a.r()] as f32));
							} else {
								v.push(Vec3::positive_unit(a).with(a.l(), (level_abs - direction_abs[a]) as f32 / direction_abs[a.l()] as f32));
							}
							
							if level_abs < direction_abs[a] + direction_abs[a.r()] {
								v.push(Vec3::positive_unit(a).with(a.r(), (level_abs - direction_abs[a]) as f32 / direction_abs[a.r()] as f32));
							}
						}
					}
					
					let pos = pos.as_type::<f32>();
					let normal = direction.as_type::<f32>().normalize();
					let uv_corner = material.get_uv().as_type::<f32>();
					
					let index_base = cell.vertices.len() as ModelIndex;
					let index_iter = match v.len() {
						3 => [0, 1, 2].iter(),
						4 => [0, 1, 2, 0, 2, 3].iter(),
						5 => [0, 1, 2, 0, 2, 3, 0, 3, 4].iter(),
						6 => [0, 2, 4, 0, 1, 2, 2, 3, 4, 4, 5, 0].iter(),
						n => panic!("{n} vertices on slope face")
					};
					
					cell.indices.append(&mut match {
						let mut reverse = true;
						direction.map(|v| if v < 0 { reverse = !reverse });
						reverse
					} {
						false => index_iter.map(|i| i + index_base).collect(),
						true => index_iter.rev().map(|i| i + index_base).collect(),
					});
					
					for vertex in v {
						let vertex = Vec3::by_axis(|a| if direction[a] >= 0 {vertex[a]} else {1.0 - vertex[a]});
						cell.vertices.push(ModelVertex {
							position: pos + vertex,
							normal,
							uv: uv_corner + vertex.xy(),
						});
					}
					
					
				}
			}
		}
		
		for d in [PX, PY, PZ, NX, NY, NZ] {
			if let Some(other_cell) = self.cells.get_mut(&(location - Vec3::<isize>::unit(d))) {
				for tile_pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type::<usize>().with(d.axis(), 1)) {
					let mut this_tile_pos = tile_pos;
					let mut other_tile_pos = tile_pos.with(d.axis(), CELL_SIZE[d.axis()] as usize - 1);
					if d.is_negative() { (this_tile_pos, other_tile_pos) = (other_tile_pos, this_tile_pos); }
					
					match other_cell.tiles[other_tile_pos] {
						Air | Water | Ramp(..) => (),
						Block(material) => if !adjacent_tile_has_full_face(d, cell.tiles[this_tile_pos]) {
							add_face(&mut other_cell.vertices, &mut other_cell.indices, other_tile_pos, material, d);
						}
					}
				}
				
				other_cell.update_mesh = true;
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
		let cell = self.get_or_load_cell(cell_location.with_z(0));
		
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


