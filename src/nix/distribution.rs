//! Distributes points onto a sphere close to evenly.
//!
//!
//!

use rand::prelude::*;

use crate::nix::Distribution;
use crate::nix::Distribute;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::units::Vector3;


impl Distribute
{

	/// Sets the points of the input equatorial array as a set of **close** to evenly spaced points.
	/// This uses the fiboancci golden ratio algorithm.
	///
	/// # Arguments
	/// * `num_points` - The number of points on the sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::util::aliases::M_PI;
	/// use star_tracker::nix::Distribute;
	/// let points = Distribute::angle_to_points(Radians(10.0));
	/// let mut eq : Vec<Equatorial> = Distribute::fibonacci_latice(points);
	/// ```
	pub fn fibonacci_latice ( num_points : usize ) -> Vec<Equatorial>
	{
		let mut output : Vec<Equatorial> = Vec::with_capacity(num_points);
		let golden_ratio = (1.0 + (5.0 as Decimal).powf(0.5)) / 2.0;

		for i in 0..num_points
		{
			let mut theta = 2.0 * M_PI * (i as Decimal) / golden_ratio;
			let phi = (1.0 - 2.0 * (i as Decimal + 0.5) / num_points as Decimal).acos();

			theta = theta % (Equatorial::range_ra().end().0);

			let mut val = Equatorial{ra: Radians(theta), dec: Radians(0.0)};
			val.set_phi(Radians(phi));
			output.push(val);
		}
		return output
	}
	
	
	
	pub fn separation ( points: &Vec<Equatorial> ) -> Distribution
	{
		let mut dist = Distribution 
		{ 
			avg: Radians(0.0), 
			max: Radians(0.0), 
			min: Radians(Decimal::INFINITY),
			dev: Radians(0.0),  
		};
		
		let mut avg_count = 0.0;
		for ii in 0..points.len()
		{
			let mut closest = Radians(Decimal::INFINITY);
			for jj in 0..points.len()
			{
				let distance = points[ii].angle_distance(points[jj]);
				if ii != jj && distance < closest
				{
					closest = distance;
				}
			}
			// The average needs to be counted so that there is a constant accurate average for the deviation.
			avg_count += closest.0;
			dist.avg = Radians(avg_count) / (ii + 1) as Decimal;
			dist.dev = dist.dev + (closest - dist.avg) / points.len() as Decimal;
			dist.min = if dist.min < closest { dist.min } else { closest };
			dist.max = if dist.max > closest { dist.max } else { closest };
		}
		return dist;
	}
	
	/// Uses monti carlo
	pub fn coverage ( points: &Vec<Equatorial> ) -> Distribution
	{
		let mut rng = rand::thread_rng();
		let mut dist = Distribution 
		{ 
			avg: Radians(0.0), 
			max: Radians(0.0), 
			min: Radians(Decimal::INFINITY),
			dev: Radians(0.0),  
		};

		let iterations = 10000;

		// The mean will be found fast enough.
		for _ii in 0..iterations
		{
			let point_r = Vector3 {
				x: rng.gen_range(-1.0..1.0), 
				y: rng.gen_range(-1.0..1.0), 
				z: rng.gen_range(-1.0..1.0) };
		
			let mut closest = Radians(Decimal::INFINITY);
			for jj in 0..points.len()
			{
				let distance = point_r.angle_distance(points[jj].to_vector3());
				if distance < closest
				{
					closest = distance;
				}
			}
			
			dist.avg = dist.avg +  closest / iterations as Decimal;
			dist.dev = dist.dev + (closest - dist.avg) / iterations as Decimal;
			dist.min = if dist.min < closest { dist.min } else { closest };
			dist.max = if dist.max > closest { dist.max } else { closest };
		}
		return dist;
	}
	

	
	/// Finds the number of points required to acheive the given angle *With Error*.  
	/// This is used for the fibonacci golden sphere.  
	/// The error is:
	/// * 43deg < angle           : 0
	/// * 24.6deg < angle < 43deg : -1
	/// * angle < 24.6deg         : slowely increasing positive number.
	/// As the angle gets smaller, the number of points error becomes higher, however it is insignificant.  
	/// When the error is negative, the angle is larger than it should be.
	pub fn angle_to_points ( angle: Radians ) -> usize
	{
		if angle < Degrees(54.7).to_radians()
		{
			// Error margin of +-3% output.
			return (37282.8117 * angle.to_degrees().0.powf(-2.0031)).round() as usize;
		}
		// Error of +-1%
		// Works better with large angles (few points).
		return (16785.5187 * angle.0.powf(-1.8178)).round() as usize;
	}


	/// Finds the angle between each point given the the number of points to generate.   
	/// This uses the fibonacci golden sphere.   
	/// The error is:
	/// * points < 12: +-2%
	/// * 12 < points: +-2%
	pub fn points_to_angle ( points: usize ) -> Radians
	{
		if 12 < points 
		{
			// Error margin of +-2% output.
			return Degrees(191.3844 * (points as Decimal).powf(-0.4990)).to_radians();
		}
		// Error of +-2%
		return Degrees(210.8359 * (points as Decimal).powf(-0.5498)).to_radians();
		
	}
}



// 
// //###############################################################################################//
// //###############################################################################################//
// //
// //										Unit Tests
// //
// //###############################################################################################//
// //###############################################################################################//
// 
// #[cfg(test)]
// #[allow(unused_must_use)]
// mod test
// {
// 	use crate::nix::Distribution;
// 	use crate::nix::Distribute;
// 
// 	// use crate::util::units::Equatorial;
// 	// use crate::util::units::Radians;
// 	// use crate::util::units::Degrees;
// 
// 	#[test]
// 	fn i_have_not_written_any_tests_for_this ( )
// 	{
// 		panic!("Have you tried doing your job.");
// 	}
// }
