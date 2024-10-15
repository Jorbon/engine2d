use crate::*;


pub struct GeneratorSettings {
	pub seed: u64,
	pub large_size: f64,
	pub small_size: f64,
	pub octave_size: f64,
	pub octave_weight: f64,
	pub height_scale: f64,
	pub center: f64,
}


pub fn generate_cell(tiles: &mut CellTiles, location: Vec3<isize>, gen: &GeneratorSettings) {
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
			tiles[pos.with_z(z)] = Tile::full(materials[z]);
		}
		
		let direction = (slope.with_z(-1.0) * -8.0).round_to();
		tiles[pos.with_z(height)] = Tile {
			material: materials[height],
			fluid: if height < gen.center.round() as usize {Water} else {Air},
			level: (direction.x() + direction.y() + direction.z()) / 2,
			direction,
		};
		
		for z in (height + 1)..gen.center.round() as usize {
			tiles[pos.with_z(z)] = Tile::empty(Water);
		}
	}
}


