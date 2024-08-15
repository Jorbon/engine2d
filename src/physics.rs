use crate::*;



pub fn get_cell(cells: &Vec<(Vec3<isize>, Cell)>, cell_pos: Vec3<isize>) -> Option<&Cell> {
	for (pos, cell) in cells {
		if *pos == cell_pos {
			return Some(cell);
		}
	}
	return None;
}


pub fn raycast(cells: &Vec<(Vec3<isize>, Cell)>, start: Vec3f, end: Vec3f, max_t: f32) -> Option<(f32, Vec3f)> {
	let x_reversed = start.x() > end.x();
	let y_reversed = start.y() > end.y();
	let z_reversed = start.z() > end.z();
	
	let mut current_pos = Vec3(start.x().floor() as isize, start.y().floor() as isize, start.z().floor() as isize);
	let mut current_t = 0.0;
	
	let mut t_next_x = prel(start.x(), end.x(), (current_pos.x() + if x_reversed { 0 } else { 1 }) as f32);
	let mut t_next_y = prel(start.y(), end.y(), (current_pos.y() + if y_reversed { 0 } else { 1 }) as f32);
	let mut t_next_z = prel(start.z(), end.z(), (current_pos.z() + if z_reversed { 0 } else { 1 }) as f32);
	
	let mut current_cell_location = Vec3(current_pos.x() >> CELL_WIDTH_BITS, current_pos.y() >> CELL_WIDTH_BITS, current_pos.z() >> CELL_HEIGHT_BITS);
	let mut current_cell = get_cell(cells, current_cell_location);
	
	loop {
		if let Some(Cell { tiles }) = current_cell {
			let cell_tile_pos = Vec3(
				(current_pos.x() - (current_cell_location.x() << CELL_WIDTH_BITS)) as usize,
				(current_pos.y() - (current_cell_location.y() << CELL_WIDTH_BITS)) as usize,
				(current_pos.z() - (current_cell_location.z() << CELL_HEIGHT_BITS)) as usize,
			);
			match tiles[cell_tile_pos.z()][cell_tile_pos.y()][cell_tile_pos.x()] {
				Air | Water => {}
				HTrack | VTrack => {}
				Block(_material) => {
					let pos = lerp(start, end, current_t);
					let subtile_pos = pos - Vec3(current_pos.x() as f32, current_pos.y() as f32, current_pos.z() as f32);
					return Some((current_t, match (
						subtile_pos.x() < subtile_pos.y(),
						subtile_pos.x() < subtile_pos.z(),
						subtile_pos.y() < subtile_pos.z(),
						subtile_pos.x() < 1.0 - subtile_pos.y(),
						subtile_pos.x() < 1.0 - subtile_pos.z(),
						subtile_pos.y() < 1.0 - subtile_pos.z(),
					) {
						(true, true, _, true, true, _) => Vec3(-1.0, 0.0, 0.0),
						(false, false, _, false, false, _) => Vec3(1.0, 0.0, 0.0),
						(false, _, true, true, _, true) => Vec3(0.0, -1.0, 0.0),
						(true, _, false, false, _, false) => Vec3(0.0, 1.0, 0.0),
						(_, false, false, _, true, true) => Vec3(0.0, 0.0, -1.0),
						(_, true, true, _, false, false) => Vec3(0.0, 0.0, 1.0),
						_ => unreachable!()
					}));
				}
				Ramp(_material, _direction, _level) => {
					return None
				}
			}
		}
		
		match (t_next_x <= t_next_y, t_next_x <= t_next_z, t_next_y <= t_next_z) {
			(true, true, _) => {
				if t_next_x > max_t { return None }
				current_t = t_next_x;
				current_pos.0 += if x_reversed { -1 } else { 1 };
				if current_pos.x() >> CELL_WIDTH_BITS != current_cell_location.x() {
					current_cell_location.0 = current_pos.x() >> CELL_WIDTH_BITS;
					current_cell = get_cell(cells, current_cell_location);
				}
				t_next_x = prel(start.x(), end.x(), (current_pos.x() + if x_reversed { -1 } else { 1 }) as f32);
			}
			(false, _, true) => {
				if t_next_y > max_t { return None }
				current_t = t_next_y;
				current_pos.1 += if y_reversed { -1 } else { 1 };
				if current_pos.y() >> CELL_WIDTH_BITS != current_cell_location.y() {
					current_cell_location.1 = current_pos.y() >> CELL_WIDTH_BITS;
					current_cell = get_cell(cells, current_cell_location);
				}
				t_next_y = prel(start.y(), end.y(), (current_pos.y() + if y_reversed { -1 } else { 1 }) as f32);
			}
			(_, false, false) => {
				if t_next_z > max_t { return None }
				current_t = t_next_z;
				current_pos.2 += if z_reversed { -1 } else { 1 };
				if current_pos.z() >> CELL_HEIGHT_BITS != current_cell_location.z() {
					current_cell_location.2 = current_pos.z() >> CELL_WIDTH_BITS;
					current_cell = get_cell(cells, current_cell_location);
				}
				t_next_z = prel(start.z(), end.z(), (current_pos.z() + if z_reversed { -1 } else { 1 }) as f32);
			}
			_ => unreachable!()
		}
	}
	
	
}



