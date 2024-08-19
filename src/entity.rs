use std::{collections::HashMap, path::Path};

use glium::{texture::SrgbTexture2d, Display};

use crate::*;




pub enum FacingDirection { Up, Down, Left, Right }

pub struct Entity {
    pub position: Vec3<f64>,
    pub velocity: Vec3<f64>,
    pub size: Vec3<f64>,
    pub direction: FacingDirection,
    pub ground_speed: f64,
	pub air_speed: f64,
	pub water_speed: f64,
	pub ground_acceleration: f64,
	pub air_acceleration: f64,
	pub water_acceleration: f64,
	pub air_resistance: f64,
    pub sprites: SpriteSet,
	pub movement_input: Vec3<f64>,
	pub jump_input: bool,
}

#[derive(PartialEq)]
pub enum EntityStatus {
	Grounded(Vec<Vec3<f64>>),
	Falling,
	Swimming,
}

pub enum SpriteSet {
	Static(SrgbTexture2d),
	Directional([SrgbTexture2d; 4])
}

impl SpriteSet {
	pub fn load(display: &Display, path: &str) -> Self {
		let full_path = format!("assets/textures/{path}.png");
		if Path::new(&full_path).is_file() {
			SpriteSet::Static(load_texture(display, path))
		} else {
			SpriteSet::Directional([
				load_texture(display, &format!("{path}/up")),
				load_texture(display, &format!("{path}/down")),
				load_texture(display, &format!("{path}/left")),
				load_texture(display, &format!("{path}/right")),
			])
		}
	}
}



const LOW_CORNER: Vec3<f64> = Vec3(-0.5, -0.5, 0.0);
const HIGH_CORNER: Vec3<f64> = Vec3(0.5, 0.5, 1.0);

const SURFACE_MARGIN: f64 = 1e-4;


impl Entity {
	pub fn new(position: Vec3<f64>, size: Vec3<f64>, sprites: SpriteSet) -> Self {
		Self {
			position,
			velocity: Vec3(0.0, 0.0, 0.0),
			size,
			direction: FacingDirection::Down,
			ground_speed: 5.0,
			air_speed: 12.0,
			water_speed: 3.0,
			ground_acceleration: 60.0,
			air_acceleration: 4.0,
			water_acceleration: 25.0,
			air_resistance: 0.001,
			sprites,
			movement_input: Vec3(0.0, 0.0, 0.0),
			jump_input: false,
		}
	}
	
	pub fn current_sprite(&self) -> &SrgbTexture2d {
		match &self.sprites {
			SpriteSet::Static(sprite) => &sprite,
			SpriteSet::Directional([up, down, left, right]) => match self.direction {
				FacingDirection::Up => &up,
				FacingDirection::Down => &down,
				FacingDirection::Left => &left,
				FacingDirection::Right => &right,
			}
		}
	}
	
	// pub fn movement(&mut self, mut input: Vec3<f64>) {
	// 	if input.is_zero() { return }
		
	// 	let (acceleration, speed) = self.get_acceleration_speed();
		
	// 	let input_length = input.length();
	// 	let input_direction = input.normalize();
		
	// 	let wish_acceleration = acceleration * input_length * dt;
	// 	let target_velocity = speed * input_length;
	// 	let current_velocity = self.velocity.dot(input_direction);
		
	// 	let (acceleration_parameter, deceleration_parameter) = match self.status {
	// 		EntityStatus::Grounded(_) => (2.0, 0.0),
	// 		EntityStatus::Falling => (1.0, 1.0),
	// 		EntityStatus::Swimming => (1.5, 0.5),
	// 	};
		
	// 	if current_velocity < -target_velocity { // reduced so that it doesn't act like 2x friction
	// 		self.velocity += input_direction * f64::min(deceleration_parameter * wish_acceleration, target_velocity - current_velocity);
	// 	} else if current_velocity < target_velocity { // 2x because 1x counters friction
	// 		self.velocity += input_direction * f64::min(acceleration_parameter * wish_acceleration, target_velocity - current_velocity);
	// 	}
		
	// 		 if input.y() < -input.x().abs() { self.direction = FacingDirection::Up; }
	// 	else if input.y() >  input.x().abs() { self.direction = FacingDirection::Down; }
	// 	else if input.x() < -input.y().abs() { self.direction = FacingDirection::Left; }
	// 	else if input.x() >  input.y().abs() { self.direction = FacingDirection::Right; }
	// }
	
	pub fn get_force_direction(&self) -> Vec3<f64> {
		Vec3(0.0, 0.0, -1.0)
	}
	
	pub fn get_force_magnitude(&self) -> f64 {
		9.8
	}
	
