use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use super::{Sqrt, Vec3};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> where {
	pub fn x(self) -> T { self.0 }
	pub fn y(self) -> T { self.1 }
	
	pub fn all(c: T) -> Self
	where
		T: Copy
	{ Self(c, c) }
	
	pub fn dot<U>(&self, v: Vec2<U>) -> <<T as Mul<U>>::Output as Add>::Output
	where
		T: Copy + Mul<U>,
		<T as Mul<U>>::Output: Add
	{ self.0 * v.0 + self.1 * v.1 }
	
	pub fn length_squared(&self) -> <<T as Mul>::Output as Add>::Output
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add
	{ self.0*self.0 + self.1*self.1 }
	
	pub fn length_as<U>(&self) -> U
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		U: Sqrt + From<<<T as Mul>::Output as Add>::Output>
	{ U::from(self.length_squared()).sqrt() }
	
	pub fn normalize_as<U>(&self) -> Vec2<<T as Div<U>>::Output>
	where
		T: Copy + Mul + Mul<U, Output = T> + Div<U>,
		<T as Mul>::Output: Add,
		U: Copy + Sqrt + From<<<T as Mul>::Output as Add>::Output>
	{ let f = self.length_as::<U>(); Vec2(self.0/f, self.1/f) }
	
	pub fn cross<U>(&self, v: &Vec2<U>) -> <<T as Mul<U>>::Output as Sub>::Output
	where
		T: Copy + Mul<U>,
		<T as Mul<U>>::Output: Sub,
		U: Copy
	{ self.0 * v.1 - self.1 * v.0 }
	
	pub fn scale<U>(self, v: Vec2<U>) -> Vec2<<T as Mul<U>>::Output>
	where
		T: Mul<U>
	{ Vec2(self.0 * v.0, self.1 * v.1) }
	
	pub fn is_zero(self) -> bool
	where
		T: Default + PartialEq
	{ self.0 == T::default() && self.1 == T::default() }
	
	pub fn vec3_xy(self) -> Vec3<T>
	where
		T: Default
	{ Vec3(self.0, self.1, T::default()) }
}

impl Vec2<f32> {
	pub fn length(&self) -> f32 { self.length_as::<f32>() }
	pub fn normalize(&self) -> Self { self.normalize_as::<f32>() }
}

impl Vec2<f64> {
	pub fn length(&self) -> f64 { self.length_as::<f64>() }
	pub fn normalize(&self) -> Self { self.normalize_as::<f64>() }
}



impl<T, U> Add<Vec2<U>> for Vec2<T> where
	T: Add<U>
{
	type Output = Vec2<<T as Add<U>>::Output>;
	fn add(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 + rhs.0, self.1 + rhs.1) }
}

impl<T, U> AddAssign<Vec2<U>> for Vec2<T> where
	T: Copy + Add<U, Output = T>
{ fn add_assign(&mut self, rhs: Vec2<U>) { *self = *self + rhs; } }

impl<T, U> Sub<Vec2<U>> for Vec2<T> where
	T: Sub<U>
{
	type Output = Vec2<<T as Sub<U>>::Output>;
	fn sub(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 - rhs.0, self.1 - rhs.1) }
}

impl<T, U> SubAssign<Vec2<U>> for Vec2<T> where
	T: Copy + Sub<U, Output = T>
{ fn sub_assign(&mut self, rhs: Vec2<U>) { *self = *self - rhs; } }

impl<T> Neg for Vec2<T> where
	T: Neg
{
	type Output = Vec2<<T as Neg>::Output>;
	fn neg(self) -> Self::Output { Vec2(-self.0, -self.1) }
}

impl<T, U> Mul<U> for Vec2<T> where
	T: Mul<U>,
	U: Copy
{
	type Output = Vec2<<T as Mul<U>>::Output>;
	fn mul(self, rhs: U) -> Self::Output { Vec2(self.0 * rhs, self.1 * rhs) }
}

impl<T, U> MulAssign<U> for Vec2<T> where
	T: Copy + Mul<U, Output = T>,
	U: Copy
{ fn mul_assign(&mut self, rhs: U) { *self = *self * rhs; } }

impl<T, U> Div<U> for Vec2<T> where
	T: Div<U>,
	U: Copy
{
	type Output = Vec2<<T as Div<U>>::Output>;
	fn div(self, rhs: U) -> Self::Output { Vec2(self.0 / rhs, self.1 / rhs) }
}

impl<T, U> DivAssign<U> for Vec2<T> where
	T: Copy + Div<U, Output = T>,
	U: Copy
{ fn div_assign(&mut self, rhs: U) { *self = *self / rhs; } }

impl<T, U> Rem<U> for Vec2<T> where
	T: Rem<U>,
	U: Copy
{
	type Output = Vec2<<T as Rem<U>>::Output>;
	fn rem(self, rhs: U) -> Self::Output { Vec2(self.0 % rhs, self.1 % rhs) }
}



impl<T> glium::Vertex for Vec2<T> where
	T: Copy,
	(T, T): glium::vertex::Attribute
{
	fn build_bindings() -> glium::VertexFormat {
		use glium::vertex::Attribute;
		&[(std::borrow::Cow::Borrowed("position"), 0, -1, <(T, T)>::TYPE, false)]
	}
}

impl glium::uniforms::AsUniformValue for Vec2<f32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Vec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<f64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::DoubleVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<i32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::IntVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<i64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Int64Vec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<u32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedIntVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<u64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedInt64Vec2([self.0, self.1]) } }

