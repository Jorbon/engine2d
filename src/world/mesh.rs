use crate::*;


const TILE_TEXTURE_ATLAS_SIZE: u32 = 16;

const FACE_UVS: [Vec2<i8>; 4] = [
	Vec2(0, 0),
	Vec2(0, 1),
	Vec2(1, 1),
	Vec2(1, 0),
];

const UV_MARGIN: f32 = 0.0001;

fn uv_unbleed(uv: Vec2<f32>) -> Vec2<f32> {
	(uv - Vec2(0.5, 0.5)) * (1.0 - UV_MARGIN) + Vec2(0.5, 0.5)
}

fn uv_to_face_mesh<T: Copy + num_traits::Zero + num_traits::One + std::ops::Sub<T, Output = T>>(uv: Vec2<T>, d: Direction) -> Vec3<T> {
	match d {
		PX => Vec3(T::one(), T::one() - uv.x(), T::one() - uv.y()),
		NX => Vec3(T::zero(), uv.x(), T::one() - uv.y()),
		PY => Vec3(uv.x(), T::one(), T::one() - uv.y()),
		NY => Vec3(T::one() - uv.x(), T::zero(), T::one() - uv.y()),
		PZ => Vec3(uv.x(), uv.y(), T::one()),
		NZ => Vec3(uv.x(), T::one() - uv.y(), T::zero()),
	}
}

fn face_mesh_to_uv<T: Copy + num_traits::One + std::ops::Sub<T, Output = T>>(pos: Vec3<T>, d: Direction) -> Vec2<T> {
	match d {
		PX => Vec2(T::one() - pos.y(), T::one() - pos.z()),
		NX => Vec2(pos.y(), T::one() - pos.z()),
		PY => Vec2(pos.x(), T::one() - pos.z()),
		NY => Vec2(T::one() - pos.x(), T::one() - pos.z()),
		PZ => Vec2(pos.x(), pos.y()),
		NZ => Vec2(pos.x(), T::one() - pos.y()),
	}
}


// The given tile should have the specified face drawn if it has that face
fn add_face(vertices: &mut Vec<ModelVertex>, indices: &mut Vec<ModelIndex>, pos: Vec3<usize>, d: Direction, tile: Tile) {
	if tile.is_empty() { return }
	
	let pos = pos.as_type::<f32>();
	let uv_corner = tile.material.get_uv().as_type::<f32>();
	
	let mut v = vec![];
	
	let s = FACE_UVS.map(|uv| tile.direction.dot(uv_to_face_mesh(uv, d)));
	
	for (i, j) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
		if s[i] <= tile.level {
			v.push(FACE_UVS[i].as_type::<f32>());
		}
		if (s[i] <= tile.level) != (s[j] <= tile.level) {
			v.push(lerp(FACE_UVS[i].as_type::<f32>(), FACE_UVS[j].as_type::<f32>(), prel(s[i] as f32, s[j] as f32, tile.level as f32)));
		}
	}
	
	
	let index_base = vertices.len() as ModelIndex;
	let index_iter = match v.len() {
		0 | 1 | 2 => return,
		3 => [0, 1, 2].iter(),
		4 => [0, 1, 2, 0, 2, 3].iter(),
		5 => [0, 1, 2, 0, 2, 3, 0, 3, 4].iter(),
		n => panic!("{n} vertices on aligned face")
	};
	indices.append(&mut index_iter.map(|i| i + index_base).collect());
	
	vertices.append(&mut v.iter().map(|uv| ModelVertex {
		position: pos + uv_to_face_mesh(*uv, d),
		normal: Vec3::<f32>::unit(d),
		uv: (uv_corner + uv_unbleed(*uv)) / TILE_TEXTURE_ATLAS_SIZE as f32,
	}).collect());
}


fn tile_has_full_face(d: Direction, tile: Tile) -> bool {
	match tile.state() {
		TileState::Empty => false,
		TileState::Full => true,
		TileState::Partial => {
			for uv in FACE_UVS {
				if !tile.includes_corner(uv_to_face_mesh(uv, d)) { return false }
			}
			true
		}
	}
}



