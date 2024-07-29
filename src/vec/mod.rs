use std::ops::{Add, Div, Mul, Sub};

mod vec2;
mod vec3;

pub use vec2::Vec2;
pub use vec3::Vec3;


pub trait Sqrt { fn sqrt(self) -> Self; }
impl Sqrt for f32 { fn sqrt(self) -> Self { f32::sqrt(self) } }
impl Sqrt for f64 { fn sqrt(self) -> Self { f64::sqrt(self) } }


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


pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;




