use crate::*;

pub fn force_from_inputs(entity: &Entity) -> Vec3<f64> {
	
	entity.movement_input * 3000.0
	
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
}