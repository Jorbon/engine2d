use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

use glium::{uniforms::{AsUniformValue, UniformValue}, vertex::{Attribute, AttributeType}, VertexFormat};
use num_traits::{AsPrimitive, ConstOne, ConstZero, Float, Signed, Zero};

use super::{Axis::{self, *}, Direction::{self, *}, Modulo, Vec2};

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Debug> Debug for Vec3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("Vec3( {:?}, {:?}, {:?} )", &self.0, &self.1, &self.2))
	}
}

impl<T> ConstZero for Vec3<T> where
	T: ConstZero
{ const ZERO: Self = Self(T::ZERO, T::ZERO, T::ZERO); }

impl<T> Vec3<T> where
	T: ConstZero
{ pub const ZERO: Self = <Self as ConstZero>::ZERO; }

impl<T> Vec3<T> where T: ConstZero + ConstOne {
	pub const X: Self = Self(T::ONE, T::ZERO, T::ZERO);
	pub const Y: Self = Self(T::ZERO, T::ONE, T::ZERO);
	pub const Z: Self = Self(T::ZERO, T::ZERO, T::ONE);
	pub const XY: Self = Self(T::ONE, T::ONE, T::ZERO);
	pub const XZ: Self = Self(T::ONE, T::ZERO, T::ONE);
	pub const YZ: Self = Self(T::ZERO, T::ONE, T::ONE);
	pub const XYZ: Self = Self(T::ONE, T::ONE, T::ONE);
}

impl<T> Vec3<T> {
	pub const fn x(self) -> T where T: Copy { self.0 }
	pub const fn y(self) -> T where T: Copy { self.1 }
	pub const fn z(self) -> T where T: Copy { self.2 }
	pub const fn xy(self) -> Vec2<T> where T: Copy { Vec2(self.0, self.1) }
	pub const fn xz(self) -> Vec2<T> where T: Copy { Vec2(self.0, self.2) }
	pub const fn yz(self) -> Vec2<T> where T: Copy { Vec2(self.1, self.2) }
	pub const fn yx(self) -> Vec2<T> where T: Copy { Vec2(self.1, self.0) }
	pub const fn zx(self) -> Vec2<T> where T: Copy { Vec2(self.2, self.0) }
	pub const fn zy(self) -> Vec2<T> where T: Copy { Vec2(self.2, self.1) }
	
	pub fn get_plane(self, plane: Axis) -> Vec2<T> where T: Copy {
		match plane {
			Z => self.xy(),
			Y => self.xz(),
			X => self.yz(),
		}
	}
	
	pub const fn all(c: T) -> Self
	where
		T: Copy
	{ Self(c, c, c) }
	
	pub fn unit(direction: Direction) -> Self
	where
		T: ConstZero + ConstOne + Neg<Output = T>
	{
		match direction {
			PX => Self::X,
			PY => Self::Y,
			PZ => Self::Z,
			NX => -Self::X,
			NY => -Self::Y,
			NZ => -Self::Z,
		}
	}
	
	pub const fn positive_unit(axis: Axis) -> Self
	where
		T: ConstZero + ConstOne
	{
		match axis {
			X => Self::X,
			Y => Self::Y,
			Z => Self::Z,
		}
	}
	
	// pub const fn positive_unit_pair(axis1: Axis, axis2: Axis) -> Self
	// where
	// 	T: ConstZero + ConstOne
	// {
	// 	match (axis1, axis2) {
	// 		(X, Y) | (Y, X) => Self::XY,
	// 		(X, Z) | (Z, X) => Self::XZ,
	// 		(Y, Z) | (Z, Y) => Self::YZ,
	// 		(X, X) | (Y, Y) | (Z, Z) => panic!("Duplicate axis passed to positive_unit_pair")
	// 	}
	// }
	
	pub fn by_axis<F: FnMut(Axis) -> T>(mut f: F) -> Self {
		Self(f(X), f(Y), f(Z))
	}
	
	pub const fn component(self, axis: Axis) -> Self
	where
		T: Copy + ConstZero
	{
		match axis {
			X => Self(self.0, T::ZERO, T::ZERO),
			Y => Self(T::ZERO, self.1, T::ZERO),
			Z => Self(T::ZERO, T::ZERO, self.2),
		}
	}
	
