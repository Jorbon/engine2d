use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

use super::{Modulo, Sqrt, Vec2};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> Vec3<T> where {
	pub fn x(self) -> T { self.0 }
	pub fn y(self) -> T { self.1 }
	pub fn z(self) -> T { self.2 }
	pub fn xy(self) -> Vec2<T> { Vec2(self.0, self.1) }
	pub fn xz(self) -> Vec2<T> { Vec2(self.0, self.2) }
	pub fn yz(self) -> Vec2<T> { Vec2(self.1, self.2) }
	pub fn yx(self) -> Vec2<T> { Vec2(self.1, self.0) }
	pub fn zx(self) -> Vec2<T> { Vec2(self.2, self.0) }
	pub fn zy(self) -> Vec2<T> { Vec2(self.2, self.1) }
	
	pub fn all(c: T) -> Self
	where
		T: Copy
	{ Self(c, c, c) }
	
	pub fn dot<U>(&self, v: Vec3<U>) -> <<<T as Mul<U>>::Output as Add>::Output as Add<<T as Mul<U>>::Output>>::Output
	where
		T: Copy + Mul<U>,
		<T as Mul<U>>::Output: Add,
		<<T as Mul<U>>::Output as Add>::Output: Add<<T as Mul<U>>::Output>
	{ self.0 * v.0 + self.1 * v.1 + self.2 * v.2 }
	
	pub fn length_squared(&self) -> <<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>
	{ self.0*self.0 + self.1*self.1 + self.2*self.2 }
	
	pub fn length_as<U>(&self) -> U
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>,
		U: Sqrt + From<<<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output>
	{ U::from(self.length_squared()).sqrt() }
	
	pub fn normalize_as<U>(&self) -> Vec3<<T as Div<U>>::Output>
	where
		T: Copy + Mul + Mul<U, Output = T> + Div<U>,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>,
		U: Copy + Sqrt + From<<<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output>
	{ let f = self.length_as::<U>(); Vec3(self.0/f, self.1/f, self.2/f) }
	
	pub fn cross<U>(&self, v: &Vec3<U>) -> Vec3<<<T as Mul<U>>::Output as Sub>::Output>
	where
		T: Copy + Mul<U>,
		<T as Mul<U>>::Output: Sub,
		U: Copy
	{ Vec3(self.1 * v.2 - self.2 * v.1, self.2 * v.0 - self.0 * v.2, self.0 * v.1 - self.1 * v.0) }
	
	pub fn scale<U>(self, v: Vec3<U>) -> Vec3<<T as Mul<U>>::Output>
	where
		T: Mul<U>
	{ Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2) }
	
	pub fn is_zero(self) -> bool
	where
		T: Default + PartialEq
	{ self.0 == T::default() && self.1 == T::default() && self.2 == T::default() }
}

impl Vec3<f32> {
	pub fn length(&self) -> f32 { self.length_as::<f32>() }
	pub fn normalize(&self) -> Vec3<f32> { self.normalize_as::<f32>() }
}

impl Vec3<f64> {
	pub fn length(&self) -> f64 { self.length_as::<f64>() }
	pub fn normalize(&self) -> Vec3<f64> { self.normalize_as::<f64>() }
}



