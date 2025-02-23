use crate::*;

use super::contact::Contact;


#[derive(Clone, Debug)]
pub struct Constraint {
	pub normal: Vec3<f64>,
	pub material_properties: MaterialProperties,
	pub delta_v: f64,
}


#[derive(Clone, Debug)]
pub enum ConstraintSet {
	None,
	Single(Constraint),
	Double(Constraint, Constraint),
	Triple,
}


pub fn find_constraints(velocity: Vec3<f64>, contacts: Vec<Contact>) -> (Vec3<f64>, ConstraintSet) {
	
	let mut filtered_contacts: Vec<(Vec3<f64>, MaterialProperties)> = vec![];
	
	for contact in contacts {
		let mut unique = true;
		for other_contact in &mut filtered_contacts {
			if (contact.normal.x() - other_contact.0.x()).abs() < 1e-7
			&& (contact.normal.y() - other_contact.0.y()).abs() < 1e-7
			&& (contact.normal.z() - other_contact.0.z()).abs() < 1e-7 {
				unique = false;
				other_contact.1 = other_contact.1.merge_with(contact.material.get_properties());
				break
			}
		}
		
		if unique {
			filtered_contacts.push((contact.normal, contact.material.get_properties()));
		}
	}
	
	
	let mut opposing = vec![];
	let mut remainder = vec![];
	for (normal, materials) in filtered_contacts {
		if velocity.dot(normal) < 0.0 {
			opposing.push((normal, materials));
		} else {
			remainder.push((normal, materials));
		}
	}
	
	if opposing.len() == 0 {
		return (velocity, ConstraintSet::None)
	}
	
	for first_contact in &opposing {
		let moment1 = -velocity.dot(first_contact.0);
		let new_velocity = velocity + first_contact.0 * moment1;
		
		let mut valid = true;
		for other_contact in &opposing {
			if other_contact == first_contact { continue }
			if new_velocity.dot(other_contact.0) < 0.0 {
				valid = false;
				break
			}
		}
		
		if valid {
			let contacts = remainder;
			
			let mut opposing: Vec<_> = vec![];
			let mut remainder = vec![];
			for contact in contacts {
				if new_velocity.dot(contact.0) < 0.0 {
					opposing.push(contact);
				} else {
					remainder.push(contact);
				}
			}
			
			if opposing.len() == 0 {
				return (
					new_velocity,
					ConstraintSet::Single(Constraint {
						normal: first_contact.0,
						delta_v: moment1,
						material_properties: first_contact.1,
					})
				)
			}
			
			for second_contact in &opposing {
				let mut newer_velocity_direction = first_contact.0.cross(second_contact.0);
				if new_velocity.dot(newer_velocity_direction) < 0.0 {
					newer_velocity_direction = -newer_velocity_direction;
				}
				
				let mut valid = true;
				for other_contact in &opposing {
					if other_contact == first_contact || other_contact == second_contact { continue }
					if newer_velocity_direction.dot(other_contact.0) < 0.0 {
						valid = false;
						break
					}
				}
				
				if valid {
					for contact in remainder {
						if newer_velocity_direction.dot(contact.0) < 0.0 {
							return (Vec3::ZERO, ConstraintSet::Triple)
						}
					}
					
					
					let newer_velocity = newer_velocity_direction * velocity.dot(newer_velocity_direction) / newer_velocity_direction.length_squared();
					let total_delta_v = newer_velocity - velocity;
					
					// todo: break up delta v for each contact
					
					return (
						newer_velocity,
						ConstraintSet::Double(Constraint {
							normal: first_contact.0,
							material_properties: first_contact.1,
							delta_v: 0.0,
						}, Constraint {
							normal: second_contact.0,
							material_properties: second_contact.1,
							delta_v: 0.0,
						})
					)
				}
			}
			
			return (Vec3::ZERO, ConstraintSet::Triple)
		}
	}
	
	for first_contact in &opposing {
		for second_contact in opposing.iter().rev() {
			if first_contact == second_contact { continue }
			
			let mut newer_velocity_direction = first_contact.0.cross(second_contact.0);
			if velocity.dot(newer_velocity_direction) < 0.0 {
				newer_velocity_direction = -newer_velocity_direction;
			}
			
			let mut valid = true;
			for other_contact in &opposing {
				if other_contact == first_contact || other_contact == second_contact { continue }
				if newer_velocity_direction.dot(other_contact.0) < 0.0 {
					valid = false;
					break
				}
			}
			
			if valid {
				for contact in remainder {
					if newer_velocity_direction.dot(contact.0) < 0.0 {
						return (Vec3::ZERO, ConstraintSet::Triple)
					}
				}
				
				
				let newer_velocity = newer_velocity_direction * velocity.dot(newer_velocity_direction) / newer_velocity_direction.length_squared();
				let total_delta_v = newer_velocity - velocity;
				
				return (
					newer_velocity,
					ConstraintSet::Double(Constraint {
						normal: first_contact.0,
						material_properties: first_contact.1,
						delta_v: 0.0,
					}, Constraint {
						normal: second_contact.0,
						material_properties: second_contact.1,
						delta_v: 0.0,
					})
				)
			}
		}
	}
	
	(Vec3::ZERO, ConstraintSet::Triple)
}

