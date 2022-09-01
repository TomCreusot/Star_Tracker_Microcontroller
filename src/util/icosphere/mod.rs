//! Creates a set of equaly spaced points on a unit sphere.
//! This creates an icosohedron and subdividing it until the resolution is reached.
//! This can be useful for simulation, construction of the database and varifying the database.
// use crate::util::list::ArrayList;
// use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::units::Vector3;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;


/// The struct in charge of creating, testing and navigating an icosphere.
pub struct IcoSphere ( pub Vec<Equatorial> );

pub struct Face ( pub usize, pub usize, pub usize );



impl IcoSphere
{
	/// Creates an icosphere.
	/// # Arguments
	/// * `subdivisions` - Every subdivision adds 3 points to each face of the previous polyhedron.
	pub fn icosphere ( subdivisions: usize ) -> Vec<Equatorial>
	{
		let phi = (1.0 + (5.0 as Decimal).powf(0.5)) / 2.0; // Golden Ratio

		let mut points : Vec<Equatorial> = Vec::new();
		points.push(Vector3{x: -1.0, y:  phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x:  1.0, y:  phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x: -1.0, y: -phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x:  1.0, y: -phi, z: 0.0}.to_equatorial());

		points.push(Vector3{x:  0.0, y: -1.0, z:  phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y:  1.0, z:  phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y: -1.0, z: -phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y:  1.0, z: -phi}.to_equatorial());

		points.push(Vector3{x:  phi, y:  0.0, z: -1.0}.to_equatorial());
		points.push(Vector3{x:  phi, y:  0.0, z:  1.0}.to_equatorial());
		points.push(Vector3{x: -phi, y:  0.0, z: -1.0}.to_equatorial());
		points.push(Vector3{x: -phi, y:  0.0, z:  1.0}.to_equatorial());

		IcoSphere::subdivide_face(&mut points, subdivisions, Face(0, 11, 5 ));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(0, 5,  1 ));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(0, 1,  7 ));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(0, 7,  10));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(0, 10, 11));

		IcoSphere::subdivide_face(&mut points, subdivisions, Face(1,  5,  9));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(5,  11, 4));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(11, 10, 2));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(10, 7,  6));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(7,  1,  8));

		IcoSphere::subdivide_face(&mut points, subdivisions, Face(3, 9, 4));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(3, 4, 2));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(3, 2, 6));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(3, 6, 8));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(3, 8, 9));

		IcoSphere::subdivide_face(&mut points, subdivisions, Face(4, 9, 5 ));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(2, 4, 11));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(6, 2, 10));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(8, 6, 7 ));
		IcoSphere::subdivide_face(&mut points, subdivisions, Face(9, 8, 1 ));

		for ii in 0..points.len()
		{
			let mut jj = ii+1;
			while jj < points.len()
			{
				if points[ii].angle_distance(points[jj]) < Radians(0.0001)
				{
					points.remove(jj);
				}
				else
				{
					jj += 1;
				}
			}
		}

		return points;
	}

	/// RECURSIVE.
	/// Doubles the amount of points on a selected face.
	/// # Arguments
	/// * `list`       - The list of points which `face` references.
	/// * `iterations` - How many subdivisions (0 means dont subdivide).
	/// * `face`       - 3 points making a face which should be subdivided.
	pub fn subdivide_face ( list: &mut Vec<Equatorial>, iterations: usize, face: Face )
	{
		if iterations > 0
		{
			let pt_a = list[face.0].to_vector3();
			let pt_b = list[face.1].to_vector3();
			let pt_c = list[face.2].to_vector3();

			let pt_d = (pt_a + pt_b).to_equatorial();
			let pt_e = (pt_a + pt_c).to_equatorial();
			let pt_f = (pt_b + pt_c).to_equatorial();


			let idx_d = list.len();
			list.push(pt_d);
			let idx_e = list.len();
			list.push(pt_e);
			let idx_f = list.len();
			list.push(pt_f);

			let face_a = Face(face.0, idx_d, idx_e);
			let face_b = Face(face.1, idx_d, idx_f);
			let face_c = Face(face.2, idx_e, idx_f);
			let face_d = Face(idx_d, idx_e, idx_f);

			IcoSphere::subdivide_face(list, iterations - 1, face_a);
			IcoSphere::subdivide_face(list, iterations - 1, face_b);
			IcoSphere::subdivide_face(list, iterations - 1, face_c);
			IcoSphere::subdivide_face(list, iterations - 1, face_d);
		}
	}


	pub fn angle_min ( subdivisions : usize ) -> Radians
	{
		return Degrees(63.43495 / 2.0_f32.powf(subdivisions as f32) as Decimal).to_radians();
	}

	pub fn angle_max ( subdivisions : usize ) -> Radians
	{
		return Degrees(63.43495 / 2.0_f32.powf(subdivisions as f32) as Decimal).to_radians();
	}
}



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//

