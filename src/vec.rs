

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2(pub f32, pub f32);
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[allow(dead_code)]
impl Vec2 {
	#[inline] pub fn all(c: f32) -> Self { Self(c, c) }
	#[inline] pub fn length_squared(self) -> f32 { self.0*self.0 + self.1*self.1 }
	#[inline] pub fn length(self) -> f32 { self.length_squared().sqrt() }
	#[inline] pub fn normalize(self) -> Self { let f = 1.0 / self.length(); Self(self.0*f, self.1*f) }
	#[inline] pub fn dot(self, v: Self) -> f32 { self.0 * v.0 + self.1 * v.1 }
	#[inline] pub fn cross(self, v: Self) -> f32 { self.0 * v.1 - self.1 * v.0 }
	#[inline] pub fn scale(self, v: Self) -> Self { Self(self.0 * v.0, self.1 * v.1) }
	#[inline] pub fn is_zero(self) -> bool { self.0 == 0.0 && self.1 == 0.0 }
	#[inline] pub fn x(self) -> f32 { self.0 }
	#[inline] pub fn y(self) -> f32 { self.1 }
	pub const ZERO: Self = Self(0.0, 0.0);
}

#[allow(dead_code)]
impl Vec3 {
	#[inline] pub fn all(c: f32) -> Self { Self(c, c, c) }
	#[inline] pub fn length_squared(self) -> f32 { self.0*self.0 + self.1*self.1 + self.2*self.2 }
	#[inline] pub fn length(self) -> f32 { self.length_squared().sqrt() }
	#[inline] pub fn normalize(self) -> Self { let f = 1.0 / self.length(); Self(self.0*f, self.1*f, self.2*f) }
	#[inline] pub fn dot(self, v: Self) -> f32 { self.0 * v.0 + self.1 * v.1 + self.2 * v.2 }
	#[inline] pub fn cross(self, v: Self) -> Vec3 { Vec3(self.1 * v.2 - self.2 * v.1, self.2 * v.0 - self.0 * v.2, self.0 * v.1 - self.1 * v.0) }
	#[inline] pub fn scale(self, v: Self) -> Self { Self(self.0 * v.0, self.1 * v.1, self.2 * v.2) }
	#[inline] pub fn is_zero(self) -> bool { self.0 == 0.0 && self.1 == 0.0 && self.2 == 0.0 }
	#[inline] pub fn x(self) -> f32 { self.0 }
	#[inline] pub fn y(self) -> f32 { self.1 }
	#[inline] pub fn z(self) -> f32 { self.2 }
	#[inline] pub fn xy(self) -> Vec2 { Vec2(self.0, self.1) }
	#[inline] pub fn xz(self) -> Vec2 { Vec2(self.0, self.2) }
	#[inline] pub fn yz(self) -> Vec2 { Vec2(self.1, self.2) }
	#[inline] pub fn yx(self) -> Vec2 { Vec2(self.1, self.0) }
	#[inline] pub fn zx(self) -> Vec2 { Vec2(self.2, self.0) }
	#[inline] pub fn zy(self) -> Vec2 { Vec2(self.2, self.1) }
	pub const ZERO: Self = Self(0.0, 0.0, 0.0);
	// #[inline] pub fn apply_transform(self, m: &Mat4) -> Self { Self(
	// 	m.0[0][0]*self.0 + m.0[1][0]*self.1 + m.0[2][0]*self.2 + m.0[3][0],
	// 	m.0[0][1]*self.0 + m.0[1][1]*self.1 + m.0[2][1]*self.2 + m.0[3][1],
	// 	m.0[0][2]*self.0 + m.0[1][2]*self.1 + m.0[2][2]*self.2 + m.0[3][2],
	// ) }
}


