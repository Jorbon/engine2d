mod constraints;
mod contact;
mod collision;
mod movement;

use constraints::ConstraintSet;

use crate::*;


fn get_force(entity: &Entity) -> Vec3<f64> { // MARK: get_force
	movement::force_from_inputs(entity) + Vec3(0.0, 0.0, -9.8) * entity.mass
}



pub const SURFACE_MARGIN: f64 = 1e-4;
pub const MIN_V_BOUNCE: f64 = 0.1;


pub fn physics_step(entity: &mut Entity, cells: &HashMap<Vec3<isize>, Cell>, dt: f64) { // MARK: Physics Step
	
	entity.velocity += get_force(entity) / entity.mass * dt;
	
	
	let mut l = entity.position + entity.size.scale(LOW_CORNER);
	let mut h = entity.position + entity.size.scale(HIGH_CORNER);
	
	let contacts = contact::detect_contacts(cells, l, h);
	
	// todo: jump direction evaluation
	if entity.jump_input {
		for contact in &contacts {
			entity.velocity += contact.normal * 5.0;
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
				contact::detect_contacts(cells, l, h)
			}
		};
		
		// todo: resolve displacements smarter
		// for (normal, displacement) in &contacts {
		// 	// entity.position += *normal * displacement;
		// }
		
		
		let (new_velocity, constraint_set) = constraints::find_constraints(entity.velocity, contacts);
		entity.velocity = new_velocity;
		
		
		
		
		if let Some(collision) = collision::detect_next_collision(entity, cells, l, h, dt_remaining) {
			entity.position += entity.velocity * collision.dt;
			
			// todo: friction on collision and sub-step movement
			
			let v_projected = entity.velocity.dot(collision.normal);
			entity.velocity -= collision.normal * v_projected * if v_projected < -MIN_V_BOUNCE {1.0 + collision.material.get_properties().bounciness} else {1.0};
			
			dt_remaining -= collision.dt;
			
			continue
		} else {
			
			// todo: unjank seriously
			if let ConstraintSet::Single(constraint) = constraint_set {
				
				let speed = entity.velocity.length();
				let friction_delta_v = (
					constraint.material_properties.friction_constant + 
					constraint.material_properties.friction_linear * speed
				) * constraint.delta_v;
				let new_speed = speed - friction_delta_v;
				
				if new_speed > 0.0 {
					entity.velocity = entity.velocity.normalize() * new_speed;
				} else {
					entity.velocity = Vec3::ZERO;
				}
			}
			
			entity.position += entity.velocity * dt_remaining;
			
			break
		}
		
	}
}