	pub fn with_x(self, v: T) -> Self { Self(v, self.1, self.2) }
	pub fn with_y(self, v: T) -> Self { Self(self.0, v, self.2) }
	pub fn with_z(self, v: T) -> Self { Self(self.0, self.1, v) }
	pub fn with(self, axis: Axis, v: T) -> Self {
		match axis {
			X => self.with_x(v),
			Y => self.with_y(v),
			Z => self.with_z(v),
		}
	}
	
	pub fn with_xy(self, v: Vec2<T>) -> Self { Self(v.0, v.1, self.2) }
	pub fn with_xz(self, v: Vec2<T>) -> Self { Self(v.0, v.1, self.2) }
	pub fn with_yz(self, v: Vec2<T>) -> Self { Self(v.0, v.1, self.2) }
	pub fn with_plane(self, plane: Axis, v: Vec2<T>) -> Self {
		match plane {
			Z => self.with_xy(v),
			Y => self.with_xz(v),
			X => self.with_yz(v),
		}
	}
	
	pub fn add_x<U>(self, v: U) -> Self where T: Add<U, Output = T> { Self(self.0 + v, self.1, self.2) }
	pub fn add_y<U>(self, v: U) -> Self where T: Add<U, Output = T> { Self(self.0, self.1 + v, self.2) }
	pub fn add_z<U>(self, v: U) -> Self where T: Add<U, Output = T> { Self(self.0, self.1, self.2 + v) }
	// pub fn add<U>(self, axis: Axis, v: U) -> Self where T: Add<U, Output = T> {
	// 	match axis {
	// 		X => self.add_x(v),
	// 		Y => self.add_y(v),
	// 		Z => self.add_z(v),
	// 	}
	// }
	
	pub fn map<U, F>(self, mut f: F) -> Vec3<U> where F: FnMut(T) -> U {
		Vec3(f(self.0), f(self.1), f(self.2))
	}
	
	pub fn dot<U>(self, v: Vec3<U>) -> <<<T as Mul<U>>::Output as Add>::Output as Add<<T as Mul<U>>::Output>>::Output
	where
		T: Copy + Mul<U>,
		U: Copy,
		<T as Mul<U>>::Output: Add,
		<<T as Mul<U>>::Output as Add>::Output: Add<<T as Mul<U>>::Output>
	{ self.0 * v.0 + self.1 * v.1 + self.2 * v.2 }
	
	pub fn length_squared(self) -> <<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>
	{ self.0*self.0 + self.1*self.1 + self.2*self.2 }
	
	pub fn length_as<U>(self) -> U
	where
		T: Copy + Mul,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>,
		U: Float + From<<<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output>
	{ <U as From<<<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output>>::from(self.length_squared()).sqrt() }
	
	pub fn normalize_as<U>(self) -> Vec3<<T as Div<U>>::Output>
	where
		T: Copy + Mul + Mul<U, Output = T> + Div<U>,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Add<<T as Mul>::Output>,
		U: Float + From<<<<T as Mul>::Output as Add>::Output as Add<<T as Mul>::Output>>::Output>
	{ let f = self.length_as::<U>(); Vec3(self.0/f, self.1/f, self.2/f) }
	
	pub fn cross<U>(self, v: Vec3<U>) -> Vec3<<<T as Mul<U>>::Output as Sub>::Output>
	where
		T: Copy + Mul<U>,
		<T as Mul<U>>::Output: Sub,
		U: Copy
	{ Vec3(self.1 * v.2 - self.2 * v.1, self.2 * v.0 - self.0 * v.2, self.0 * v.1 - self.1 * v.0) }
	
	pub fn scale<U>(self, v: Vec3<U>) -> Vec3<<T as Mul<U>>::Output>
	where
		T: Mul<U>
	{ Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2) }
	
	pub fn scale_divide<U>(self, v: Vec3<U>) -> Vec3<<T as Div<U>>::Output>
	where
		T: Div<U>
	{ Vec3(self.0 / v.0, self.1 / v.1, self.2 / v.2) }
	
	pub fn abs(self) -> Self
	where
		T: Signed
	{ Self(self.0.abs(), self.1.abs(), self.2.abs() ) }
	
	pub fn as_type<U>(self) -> Vec3<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ Vec3(self.0.as_(), self.1.as_(), self.2.as_()) }
	
	pub fn from_type<U>(self) -> Vec3<U>
	where
		T: Into<U> + Debug + Copy
	{ Vec3(self.0.into(), self.1.into(), self.2.into()) }
}



