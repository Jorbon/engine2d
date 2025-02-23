use crate::*;


#[derive(Copy, Clone, Debug)]
pub struct Contact {
	pub normal: Vec3<f64>,
	pub material: Material,
	pub displacement: f64,
}


// MARK: Detect Contacts

// Decide what surfaces the hitbox is in contact with
pub fn detect_contacts(cells: &HashMap<Vec3<isize>, Cell>, l: Vec3<f64>, h: Vec3<f64>) -> Vec<Contact> {
	Vec3Range::<isize, ZYX>::inclusive(
		(l - Vec3::all(SURFACE_MARGIN)).floor_to(),
		(h + Vec3::all(SURFACE_MARGIN)).floor_to()
	).map(|tile_pos| test_contact(l, h, cells, tile_pos)).flatten().collect::<Vec<_>>()
}


fn test_contact(l: Vec3<f64>, h: Vec3<f64>, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>) -> Vec<Contact> {
	let cell_pos = tile_pos >> CELL_SIZE_BITS;
	if let Some(cell) = cells.get(&cell_pos) {
		let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
		
		match tile.state() {
			TileState::Empty => vec![],
			TileState::Full => match test_contact_full_block(l, h, tile_pos) {
				Some((direction, displacement)) => vec![Contact {
					normal: Vec3::<f64>::unit(direction),
					material: tile.material,
					displacement,
				}],
				None => vec![]
			}
			TileState::Partial => match test_contact_slope(l, h, tile_pos, tile.direction, tile.level) {
				Some((normal, displacement)) => vec![Contact {
					normal,
					material: tile.material,
					displacement,
				}],
				None => vec![]
			}
		}
	} else { vec![] }
}

fn test_contact_full_block(l: Vec3<f64>, h: Vec3<f64>, tile_pos: Vec3<isize>) -> Option<(Direction, f64)> {	
	let h_inset = h - tile_pos.as_type::<f64>();
	let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
	
	for a in [Z, Y, X] {
		if h_inset[a.l()] > 0.0 && l_inset[a.l()] > 0.0
		&& h_inset[a.r()] > 0.0 && l_inset[a.r()] > 0.0 {
			if h_inset[a].abs() < SURFACE_MARGIN {
				return Some((a.n(), h_inset[a]));
			}
			if l_inset[a].abs() < SURFACE_MARGIN {
				return Some((a.p(), l_inset[a]));
			}
		}
	}
	
	None
}

fn test_contact_slope(l: Vec3<f64>, h: Vec3<f64>, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8) -> Option<(Vec3<f64>, f64)> {
	
	{ // Decide if we even need to run this at all
		let mut positive_sum = 0;
		let mut negative_sum = 0;
		direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
		
		if level <= negative_sum { return None }
		if level >= positive_sum { return test_contact_full_block(l, h, tile_pos).map(|(d, displacement)| (Vec3::<f64>::unit(d), displacement)) }
	}
	
	
	let near_corner = Vec3::by_axis(|a| if direction[a] >= 0 {l[a]} else {h[a]});
	
	let slope_normal = direction.as_type::<f64>();
	let slope_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
	
	// let h_inset = h - tile_pos.as_type::<f64>();
	// let l_inset = tile_pos.as_type::<f64>() + Vec3(1.0, 1.0, 1.0) - l;
	
	// Main slope face
	for _ in std::iter::once(()) {
		let current_s = near_corner.dot(slope_normal);
		let s_inset = (slope_s - current_s) / slope_normal.length();
		
		if s_inset.abs() < SURFACE_MARGIN
		&& near_corner.x() + 1e-10 >= tile_pos.x() as f64 && near_corner.x() <= tile_pos.x() as f64 + 1.0 + 1e-10
		&& near_corner.y() + 1e-10 >= tile_pos.y() as f64 && near_corner.y() <= tile_pos.y() as f64 + 1.0 + 1e-10
		&& near_corner.z() + 1e-10 >= tile_pos.z() as f64 && near_corner.z() <= tile_pos.z() as f64 + 1.0 + 1e-10 {
			return Some((slope_normal.normalize(), s_inset))
		}
	}
	
	// Acute edges
	for a in [X, Y, Z] {
		if level >= direction[a].min(0) + direction[a.l()].max(0) + direction[a.r()].max(0) { continue }
		
		let edge_normal = slope_normal.get_plane(a);
		
		let plane_tile_pos = tile_pos[a];
		let tile_pos = tile_pos.get_plane(a);
		let near_edge = near_corner.get_plane(a);
		
		let edge_s = (tile_pos.dot(direction.get_plane(a).as_type::<isize>()) + (level - direction[a].min(0)) as isize) as f64;
		let current_s = near_edge.dot(edge_normal);
		let s_inset = (edge_s - current_s) / edge_normal.length();
		
		let plane_relative_position = if direction[a] >= 0 { plane_tile_pos } else { plane_tile_pos + 1 } as f64;
		
		if s_inset.abs() < SURFACE_MARGIN
		&& near_edge.x() + 1e-10 >= tile_pos.x() as f64 && near_edge.x() <= tile_pos.x() as f64 + 1.0 + 1e-10
		&& near_edge.y() + 1e-10 >= tile_pos.y() as f64 && near_edge.y() <= tile_pos.y() as f64 + 1.0 + 1e-10
		&& plane_relative_position > l[a] && plane_relative_position < h[a] {
			return Some((edge_normal.normalize().vec3(a), s_inset))
		}
	}
	
	// Acute corners
	for a in [X, Y, Z] {
		if level > direction[a].max(0) + direction[a.l()].min(0) + direction[a.r()].min(0) { continue }
		
		let corner_s = (tile_pos[a] * direction[a] as isize + (level - direction[a.l()].min(0) - direction[a.r()].min(0)) as isize) as f64;
		let current_s = near_corner[a] * slope_normal[a];
		let s_inset = (corner_s - current_s) / slope_normal[a];
		
		let corner_relative_position = Vec2(
			if direction[a.l()] >= 0 { tile_pos[a.l()] } else { tile_pos[a.l()] + 1 } as f64,
			if direction[a.r()] >= 0 { tile_pos[a.r()] } else { tile_pos[a.r()] + 1 } as f64,
		);
		
		if s_inset.abs() < SURFACE_MARGIN
		&& corner_relative_position.x() > l[a.l()] && corner_relative_position.x() < h[a.l()]
		&& corner_relative_position.y() > l[a.r()] && corner_relative_position.y() < h[a.r()] {
			return Some((Vec3::unit(match direction[a] >= 0 { true => a.p(), false => a.n() }), s_inset))
		}
	}
	
	// Regular face contact
	let (d, displacement) = test_contact_full_block(l, h, tile_pos)?;
	let a = d.axis();
	let contacting_corner = near_corner.with(a, if d.is_positive() {l[a]} else {h[a]});
	let contacting_point = Vec3::by_axis(|a| contacting_corner[a].clamp(tile_pos[a] as f64, tile_pos[a] as f64 + 1.0));
	
	if contacting_point.dot(slope_normal) + 1e-10 < slope_s {
		Some((Vec3::unit(d), displacement))
	} else {
		None
	}
}


