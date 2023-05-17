//! A wrapper for [libm](libm).
//!
//!
pub use libm::fabs;
pub use libm::fabsf;

pub use libm::sqrt;
pub use libm::sqrtf;

pub use libm::pow;
pub use libm::powf;

pub use libm::floor;
pub use libm::floorf;
pub use libm::ceil;
pub use libm::ceilf;
pub use libm::round;
pub use libm::roundf;

pub use libm::cos;
pub use libm::cosf;
pub use libm::sin;
pub use libm::sinf;
pub use libm::acos;
pub use libm::acosf;
pub use libm::asin;
pub use libm::asinf;
pub use libm::atan2;
pub use libm::atan2f;

pub use libm::copysign;
pub use libm::copysignf;


pub trait Maths
{
	fn cos   ( self ) -> Self;
	fn sin   ( self ) -> Self;
	fn acos  ( self ) -> Self;
	fn asin  ( self ) -> Self;
	fn atan2 ( self, div: Self ) -> Self;
	
	fn abs   ( self ) -> Self;
	fn ceil  ( self ) -> Self;
	fn floor ( self ) -> Self;
	fn round ( self ) -> Self;

	fn sqrt  ( self ) -> Self;
	fn powf  ( self, exp: Self ) -> Self;
	
	fn fract     ( self ) -> Self;
	fn copysign ( self, sign: Self ) -> Self;
	
}

#[cfg(not(test))]
impl Maths for f32
{
	fn cos   ( self ) -> Self { return libm::cosf  (self); }
	fn sin   ( self ) -> Self { return libm::sinf  (self); }
	fn acos  ( self ) -> Self { return libm::acosf (self); }
	fn asin  ( self ) -> Self { return libm::asinf (self); }
	fn atan2 ( self, div: Self ) -> Self { return libm::atan2f (self, div); }

	fn abs   ( self ) -> Self { return libm::fabsf (self); }
	fn ceil  ( self ) -> Self { return libm::ceilf (self); }
	fn floor ( self ) -> Self { return libm::floorf(self); }
	fn round ( self ) -> Self { return libm::roundf(self); }
	
	fn sqrt  ( self ) -> Self { return libm::sqrtf (self); }
	fn powf  ( self, exp: Self ) -> Self { return libm::powf  (self, exp); }
	
	fn fract  ( self ) -> Self { return self - self.floor(); }
	fn copysign ( self, sign: Self ) -> Self { return libm::copysignf (self, sign); }
}

#[cfg(not(test))]
impl Maths for f64
{
	fn cos   ( self ) -> Self { return libm::cos  (self); }
	fn sin   ( self ) -> Self { return libm::sin  (self); }
	fn acos  ( self ) -> Self { return libm::acos (self); }
	fn asin  ( self ) -> Self { return libm::asin (self); }
	fn atan2 ( self, div: Self ) -> Self { return libm::atan2  (self, div); }

	fn abs   ( self ) -> Self { return libm::fabs (self); }
	fn ceil  ( self ) -> Self { return libm::ceil (self); }
	fn floor ( self ) -> Self { return libm::floor(self); }
	fn round ( self ) -> Self { return libm::round(self); }
	
	fn sqrt  ( self ) -> Self { return libm::sqrt (self); }
	fn powf  ( self, exp: Self ) -> Self { return libm::pow (self, exp); }
	
	fn fract  ( self ) -> Self  { return self - self.floor(); }
	fn copysign ( self, sign: Self ) -> Self { return libm::copysign (self, sign); }
}