impl<T> Vec3<T> where T: Float {
	pub fn length(self) -> T { self.length_as::<T>() }
	pub fn normalize(self) -> Self { self.normalize_as::<T>() }
	pub fn normalize_or_zero(self) -> Self { if self.is_zero() { Self::zero() } else { self.normalize() } }
	pub fn floor(self) -> Self { Self(self.0.floor(), self.1.floor(), self.2.floor()) }
	pub fn ceil(self) -> Self { Self(self.0.ceil(), self.1.ceil(), self.2.ceil()) }
	pub fn round(self) -> Self { Self(self.0.round(), self.1.round(), self.2.round()) }
	
	pub fn floor_to<U>(self) -> Vec3<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ self.floor().as_type::<U>() }
	
	pub fn ceil_to<U>(self) -> Vec3<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ self.ceil().as_type::<U>() }
	
	pub fn round_to<U>(self) -> Vec3<U>
	where
		T: Copy + 'static + AsPrimitive<U>,
		U: Copy + 'static
	{ self.round().as_type::<U>() }
}

impl<T> Zero for Vec3<T> where
	T: Zero
{
	fn zero() -> Self { Self(T::zero(), T::zero(), T::zero()) }
	fn is_zero(&self) -> bool { self.0.is_zero() && self.1.is_zero() && self.2.is_zero() }
	fn set_zero(&mut self) { *self = Self::zero() }
}

impl<T, U> Modulo<U> for Vec3<T> where
	T: Modulo<U>,
	U: Copy
{ fn modulo(self, rhs: U) -> Self { Vec3(self.0.modulo(rhs), self.1.modulo(rhs), self.2.modulo(rhs)) } }



impl<T> Vec3<Vec3<T>> {
	pub fn matmul<U>(self, rhs: Vec3<Vec3<U>>) -> Vec3<Vec3<<T as Mul<U>>::Output>>
	where
		T: Copy + Mul<U>,
		U: Copy,
		<T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>
	{
		Vec3(
			Vec3(
				self.0.0 * rhs.0.0 + self.1.0 * rhs.0.1 + self.2.0 * rhs.0.2,
				self.0.1 * rhs.0.0 + self.1.1 * rhs.0.1 + self.2.1 * rhs.0.2,
				self.0.2 * rhs.0.0 + self.1.2 * rhs.0.1 + self.2.2 * rhs.0.2,
			), Vec3(
				self.0.0 * rhs.1.0 + self.1.0 * rhs.1.1 + self.2.0 * rhs.1.2,
				self.0.1 * rhs.1.0 + self.1.1 * rhs.1.1 + self.2.1 * rhs.1.2,
				self.0.2 * rhs.1.0 + self.1.2 * rhs.1.1 + self.2.2 * rhs.1.2,
			), Vec3(
				self.0.0 * rhs.2.0 + self.1.0 * rhs.2.1 + self.2.0 * rhs.2.2,
				self.0.1 * rhs.2.0 + self.1.1 * rhs.2.1 + self.2.1 * rhs.2.2,
				self.0.2 * rhs.2.0 + self.1.2 * rhs.2.1 + self.2.2 * rhs.2.2,
			)
		)
	}
}



unsafe impl<T> Attribute for Vec3<T> where
	(T, T, T): Attribute
{
	fn get_type() -> AttributeType {
		<(T, T, T) as Attribute>::get_type()
	}
}

impl<T> glium::Vertex for Vec3<T> where
	Self: Attribute,
	T: Copy
{
	fn build_bindings() -> VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <Self as Attribute>::get_type(), false)])
	}
}

impl AsUniformValue for Vec3<f32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::Vec3([self.0, self.1, self.2]) } }
impl AsUniformValue for Vec3<f64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::DoubleVec3([self.0, self.1, self.2]) } }
impl AsUniformValue for Vec3<i32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::IntVec3([self.0, self.1, self.2]) } }
impl AsUniformValue for Vec3<i64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::Int64Vec3([self.0, self.1, self.2]) } }
impl AsUniformValue for Vec3<u32> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::UnsignedIntVec3([self.0, self.1, self.2]) } }
impl AsUniformValue for Vec3<u64> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::UnsignedInt64Vec3([self.0, self.1, self.2]) } }

