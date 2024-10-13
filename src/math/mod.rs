use std::{cmp::Ordering, marker::PhantomData, ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign}};
use num_traits::{ConstOne, PrimInt};

mod vec2;
mod vec3;
pub use vec2::*;
pub use vec3::*;

pub trait Modulo<T> { fn modulo(self, rhs: T) -> Self; }
impl<T> Modulo<T> for f32 where
	Self: Rem<T, Output = Self> + Add<T, Output = Self>,
	T: Copy
{ fn modulo(self, rhs: T) -> Self { ((self % rhs) + rhs) % rhs } }
impl<T> Modulo<T> for f64 where
	Self: Rem<T, Output = Self> + Add<T, Output = Self>,
	T: Copy
{ fn modulo(self, rhs: T) -> Self { ((self % rhs) + rhs) % rhs } }


pub fn lerp<T, U>(a: T, b: T, t: U) -> T
where
	T: Copy + Sub + Add<<<T as Sub>::Output as Mul<U>>::Output, Output = T>,
	<T as Sub>::Output: Mul<U>
{
	a + (b - a) * t
}

pub fn prel<T, U>(a: T, b: T, y: T) -> U
where
	T: Copy + Sub,
	<T as Sub>::Output: Div<Output = U>
{
	(y - a) / (b - a)
}


pub struct DirectionalRange<T> {
	start: T,
	end: T,
	current: Option<T>,
}

impl<T> DirectionalRange<T> where
	T: PartialOrd
{
	pub fn new(start: T, end: T) -> Self {
		Self {
			start,
			end,
			current: None,
		}
	}
}

impl<T> Iterator for DirectionalRange<T> where
	T: PrimInt
{
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.current = match self.current {
			None => Some(self.start),
			Some(position) => match position.cmp(&self.end) {
				Ordering::Less => Some(position + T::one()),
				Ordering::Greater => Some(position - T::one()),
				Ordering::Equal => None,
			}
		};
		self.current
	}
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Axis { X, Y, Z }
pub use Axis::*;
impl Axis {
	pub const fn l(self) -> Self { match self { X => Z, Y => X, Z => Y } }
	pub const fn r(self) -> Self { match self { X => Y, Y => Z, Z => X } }
	pub const fn p(self) -> Direction { match self { X => PX, Y => PY, Z => PZ } }
	pub const fn n(self) -> Direction { match self { X => NX, Y => NY, Z => NZ } }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction { PX, PY, PZ, NX, NY, NZ }
pub use Direction::*;
impl Direction {
	pub const fn axis(self) -> Axis { match self { PX | NX => X, PY | NY => Y, PZ | NZ => Z } }
	pub const fn is_positive(self) -> bool { match self { PX | PY | PZ => true, NX | NY | NZ => false } }
	pub const fn is_negative(self) -> bool { match self { NX | NY | NZ => true, PX | PY | PZ => false } }
}


pub trait AxisOrder { const AXIS: [Axis; 3]; }
pub enum XYZ {} impl AxisOrder for XYZ { const AXIS: [Axis; 3] = [X,Y,Z]; }
pub enum XZY {} impl AxisOrder for XZY { const AXIS: [Axis; 3] = [X,Z,Y]; }
pub enum YXZ {} impl AxisOrder for YXZ { const AXIS: [Axis; 3] = [Y,X,Z]; }
pub enum YZX {} impl AxisOrder for YZX { const AXIS: [Axis; 3] = [Y,Z,X]; }
pub enum ZXY {} impl AxisOrder for ZXY { const AXIS: [Axis; 3] = [Z,X,Y]; }
pub enum ZYX {} impl AxisOrder for ZYX { const AXIS: [Axis; 3] = [Z,Y,X]; }


pub struct Vec3Range<T, O> {
	start: Vec3<T>,
	end: Vec3<T>,
	current: Option<Vec3<T>>,
	_order: PhantomData<O>
}

impl<T, O: AxisOrder> Vec3Range<T, O> {
	pub fn inclusive(start: Vec3<T>, end: Vec3<T>) -> Self
	where
		T: PrimInt
	{
		Self {
			start,
			end,
			current: None,
			_order: PhantomData::<O>::default()
		}
	}
	pub fn exclusive(start: Vec3<T>, end: Vec3<T>) -> Self
	where
		T: PrimInt + ConstOne
	{
		Self {
			start,
			end: end - Vec3::all(T::ONE),
			current: None,
			_order: PhantomData::<O>::default()
		}
	}
}

impl<T, O: AxisOrder> Iterator for Vec3Range<T, O> where
	T: PrimInt + AddAssign + SubAssign
{
	type Item = Vec3<T>;
	fn next(&mut self) -> Option<Self::Item> {
		self.current = match self.current {
			None => Some(self.start),
			Some(mut position) => match position[O::AXIS[2]].cmp(&self.end[O::AXIS[2]]) {
				Ordering::Less => { position[O::AXIS[2]] += T::one(); Some(position) }
				Ordering::Greater => { position[O::AXIS[2]] -= T::one(); Some(position) }
				Ordering::Equal => {
					position[O::AXIS[2]] = self.start[O::AXIS[2]];
					match position[O::AXIS[1]].cmp(&self.end[O::AXIS[1]]) {
						Ordering::Less => { position[O::AXIS[1]] += T::one(); Some(position) }
						Ordering::Greater => { position[O::AXIS[1]] -= T::one(); Some(position) }
						Ordering::Equal => {
							position[O::AXIS[1]] = self.start[O::AXIS[1]];
							match position[O::AXIS[0]].cmp(&self.end[O::AXIS[0]]) {
								Ordering::Less => { position[O::AXIS[0]] += T::one(); Some(position) }
								Ordering::Greater => { position[O::AXIS[0]] -= T::one(); Some(position) }
								Ordering::Equal => None,
							}
						}
					}
				}
			}
		};
		self.current
	}
}