	pub fn physics_step(&mut self, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) {
		let mut normals = vec![];
		
		let l = self.position + self.size.scale(LOW_CORNER);
		let h = self.position + self.size.scale(HIGH_CORNER);
		
		for tile_pos in Vec3Range::<isize, ZYX>::inclusive((l - Vec3::all(SURFACE_MARGIN)).floor_to(), (h + Vec3::all(SURFACE_MARGIN)).floor_to()) {
			let cell_pos = tile_pos >> CELL_SIZE_BITS;
			if let Some(cell) = cells.get(&cell_pos) {
				match cell.tiles[(tile_pos & CELL_MASK).as_type()] {
					Tile::Air | Tile::Water | Tile::HTrack | Tile::VTrack => (),
					Tile::Block(_material) => {
						let h_inset = h - tile_pos.as_type::<f64>();
						let l_inset = (tile_pos + Vec3(1, 1, 1)).as_type::<f64>() - l;
						
						for (a, b, c) in [(Z, X, Y), (Y, X, Z), (X, Y, Z)] {
							if h_inset[b] > SURFACE_MARGIN && l_inset[b] > SURFACE_MARGIN
							&& h_inset[c] > SURFACE_MARGIN && l_inset[c] > SURFACE_MARGIN {
								if h_inset[a].abs() < SURFACE_MARGIN {
									self.position[a] = tile_pos[a] as f64 - self.size[a] * HIGH_CORNER[a];
									normals.push(-Vec3::<f64>::unit(a));
								}
								if l_inset[a].abs() < SURFACE_MARGIN {
									self.position[a] = (tile_pos[a] + 1) as f64 - self.size[a] * LOW_CORNER[a];
									normals.push(Vec3::unit(a));
								}
							}
						}
					}
					Tile::Ramp(_material, direction, level) => {
						todo!()
					}
				}
			}
		}
		
		
		self.velocity += self.get_force_direction() * self.get_force_magnitude() * dt;
		
		self.velocity += self.movement_input * dt * 5.0;
		
		if self.jump_input && normals.len() > 0 {
			self.velocity -= self.get_force_direction() * 2.0;
		}
		
		for normal in normals {
			if self.velocity.dot(normal) < 0.0 {
				self.velocity -= normal * self.velocity.dot(normal);
			}
		}
		
		
		
		let mut dt_remaining = dt;
		
		loop {
			
			let mut first_collision = None;
			let mut first_collision_t = dt_remaining;
			
			let reversed = self.velocity.map(|v| v < 0.0);
			let step = reversed.map(|r| match r { false => 1, true => -1 });
			
			let main_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => HIGH_CORNER[a], true => LOW_CORNER[a] }));
			let far_corner = self.position + self.size.scale(Vec3::by_axis(|a| match reversed[a] { false => LOW_CORNER[a], true => HIGH_CORNER[a] }));
			
			let mut main_tile = Vec3::by_axis(|a| if reversed[a] { main_corner[a].floor() } else { main_corner[a].ceil() - 1.0 } as isize);
			let far_tile = Vec3::by_axis(|a| if reversed[a] { far_corner[a].ceil() - 1.0 } else { far_corner[a].floor() } as isize);
			
			for (axis, check_axis) in [(Z, None), (Y, Some(Z)), (X, Some(Y))] {
				if let Some(ca) = check_axis {
					if main_tile[ca] == far_tile[ca] {
						break
					} else {
						main_tile[ca] -= step[ca];
					}
				}
				
				for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile.with(axis, main_tile[axis])) {
					if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
						first_collision = Some(collision);
						first_collision_t = collision.0;
					}
				}
			}
			
			if first_collision.is_none() {
				let mut current_tile = main_corner.floor_to::<isize>();
				let mut next_tile_boundary = current_tile + reversed.map(|r| if r {0} else {1});
				
				loop {
					let t_next = Vec3::by_axis(|a| prel(main_corner[a], main_corner[a] + self.velocity[a], next_tile_boundary[a] as f64));
					let a = match (t_next.x() < t_next.y(), t_next.x() < t_next.z(), t_next.y() < t_next.z()) {
						(true, true, _) => X,
						(false, _, true) => Y,
						(_, false, false) => Z,
						_ => unreachable!()
					};
					if t_next[a] > dt_remaining { break }
					current_tile += step.component(a);
					next_tile_boundary += step.component(a);
					
					let current_t = t_next[a];
					let current_main_pos = main_corner + self.velocity * current_t;
					let current_far_pos = far_corner + self.velocity * current_t;
					let main_tile = Vec3::by_axis(|a| if reversed[a] { current_main_pos[a].floor() } else { current_main_pos[a].ceil() - 1.0 } as isize).with(a, current_tile[a]);
					let far_tile = Vec3::by_axis(|a| if reversed[a] { current_far_pos[a].ceil() - 1.0 } else { current_far_pos[a].floor() } as isize).with(a, current_tile[a]);
					for tile_pos in Vec3Range::<isize, ZYX>::inclusive(main_tile, far_tile) {
						if let Some(collision) = self.test_collision(cells, tile_pos, first_collision_t) {
							first_collision = Some(collision);
							first_collision_t = collision.0;
						}
					}
				}
			}
			
			if let Some((t, normal)) = first_collision {
				self.position += self.velocity * t;
				self.velocity -= normal * self.velocity.dot(normal);
				dt_remaining -= t;
				continue
			} else {
				self.position += self.velocity * dt_remaining;
				break
			}
			
		}
	}
	
	pub fn test_collision(&self, cells: &HashMap<Vec3<isize>, Cell>, tile_pos: Vec3<isize>, max_t: f64) -> Option<(f64, Vec3<f64>)> {
		let cell_pos = tile_pos >> CELL_SIZE_BITS;
		if let Some(cell) = cells.get(&cell_pos) {
			match cell.tiles[(tile_pos & CELL_MASK).as_type()] {
				Tile::Air | Tile::Water | Tile::HTrack | Tile::VTrack => None,
				Tile::Block(_material) => {
					let l = self.position + self.size.scale(LOW_CORNER);
					let h = self.position + self.size.scale(HIGH_CORNER);
					for a in [Z, Y, X] {
						
						if self.velocity[a] < 0.0 {
							let t = prel(l[a], l[a] + self.velocity[a], tile_pos[a] as f64 + 1.0);
							if t >= 0.0 && t <= max_t {
								return Some((t, Vec3::unit(a)))
							}
						} else if self.velocity[a] > 0.0 {
							let t = prel(h[a], h[a] + self.velocity[a], tile_pos[a] as f64);
							if t >= 0.0 && t <= max_t {
								return Some((t, -Vec3::<f64>::unit(a)))
							}
						}
					}
					None
				}
				Tile::Ramp(_material, direction, level) => {
					todo!()
				}
			}
		} else {
			None
		}
	}
}