impl AsUniformValue for Vec3<Vec3<f32>> { fn as_uniform_value(&self) -> UniformValue<'_> { UniformValue::Mat3([[self.0.0, self.0.1, self.0.2], [self.1.0, self.1.1, self.1.2], [self.2.0, self.2.1, self.2.2]]) } }



// Operator overloading

impl<T> Index<Axis> for Vec3<T> {
	type Output = T;
	fn index(&self, index: Axis) -> &Self::Output {
		match index {
			X => &self.0,
			Y => &self.1,
			Z => &self.2,
		}
	}
}

impl<T> IndexMut<Axis> for Vec3<T> {
	fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
		match index {
			X => &mut self.0,
			Y => &mut self.1,
			Z => &mut self.2,
		}
	}
}

impl<T, U> Add<Vec3<U>> for Vec3<T> where
	T: Add<U>
{
	type Output = Vec3<<T as Add<U>>::Output>;
	fn add(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
}
impl<T, U> AddAssign<U> for Vec3<T> where
	Vec3<T>: Copy + Add<U, Output = Vec3<T>>
{ fn add_assign(&mut self, rhs: U) { *self = *self + rhs; } }

impl<T, U> Sub<Vec3<U>> for Vec3<T> where
	T: Sub<U>
{
	type Output = Vec3<<T as Sub<U>>::Output>;
	fn sub(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
}
impl<T, U> SubAssign<U> for Vec3<T> where
	Vec3<T>: Copy + Sub<U, Output = Vec3<T>>
{ fn sub_assign(&mut self, rhs: U) { *self = *self - rhs; } }

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



impl<T, U> Rem<Vec3<U>> for Vec3<T> where
	T: Rem<U>
{
	type Output = Vec3<<T as Rem<U>>::Output>;
	fn rem(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2) }
}
impl<T, U> RemAssign<U> for Vec3<T> where
	Vec3<T>: Copy + Rem<U, Output = Vec3<T>>
{ fn rem_assign(&mut self, rhs: U) { *self = *self % rhs } }

impl<T, U> Shl<Vec3<U>> for Vec3<T> where
	T: Shl<U>,
	U: Copy
{
	type Output = Vec3<<T as Shl<U>>::Output>;
	fn shl(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 << rhs.0, self.1 << rhs.1, self.2 << rhs.2) }
}
impl<T, U> ShlAssign<U> for Vec3<T> where
	Vec3<T>: Copy + Shl<U, Output = Vec3<T>>,
	U: Copy
{ fn shl_assign(&mut self, rhs: U) { *self = *self << rhs } }

impl<T, U> Shr<Vec3<U>> for Vec3<T> where
	T: Shr<U>,
	U: Copy
{
	type Output = Vec3<<T as Shr<U>>::Output>;
	fn shr(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 >> rhs.0, self.1 >> rhs.1, self.2 >> rhs.2) }
}
impl<T, U> ShrAssign<U> for Vec3<T> where
	Vec3<T>: Copy + Shr<U, Output = Vec3<T>>,
	U: Copy
{ fn shr_assign(&mut self, rhs: U) { *self = *self >> rhs } }

impl<T, U> BitAnd<Vec3<U>> for Vec3<T> where
	T: BitAnd<U>
{
	type Output = Vec3<<T as BitAnd<U>>::Output>;
	fn bitand(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 & rhs.0, self.1 & rhs.1, self.2 & rhs.2) }
}
impl<T, U> BitAndAssign<U> for Vec3<T> where
	Vec3<T>: Copy + BitAnd<U, Output = Vec3<T>>
{ fn bitand_assign(&mut self, rhs: U) { *self = *self & rhs } }

impl<T, U> BitOr<Vec3<U>> for Vec3<T> where
	T: BitOr<U>
{
	type Output = Vec3<<T as BitOr<U>>::Output>;
	fn bitor(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 | rhs.0, self.1 | rhs.1, self.2 | rhs.2) }
}
impl<T, U> BitOrAssign<U> for Vec3<T> where
	Vec3<T>: Copy + BitOr<U, Output = Vec3<T>>
{ fn bitor_assign(&mut self, rhs: U) { *self = *self | rhs } }

impl<T, U> BitXor<Vec3<U>> for Vec3<T> where
	T: BitXor<U>
{
	type Output = Vec3<<T as BitXor<U>>::Output>;
	fn bitxor(self, rhs: Vec3<U>) -> Self::Output { Vec3(self.0 ^ rhs.0, self.1 ^ rhs.1, self.2 ^ rhs.2) }
}
impl<T, U> BitXorAssign<U> for Vec3<T> where
	Vec3<T>: Copy + BitXor<U, Output = Vec3<T>>
{ fn bitxor_assign(&mut self, rhs: U) { *self = *self ^ rhs } }