#[cfg(test)]
#[allow(unused_must_use)]
mod test
{
	use crate::util::icosphere::IcoSphere;
	use crate::util::list::List;
	use crate::util::aliases::Decimal;
	use crate::util::units::Radians;
	use crate::util::units::Equatorial;

	// List: The list to look at.
	// Above: Exclude anything below this value.
	fn find_angle ( list : &Vec<Equatorial>, above: Radians ) -> (usize, Radians)
	{
		let tolerance_angle = Radians(0.00001);
		let mut angle = Radians(Decimal::MAX);
		let mut num = 0;

		for pt in 0..list.size()
		{
			let point = list[pt];
			let mut smallest_angle = false;
			for adjacent in 0..list.len()
			{
				let distance = point.angle_distance(list[adjacent]);
				if pt != adjacent && above < distance
				{
					if distance < angle - tolerance_angle  // Angle was incorrect.
					{
						angle = distance;
						num = 0;
						smallest_angle = true;
					}
					else if distance < angle + tolerance_angle
					{
						smallest_angle = true;
					}
				}
			}
			if smallest_angle
			{
				num += 1;
			}
		}
		return (num, angle)
	}

	#[test]
	fn test_icosphere_angle_0_sub ( )
	{
		let icosohedron = IcoSphere::icosphere(0);
		let min_angle = find_angle(&icosohedron, Radians(-100.0));
		let avg_angle = find_angle(&icosohedron, min_angle.1 + Radians(0.001));
		println!("{} \t {}", min_angle.0, min_angle.1.to_degrees());
		println!("{} \t {}", avg_angle.0, avg_angle.1.to_degrees());
		panic!("");
	}


	#[test]
	fn test_icosphere_angle_1_sub ( )
	{
		let icosphere = IcoSphere::icosphere(1);
		let len = icosphere.len();
		let min_angle = find_angle(&icosphere, Radians(-100.0));
		let avg_angle = find_angle(&icosphere, min_angle.1 + Radians(0.001));
		println!("{} \t {}", min_angle.0, min_angle.1.to_degrees());
		println!("{} \t {}", avg_angle.0, avg_angle.1.to_degrees());
		panic!("");
	}

	#[test]
	fn test_icosphere_angle_2_sub ( )
	{
		let icosphere = IcoSphere::icosphere(2);
		let len = icosphere.len();
		let min_angle = find_angle(&icosphere, Radians(-100.0));
		let avg_angle = find_angle(&icosphere, min_angle.1 + Radians(0.001));
		println!("{} \t {}", min_angle.0, min_angle.1.to_degrees());
		println!("{} \t {}", avg_angle.0, avg_angle.1.to_degrees());
		panic!("");
	}
/*

	#[test]
	fn test_icosphere_max_angle_0_sub ( )
	{
		let icosohedron = IcoSphere::icosphere(0);
		let val = significant_angle(icosohedron, false);
		println!("{} \t {}", val.0, val.1.to_degrees());
		panic!("");
	}


	#[test]
	fn test_icosphere_max_angle_1_sub ( )
	{
		let icosohedron = IcoSphere::icosphere(1);
		let val = significant_angle(icosohedron, false);
		println!("{} \t {}", val.0, val.1.to_degrees());
		panic!("");
	}

	#[test]
	fn test_icosphere_max_angle_2_sub ( )
	{
		let icosohedron = IcoSphere::icosphere(2);
		let val = significant_angle(icosohedron, false);
		println!("{} \t {}", val.0, val.1.to_degrees());
		panic!("");
	}*/




}