impl<T, U> Add<Vec3<U>> for Vec3<T> where
	T: Add<U>
{
	type Output = Vec3<<T as Add<U>>::Output>;
	fn add(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
}

impl<T, U> AddAssign<Vec3<U>> for Vec3<T> where
	T: Copy + Add<U, Output = T>
{ fn add_assign(&mut self, rhs: Vec3<U>) { *self = *self + rhs; } }

impl<T, U> Sub<Vec3<U>> for Vec3<T> where
	T: Sub<U>
{
	type Output = Vec3<<T as Sub<U>>::Output>;
	fn sub(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
}

impl<T, U> SubAssign<Vec3<U>> for Vec3<T> where
	T: Copy + Sub<U, Output = T>
{ fn sub_assign(&mut self, rhs: Vec3<U>) { *self = *self - rhs; } }

impl<T> Neg for Vec3<T> where
	T: Neg
{
	type Output = Vec3<<T as Neg>::Output>;
	fn neg(self) -> Self::Output { Vec3(-self.0, -self.1, -self.2) }
}

impl<T, U> Mul<U> for Vec3<T> where
	T: Mul<U>,
	U: Copy
{
	type Output = Vec3<<T as Mul<U>>::Output>;
	fn mul(self, rhs: U) -> Self::Output { Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs) }
}

impl<T, U> MulAssign<U> for Vec3<T> where
	T: Copy + Mul<U, Output = T>,
	U: Copy
{ fn mul_assign(&mut self, rhs: U) { *self = *self * rhs; } }

impl<T, U> Div<U> for Vec3<T> where
	T: Div<U>,
	U: Copy
{
	type Output = Vec3<<T as Div<U>>::Output>;
	fn div(self, rhs: U) -> Self::Output { Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs) }
}

impl<T, U> DivAssign<U> for Vec3<T> where
	T: Copy + Div<U, Output = T>,
	U: Copy
{ fn div_assign(&mut self, rhs: U) { *self = *self / rhs; } }

impl<T, U> Rem<U> for Vec3<T> where
	T: Rem<U>,
	U: Copy
{
	type Output = Vec3<<T as Rem<U>>::Output>;
	fn rem(self, rhs: U) -> Self::Output { Vec3(self.0 % rhs, self.1 % rhs, self.2 % rhs) }
}

impl<T, U> Shl<U> for Vec3<T> where
	T: Shl<U>,
	U: Copy
{
	type Output = Vec3<<T as Shl<U>>::Output>;
	fn shl(self, rhs: U) -> Self::Output { Vec3(self.0 << rhs, self.1 << rhs, self.2 << rhs) }
}

impl<T, U> ShlAssign<U> for Vec3<T> where
	T: Copy + Shl<U, Output = T>,
	U: Copy
{ fn shl_assign(&mut self, rhs: U) { *self = *self << rhs } }

impl<T, U> Shr<U> for Vec3<T> where
	T: Shr<U>,
	U: Copy
{
	type Output = Vec3<<T as Shr<U>>::Output>;
	fn shr(self, rhs: U) -> Self::Output { Vec3(self.0 >> rhs, self.1 >> rhs, self.2 >> rhs) }
}

impl<T, U> ShrAssign<U> for Vec3<T> where
	T: Copy + Shr<U, Output = T>,
	U: Copy
{ fn shr_assign(&mut self, rhs: U) { *self = *self >> rhs } }

impl<T, U> Modulo<U> for Vec3<T> where
	T: Modulo<U>,
	U: Copy
{ fn modulo(self, rhs: U) -> Self { Vec3(self.0.modulo(rhs), self.1.modulo(rhs), self.2.modulo(rhs)) } }



impl<T> glium::Vertex for Vec3<T> where
	T: Copy,
	(T, T, T): glium::vertex::Attribute
{
	fn build_bindings() -> glium::VertexFormat {
		use glium::vertex::Attribute;
		&[(std::borrow::Cow::Borrowed("position"), 0, -1, <(T, T, T)>::TYPE, false)]
	}
}

impl glium::uniforms::AsUniformValue for Vec3<f32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Vec3([self.0, self.1, self.2]) } }
impl glium::uniforms::AsUniformValue for Vec3<f64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::DoubleVec3([self.0, self.1, self.2]) } }
impl glium::uniforms::AsUniformValue for Vec3<i32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::IntVec3([self.0, self.1, self.2]) } }
impl glium::uniforms::AsUniformValue for Vec3<i64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Int64Vec3([self.0, self.1, self.2]) } }
impl glium::uniforms::AsUniformValue for Vec3<u32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedIntVec3([self.0, self.1, self.2]) } }
impl glium::uniforms::AsUniformValue for Vec3<u64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedInt64Vec3([self.0, self.1, self.2]) } }

