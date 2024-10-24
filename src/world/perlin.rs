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

