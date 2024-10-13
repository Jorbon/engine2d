use num_traits::ConstZero;

use crate::*;


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



pub fn build_cell_mesh(new_cell: &mut Cell, location: Vec3<isize>, cells: &mut HashMap<Vec3<isize>, Cell>) {
	for pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type()) {
		match new_cell.tiles[pos] {
			Air | Water => (),
			Block(material) => {
				for d in [PX, PY, PZ, NX, NY, NZ] {
					if !adjacent_tile_has_full_face(d,
						if match d.is_positive() {
							true => pos[d.axis()] < CELL_SIZE[d.axis()] as usize - 1,
							false => pos[d.axis()] > 0,
						} {
							new_cell.tiles[(pos.as_type::<isize>() + Vec3::<isize>::unit(d)).as_type::<usize>()]
						} else if let Some(other_cell) = cells.get(&(location + Vec3::<isize>::unit(d))) {
							other_cell.tiles[pos.with(d.axis(), if d.is_positive() {0} else {CELL_SIZE[d.axis()] as usize - 1})]
						} else { continue }
					) {
						add_face(&mut new_cell.vertices, &mut new_cell.indices, pos, material, d);
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
				let normal = direction.as_type::<f32>().normalize();
				let uv_corner = material.get_uv().as_type::<f32>();
				
				let index_base = new_cell.vertices.len() as ModelIndex;
				let index_iter = match v.len() {
					3 => [0, 1, 2].iter(),
					4 => [0, 1, 2, 0, 2, 3].iter(),
					5 => [0, 1, 2, 0, 2, 3, 0, 3, 4].iter(),
					6 => [0, 2, 4, 0, 1, 2, 2, 3, 4, 4, 5, 0].iter(),
					n => panic!("{n} vertices on slope face")
				};
				
				new_cell.indices.append(&mut match {
					let mut reverse = true;
					direction.map(|v| if v < 0 { reverse = !reverse });
					reverse
				} {
					false => index_iter.map(|i| i + index_base).collect(),
					true => index_iter.rev().map(|i| i + index_base).collect(),
				});
				
				for vertex in v {
					let vertex = Vec3::by_axis(|a| if direction[a] >= 0 {vertex[a]} else {1.0 - vertex[a]});
					new_cell.vertices.push(ModelVertex {
						position: pos + vertex,
						normal,
						uv: uv_corner + vertex.xy(),
					});
				}
				
				
			}
		}
	}
	
	for d in [PX, PY, PZ, NX, NY, NZ] {
		if let Some(other_cell) = cells.get_mut(&(location - Vec3::<isize>::unit(d))) {
			for tile_pos in Vec3Range::<usize, ZYX>::exclusive(Vec3::ZERO, CELL_SIZE.as_type::<usize>().with(d.axis(), 1)) {
				let mut this_tile_pos = tile_pos;
				let mut other_tile_pos = tile_pos.with(d.axis(), CELL_SIZE[d.axis()] as usize - 1);
				if d.is_negative() { (this_tile_pos, other_tile_pos) = (other_tile_pos, this_tile_pos); }
				
				match other_cell.tiles[other_tile_pos] {
					Air | Water | Ramp(..) => (),
					Block(material) => if !adjacent_tile_has_full_face(d, new_cell.tiles[this_tile_pos]) {
						add_face(&mut other_cell.vertices, &mut other_cell.indices, other_tile_pos, material, d);
					}
				}
			}
			
			other_cell.update_mesh = true;
		}
	}
	
	new_cell.update_mesh = true;
}

