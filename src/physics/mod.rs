mod collision;
mod movement;

use crate::*;

fn get_force(entity: &Entity) -> Vec3<f64> { // MARK: get_force
	movement::force_from_inputs(entity) + Vec3(0.0, 0.0, -9.8)
}



pub const SURFACE_MARGIN: f64 = 1e-4;
pub const MIN_V_BOUNCE: f64 = 0.1;


pub fn physics_step(entity: &mut Entity, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) { // MARK: Physics Step
	
	entity.velocity += get_force(entity) * dt;
	
	
	let mut l = entity.position + entity.size.scale(LOW_CORNER);
	let mut h = entity.position + entity.size.scale(HIGH_CORNER);
	
	let contacts = collision::detect_contacts(cells, l, h);
	
	// todo: jump direction evaluation
	if entity.jump_input {
		for (normal, _) in &contacts {
			entity.velocity += *normal * 5.0;
			break
		}
	}
	
	
	let mut contacts_iter = std::iter::once(contacts);
	let mut dt_remaining = dt;
	
	loop {
		
		let contacts = match contacts_iter.next() {
			Some(contacts) => contacts,
			None => {
				l = entity.position + entity.size.scale(LOW_CORNER);
				h = entity.position + entity.size.scale(HIGH_CORNER);
				collision::detect_contacts(cells, l, h)
			}
		};
		
		// todo: resolve displacements smarter
		// for (normal, displacement) in &contacts {
		// 	// entity.position += *normal * displacement;
		// }
		
		let contacts = {
			let mut filtered_contacts: Vec<Vec3<f64>> = vec![];
			
			for (normal, _) in contacts {
				let mut unique = true;
				for other_normal in &filtered_contacts {
					if (normal.x() - other_normal.x()).abs() < 1e-7
					&& (normal.y() - other_normal.y()).abs() < 1e-7
					&& (normal.z() - other_normal.z()).abs() < 1e-7 {
						unique = false;
						break
					}
				}
				
				if unique {
					filtered_contacts.push(normal);
				}
			}
			
			filtered_contacts
		};
		
		entity.velocity = collision::constrain_velocity(entity.velocity, contacts);
		
		if let Some((t, normal)) = collision::detect_next_collision(entity, cells, l, h, dt_remaining) {
			entity.position += entity.velocity * t;
			
			let bounce = 0.0;
			
			let v_projected = entity.velocity.dot(normal);
			entity.velocity -= normal * v_projected * if v_projected < -MIN_V_BOUNCE {1.0 + bounce} else {1.0};
			
			dt_remaining -= t;
			
			continue
		} else {
			entity.position += entity.velocity * dt_remaining;
			break
		}
		
	}
}