impl std::ops::Add for Vec2 { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1) } }
impl std::ops::AddAssign for Vec2 { #[inline] fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Vec2 { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1) } }
impl std::ops::SubAssign for Vec2 { #[inline] fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Neg for Vec2 { type Output = Self; #[inline] fn neg(self) -> Self::Output { Self(-self.0, -self.1) } }
impl std::ops::Mul<f32> for Vec2 { type Output = Self; #[inline] fn mul(self, rhs: f32) -> Self::Output { Self(self.0 * rhs, self.1 * rhs) } }
impl std::ops::MulAssign<f32> for Vec2 { #[inline] fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; } }
impl std::ops::Mul<Vec2> for f32 { type Output = Vec2; #[inline] fn mul(self, rhs: Vec2) -> Self::Output { rhs * self } }
impl std::ops::Div<f32> for Vec2 { type Output = Self; #[inline] fn div(self, rhs: f32) -> Self::Output { self * (1.0/rhs) } }
impl std::ops::DivAssign<f32> for Vec2 { #[inline] fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; } }
impl std::ops::Div<Vec2> for f32 { type Output = Vec2; #[inline] fn div(self, rhs: Vec2) -> Self::Output { Vec2(self / rhs.0, self / rhs.1) } }
impl std::ops::Rem<Vec2> for Vec2 { type Output = Self; #[inline] fn rem(self, rhs: Self) -> Self::Output { Self(self.0 % rhs.0, self.1 % rhs.1) } }
impl std::ops::Rem<f32> for Vec2 { type Output = Self; #[inline] fn rem(self, rhs: f32) -> Self::Output { Self(self.0 % rhs, self.1 % rhs) } }

impl std::ops::Add for Vec3 { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) } }
impl std::ops::AddAssign for Vec3 { #[inline] fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Vec3 { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) } }
impl std::ops::SubAssign for Vec3 { #[inline] fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Neg for Vec3 { type Output = Self; #[inline] fn neg(self) -> Self::Output { Self(-self.0, -self.1, -self.2) } }
impl std::ops::Mul<f32> for Vec3 { type Output = Self; #[inline] fn mul(self, rhs: f32) -> Self::Output { Self(self.0 * rhs, self.1 * rhs, self.2 * rhs) } }
impl std::ops::MulAssign<f32> for Vec3 { #[inline] fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; } }
impl std::ops::Mul<Vec3> for f32 { type Output = Vec3; #[inline] fn mul(self, rhs: Vec3) -> Self::Output { rhs * self } }
impl std::ops::Div<f32> for Vec3 { type Output = Self; #[inline] fn div(self, rhs: f32) -> Self::Output { self * (1.0/rhs) } }
impl std::ops::DivAssign<f32> for Vec3 { #[inline] fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; } }
impl std::ops::Div<Vec3> for f32 { type Output = Vec3; #[inline] fn div(self, rhs: Vec3) -> Self::Output { Vec3(self / rhs.0, self / rhs.1, self / rhs.2) } }
impl std::ops::Rem<Vec3> for Vec3 { type Output = Self; #[inline] fn rem(self, rhs: Self) -> Self::Output { Self(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2) } }
impl std::ops::Rem<f32> for Vec3 { type Output = Self; #[inline] fn rem(self, rhs: f32) -> Self::Output { Self(self.0 % rhs, self.1 % rhs, self.2 % rhs) } }

impl glium::Vertex for Vec2 { fn build_bindings() -> glium::VertexFormat { &[
    (std::borrow::Cow::Borrowed("position"), 0, -1, glium::vertex::AttributeType::F32F32, false)
] } }
impl glium::Vertex for Vec3 { fn build_bindings() -> glium::VertexFormat { &[
    (std::borrow::Cow::Borrowed("position"), 0, -1, glium::vertex::AttributeType::F32F32F32, false)
] } }

impl glium::uniforms::AsUniformValue for Vec2 {
	fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
		glium::uniforms::UniformValue::Vec2([self.0, self.1])
	}
}
impl glium::uniforms::AsUniformValue for Vec3 {
	fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
		glium::uniforms::UniformValue::Vec3([self.0, self.1, self.2])
	}
}
