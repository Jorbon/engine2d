use crate::*;


pub fn cast_ray(cells: &HashMap<Vec3<isize>, Cell>, origin: Vec3<f64>, ray: Vec3<f64>) -> Option<(Vec3<isize>, Vec3<f64>)> {
	let reversed = ray.map(|v| v < 0.0);
	let step = reversed.map(|r| match r { false => 1, true => -1 });
	
	let mut current_tile = Vec3::by_axis(|a| match reversed[a] { false => origin[a].floor(), true => origin[a].ceil() - 1.0 } as isize);
	
	if let Some(collision) = cast_ray_on_tile(cells, origin, ray, current_tile, None) {
		return Some((current_tile, collision))
	}
	
	
	// Next, visit each tile boundary encounter in chronological order
	let mut next_tile_boundary = current_tile + reversed.map(|r| if r {0} else {1});
	
	loop {
		let t_next = Vec3::by_axis(|a| prel(origin[a], origin[a] + ray[a], next_tile_boundary[a] as f64)).map(|v| if v < 0.0 {f64::INFINITY} else {v});
		let a = match (t_next.x() < t_next.y(), t_next.x() < t_next.z(), t_next.y() < t_next.z()) {
			(true, true, _) => X,
			(false, _, true) => Y,
			(_, false, false) => Z,
			_ => unreachable!()
		};
		
		let current_t = t_next[a];
		if t_next[a] > 1.0 {
			return None
		}
		
		current_tile += step.component(a);
		next_tile_boundary += step.component(a);
		
		let tile_incidence = origin + ray * current_t;
		
		if let Some(collision) = cast_ray_on_tile(cells, origin, ray, current_tile, Some(tile_incidence)) {
			return Some((current_tile, collision))
		}
	}
}

fn cast_ray_on_tile(cells: &HashMap<Vec3<isize>, Cell>, origin: Vec3<f64>, ray: Vec3<f64>, tile_pos: Vec3<isize>, tile_incidence: Option<Vec3<f64>>) -> Option<Vec3<f64>> {
	let cell_pos = tile_pos >> CELL_SIZE_BITS;
	if let Some(cell) = cells.get(&cell_pos) {
		let tile = cell.tiles[(tile_pos & CELL_MASK).as_type()];
		
		match tile.state() {
			TileState::Empty => None,
			TileState::Full => Some(tile_incidence?),
			TileState::Partial => cast_ray_on_slope(origin, ray, tile_pos, tile.direction, tile.level, tile_incidence),
		}
	} else { None }
}

fn cast_ray_on_slope(origin: Vec3<f64>, ray: Vec3<f64>, tile_pos: Vec3<isize>, direction: Vec3<i8>, level: i8, tile_incidence: Option<Vec3<f64>>) -> Option<Vec3<f64>> {
	{ // Decide if we even need to run this at all
		let mut positive_sum = 0;
		let mut negative_sum = 0;
		direction.map(|v| if v >= 0 { positive_sum += v; } else { negative_sum += v; });
		
		if level <= negative_sum { return None }
		if level >= positive_sum { return tile_incidence }
	}
	
	let slope_normal = direction.as_type::<f64>();
	let slope_s = (tile_pos.dot(direction.as_type::<isize>()) + level as isize) as f64;
	
	if let Some(incidence) = tile_incidence {
		if incidence.dot(slope_normal) <= slope_s {
			return Some(incidence)
		}
	}
	
	let s_velocity = ray.dot(slope_normal);
	if s_velocity > 0.0 { return None }
	
	let origin_s = origin.dot(slope_normal);
	if origin_s < slope_s { return None }
	
	let t = (slope_s - origin_s) / s_velocity;
	if t > 1.0 { return None }
	
	let collision_pos = origin + ray * t;
	if collision_pos.x() >= tile_pos.x() as f64 && collision_pos.x() <= tile_pos.x() as f64 + 1.0
	&& collision_pos.y() >= tile_pos.y() as f64 && collision_pos.y() <= tile_pos.y() as f64 + 1.0
	&& collision_pos.z() >= tile_pos.z() as f64 && collision_pos.z() <= tile_pos.z() as f64 + 1.0 {
		Some(collision_pos)
	} else {
		None
	}
}