pub fn build_cell_mesh(cell: &mut Cell, location: Vec3<isize>, cells: &mut HashMap<Vec3<isize>, Cell>) {
	for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type()) {
		let tile = cell.tiles[pos];
		
		if tile.is_empty() { continue }
		
		// Aligned faces
		for d in [PX, PY, PZ, NX, NY, NZ] {
			if !tile_has_full_face(-d,
				if match d.is_positive() {
					true => pos[d.axis()] < CELL_SIZE[d.axis()] as usize - 1,
					false => pos[d.axis()] > 0,
				} {
					cell.tiles[(pos.as_type::<isize>() + Vec3::<isize>::unit(d)).as_type::<usize>()]
				} else if let Some(other_cell) = cells.get(&(location + Vec3::<isize>::unit(d))) {
					other_cell.tiles[pos.with(d.axis(), if d.is_positive() {0} else {CELL_SIZE[d.axis()] as usize - 1})]
				} else { continue }
			) {
				add_face(&mut cell.vertices, &mut cell.indices, pos, d, tile);
			}
		}
		
		// Slope face
		if !tile.is_full() {
			let (mut min_level, mut max_level) = (0, 0);
			tile.direction.map(|v| if v > 0 { max_level += v } else { min_level += v });
			
			if tile.level <= min_level { println!("Empty ramp at {:?}", (location << CELL_SIZE_BITS) + pos.as_type::<isize>()); continue }
			if tile.level >= max_level { println!("Full ramp at {:?}", (location << CELL_SIZE_BITS) + pos.as_type::<isize>()); continue }
			
			let direction_abs = tile.direction.abs();
			let level_abs = tile.level + (-tile.direction.x()).max(0) + (-tile.direction.y()).max(0) + (-tile.direction.z()).max(0);
			let mut v = vec![];
			for a in [X, Y, Z] {
				if level_abs < direction_abs[a] {
					v.push(Vec3::ZERO.with(a, level_abs as f32 / direction_abs[a] as f32));
				} else {
					if level_abs < direction_abs[a] + direction_abs[a.l()] {
						v.push(Vec3::positive_unit(a).with(a.l(), (level_abs - direction_abs[a]) as f32 / direction_abs[a.l()] as f32));
					} else {
						v.push(Vec3::XYZ.with(a.r(), (level_abs - (direction_abs[a] + direction_abs[a.l()])) as f32 / direction_abs[a.r()] as f32));
					}
					
					if level_abs < direction_abs[a] + direction_abs[a.r()] {
						v.push(Vec3::positive_unit(a).with(a.r(), (level_abs - direction_abs[a]) as f32 / direction_abs[a.r()] as f32));
					}
				}
			}
			
			let pos = pos.as_type::<f32>();
			let uv_corner = tile.material.get_uv().as_type::<f32>();
			
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
				tile.direction.map(|v| if v < 0 { reverse = !reverse });
				reverse
			} {
				false => index_iter.map(|i| i + index_base).collect(),
				true => index_iter.rev().map(|i| i + index_base).collect(),
			});
			
			let uv_direction = *[PX, PY, PZ, NX, NY, NZ].iter().map(|d| (d, tile.direction.dot(Vec3::<i8>::unit(*d)))).max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
			
			for vertex in v {
				let vertex = Vec3::by_axis(|a| if tile.direction[a] >= 0 {vertex[a]} else {1.0 - vertex[a]});
				cell.vertices.push(ModelVertex {
					position: pos + vertex,
					normal: tile.direction.as_type::<f32>().normalize(),
					uv: (uv_corner + uv_unbleed(face_mesh_to_uv(vertex, uv_direction))) / TILE_TEXTURE_ATLAS_SIZE as f32,
				});
			}
		}
	}
	
	// Add missing faces on neighboring cell boundaries
	for d in [PX, PY, PZ, NX, NY, NZ] {
		if let Some(other_cell) = cells.get_mut(&(location - Vec3::<isize>::unit(d))) {
			for tile_pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type::<usize>().with(d.axis(), 1)) {
				let mut this_tile_pos = tile_pos;
				let mut other_tile_pos = tile_pos.with(d.axis(), CELL_SIZE[d.axis()] as usize - 1);
				if d.is_negative() { (this_tile_pos, other_tile_pos) = (other_tile_pos, this_tile_pos); }
				
				let tile = other_cell.tiles[other_tile_pos];
				if tile.is_empty() { continue }
				
				if !tile_has_full_face(-d, cell.tiles[this_tile_pos]) {
					add_face(&mut other_cell.vertices, &mut other_cell.indices, other_tile_pos, d, tile);
				}
			}
			
			other_cell.update_mesh_buffers = true;
		}
	}
	
	cell.update_mesh_buffers = true;
}

