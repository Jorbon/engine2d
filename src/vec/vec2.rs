use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

use num_traits::{AsPrimitive, Zero};

use super::{Modulo, Sqrt, Vec3};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
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
	
	pub fn scale_divide<U>(self, v: Vec2<U>) -> Vec2<<T as Div<U>>::Output>
	where
		T: Div<U>
	{ Vec2(self.0 / v.0, self.1 / v.1) }
	
	pub fn vec3_xy(self) -> Vec3<T>
	where
		T: Default
	{ Vec3(self.0, self.1, T::default()) }
	
	pub fn as_type<U>(self) -> Vec2<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ Vec2(self.0.as_(), self.1.as_()) }
	
	pub fn from_type<U>(self) -> Vec2<U>
	where
		T: Into<U> + Debug + Copy
	{ Vec2(self.0.into(), self.1.into()) }
}

impl Vec2<f32> {
	pub fn length(&self) -> f32 { self.length_as::<f32>() }
	pub fn normalize(&self) -> Self { self.normalize_as::<f32>() }
	pub fn normalize_or_zero(&self) -> Self { if self.is_zero() { Self::zero() } else { self.normalize() } }
}

impl Vec2<f64> {
	pub fn length(&self) -> f64 { self.length_as::<f64>() }
	pub fn normalize(&self) -> Self { self.normalize_as::<f64>() }
	pub fn normalize_or_zero(&self) -> Self { if self.is_zero() { Self::zero() } else { self.normalize() } }
}

impl<T> Zero for Vec2<T> where
	T: Zero + PartialEq
{
	fn zero() -> Self { Self(T::zero(), T::zero()) }
	fn is_zero(&self) -> bool { self.0 == T::zero() && self.1 == T::zero() }
	fn set_zero(&mut self) { *self = Self::zero() }
}


impl<T, U> Modulo<U> for Vec2<T> where
	T: Modulo<U>,
	U: Copy
{ fn modulo(self, rhs: U) -> Self { Vec2(self.0.modulo(rhs), self.1.modulo(rhs)) } }




impl<T> glium::Vertex for Vec2<T> where
	T: Copy,
	(T, T): glium::vertex::Attribute
{
	fn build_bindings() -> glium::VertexFormat {
		use glium::vertex::Attribute;
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <(T, T)>::get_type(), false)])
	}
}

impl glium::uniforms::AsUniformValue for Vec2<f32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Vec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<f64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::DoubleVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<i32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::IntVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<i64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::Int64Vec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<u32> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedIntVec2([self.0, self.1]) } }
impl glium::uniforms::AsUniformValue for Vec2<u64> { fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> { glium::uniforms::UniformValue::UnsignedInt64Vec2([self.0, self.1]) } }



// Operator overloading

impl<T, U> Add<Vec2<U>> for Vec2<T> where
	T: Add<U>
{
	type Output = Vec2<<T as Add<U>>::Output>;
	fn add(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 + rhs.0, self.1 + rhs.1) }
}
impl<T, U> AddAssign<U> for Vec2<T> where
	Vec2<T>: Copy + Add<U, Output = Vec2<T>>
{ fn add_assign(&mut self, rhs: U) { *self = *self + rhs; } }

impl<T, U> Sub<Vec2<U>> for Vec2<T> where
	T: Sub<U>
{
	type Output = Vec2<<T as Sub<U>>::Output>;
	fn sub(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 - rhs.0, self.1 - rhs.1) }
}
impl<T, U> SubAssign<U> for Vec2<T> where
	Vec2<T>: Copy + Sub<U, Output = Vec2<T>>
{ fn sub_assign(&mut self, rhs: U) { *self = *self - rhs; } }

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



impl<T, U> Rem<Vec2<U>> for Vec2<T> where
	T: Rem<U>
{
	type Output = Vec2<<T as Rem<U>>::Output>;
	fn rem(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 % rhs.0, self.1 % rhs.1) }
}
impl<T, U> RemAssign<U> for Vec2<T> where
	Vec2<T>: Copy + Rem<U, Output = Vec2<T>>
{ fn rem_assign(&mut self, rhs: U) { *self = *self % rhs } }

impl<T, U> Shl<Vec2<U>> for Vec2<T> where
	T: Shl<U>,
	U: Copy
{
	type Output = Vec2<<T as Shl<U>>::Output>;
	fn shl(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 << rhs.0, self.1 << rhs.1) }
}
impl<T, U> ShlAssign<U> for Vec2<T> where
	Vec2<T>: Copy + Shl<U, Output = Vec2<T>>,
	U: Copy
{ fn shl_assign(&mut self, rhs: U) { *self = *self << rhs } }

impl<T, U> Shr<Vec2<U>> for Vec2<T> where
	T: Shr<U>,
	U: Copy
{
	type Output = Vec2<<T as Shr<U>>::Output>;
	fn shr(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 >> rhs.0, self.1 >> rhs.1) }
}
impl<T, U> ShrAssign<U> for Vec2<T> where
	Vec2<T>: Copy + Shr<U, Output = Vec2<T>>,
	U: Copy
{ fn shr_assign(&mut self, rhs: U) { *self = *self >> rhs } }

impl<T, U> BitAnd<Vec2<U>> for Vec2<T> where
	T: BitAnd<U>
{
	type Output = Vec2<<T as BitAnd<U>>::Output>;
	fn bitand(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 & rhs.0, self.1 & rhs.1) }
}
impl<T, U> BitAndAssign<U> for Vec2<T> where
	Vec2<T>: Copy + BitAnd<U, Output = Vec2<T>>
{ fn bitand_assign(&mut self, rhs: U) { *self = *self & rhs } }

impl<T, U> BitOr<Vec2<U>> for Vec2<T> where
	T: BitOr<U>
{
	type Output = Vec2<<T as BitOr<U>>::Output>;
	fn bitor(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 | rhs.0, self.1 | rhs.1) }
}
impl<T, U> BitOrAssign<U> for Vec2<T> where
	Vec2<T>: Copy + BitOr<U, Output = Vec2<T>>
{ fn bitor_assign(&mut self, rhs: U) { *self = *self | rhs } }

impl<T, U> BitXor<Vec2<U>> for Vec2<T> where
	T: BitXor<U>
{
	type Output = Vec2<<T as BitXor<U>>::Output>;
	fn bitxor(self, rhs: Vec2<U>) -> Self::Output { Vec2(self.0 ^ rhs.0, self.1 ^ rhs.1) }
}
impl<T, U> BitXorAssign<U> for Vec2<T> where
	Vec2<T>: Copy + BitXor<U, Output = Vec2<T>>
{ fn bitxor_assign(&mut self, rhs: U) { *self = *self ^ rhs } }


