use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

use glium::{uniforms::{AsUniformValue, UniformValue}, vertex::{Attribute, AttributeType}, Vertex, VertexFormat};
use num_traits::{AsPrimitive, ConstOne, ConstZero, Float, Zero};

use super::{Axis::{self, *}, Modulo, Vec3};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> where T: ConstZero + ConstOne {
	pub const X: Self = Self(T::ONE, T::ZERO);
	pub const Y: Self = Self(T::ZERO, T::ONE);
}

impl<T> Vec2<T> where {
	pub const fn x(self) -> T where T: Copy { self.0 }
	pub const fn y(self) -> T where T: Copy { self.1 }
	
	pub const fn all(c: T) -> Self
	where
		T: Copy
	{ Self(c, c) }
	
	pub const fn unit(axis: Axis) -> Self
	where
		T: ConstZero + ConstOne
	{
		match axis {
			X => Self::X,
			Y => Self::Y,
			Z => Self::ZERO,
		}
	}
	
	pub fn by_axis<F: FnMut(Axis) -> T>(mut f: F) -> Self {
		Self(f(X), f(Y))
	}
	
	pub fn component(self, axis: Axis) -> Self
	where
		T: ConstZero
	{
		match axis {
			X => Self(self.0, T::ZERO),
			Y => Self(T::ZERO, self.1),
			Z => Self(T::ZERO, T::ZERO),
		}
	}
	
	pub fn with_x(self, v: T) -> Self { Self(v, self.1) }
	pub fn with_y(self, v: T) -> Self { Self(self.0, v) }
	pub fn with(self, axis: Axis, v: T) -> Self {
		match axis {
			X => self.with_x(v),
			Y => self.with_y(v),
			Z => self,
		}
	}
	
	pub fn add_x<U>(self, v: U) -> Self where T: Add<U, Output = T> { Self(self.0 + v, self.1) }
	pub fn add_y<U>(self, v: U) -> Self where T: Add<U, Output = T> { Self(self.0, self.1 + v) }
	// pub fn add<U>(self, axis: Axis, v: U) -> Self where T: Add<U, Output = T> {
	// 	match axis {
	// 		X => self.add_x(v),
	// 		Y => self.add_y(v),
	// 		Z => self,
	// 	}
	// }
	
	pub fn map<U, F>(self, mut f: F) -> Vec2<U> where F: FnMut(T) -> U {
		Vec2(f(self.0), f(self.1))
	}
	
	pub fn dot<U>(self, v: Vec2<U>) -> <<T as Mul<U>>::Output as Add>::Output
	where
		T: Mul<U>,
		<T as Mul<U>>::Output: Add
	{ self.0 * v.0 + self.1 * v.1 }
	
	pub fn length_squared(self) -> <<T as Mul>::Output as Add>::Output
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add
	{ self.0*self.0 + self.1*self.1 }
	
	pub fn length_as<U>(self) -> U
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		U: Float + From<<<T as Mul>::Output as Add>::Output>
	{ <U as From<<<T as Mul>::Output as Add>::Output>>::from(self.length_squared()).sqrt() }
	
	pub fn normalize_as<U>(self) -> Vec2<<T as Div<U>>::Output>
	where
		T: Copy + Mul + Mul<U, Output = T> + Div<U>,
		<T as Mul>::Output: Add,
		U: Float + From<<<T as Mul>::Output as Add>::Output>
	{ let f = self.length_as::<U>(); Vec2(self.0/f, self.1/f) }
	
	pub fn cross<U>(self, v: Vec2<U>) -> <<T as Mul<U>>::Output as Sub>::Output
	where
		T: Mul<U>,
		<T as Mul<U>>::Output: Sub
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

impl<T> Vec2<T> where T: Float {
	pub fn length(self) -> T { self.length_as::<T>() }
	pub fn normalize(self) -> Self { self.normalize_as::<T>() }
	pub fn normalize_or_zero(self) -> Self { if self.is_zero() { Self::zero() } else { self.normalize() } }
	pub fn floor(self) -> Self { Self(self.0.floor(), self.1.floor()) }
	pub fn ceil(self) -> Self { Self(self.0.ceil(), self.1.ceil()) }
	
	pub fn floor_to<U>(self) -> Vec2<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ self.floor().as_type::<U>() }
	
	pub fn ceil_to<U>(self) -> Vec2<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ self.ceil().as_type::<U>() }
}

impl<T> Zero for Vec2<T> where
	T: Zero
{
	fn zero() -> Self { Self(T::zero(), T::zero()) }
	fn is_zero(&self) -> bool { self.0.is_zero() && self.1.is_zero() }
	fn set_zero(&mut self) { *self = Self::zero() }
}

impl<T> ConstZero for Vec2<T> where
	T: ConstZero
{ const ZERO: Self = Self(T::ZERO, T::ZERO); }


impl<T, U> Modulo<U> for Vec2<T> where
	T: Modulo<U>,
	U: Copy
{ fn modulo(self, rhs: U) -> Self { Vec2(self.0.modulo(rhs), self.1.modulo(rhs)) } }



unsafe impl<T> Attribute for Vec2<T> where
	(T, T): Attribute
{
	fn get_type() -> AttributeType {
		<(T, T) as Attribute>::get_type()
	}
}

impl<T> Vertex for Vec2<T> where
	Self: Attribute,
	T: Copy
{
	fn build_bindings() -> VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <Self as Attribute>::get_type(), false)])
	}
}

impl AsUniformValue for Vec2<f32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::Vec2([self.0, self.1]) } }
impl AsUniformValue for Vec2<f64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::DoubleVec2([self.0, self.1]) } }
impl AsUniformValue for Vec2<i32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::IntVec2([self.0, self.1]) } }
impl AsUniformValue for Vec2<i64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::Int64Vec2([self.0, self.1]) } }
impl AsUniformValue for Vec2<u32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::UnsignedIntVec2([self.0, self.1]) } }
impl AsUniformValue for Vec2<u64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::UnsignedInt64Vec2([self.0, self.1]) } }



// Operator overloading

impl<T> Index<Axis> for Vec2<T> {
	type Output = T;
	fn index(&self, index: Axis) -> &Self::Output {
		match index {
			X => &self.0,
			Y => &self.1,
			Z => panic!("No Z axis for Vec2")
		}
	}
}

impl<T> IndexMut<Axis> for Vec2<T> {
	fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
		match index {
			X => &mut self.0,
			Y => &mut self.1,
			Z => panic!("No Z axis for Vec2")
		}
	}
}

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


