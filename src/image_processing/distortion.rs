//! This is the implementation of Distorition.
use super::Distortion;

use crate::util::units::PixelWeighted;
use crate::util::units::Cartesian3D;
// use crate::util::units::Equatorial;
use crate::util::units::Matrix;
use crate::util::units::MatPos;


impl Distortion
{
	/*pub image_to_local_equatorial ( pt: Cartesian2D ) -> Equatorial
	{

	}*/
	
	
	pub fn equatorial_to_local_image ( pt_center: Cartesian3D, pt: Cartesian3D ) -> PixelWeighted
	{
		let mut pt_mat : Matrix<1, 4> = Matrix::new();
		pt_mat.set(MatPos{row: 0, col: 0}, pt.x);
		pt_mat.set(MatPos{row: 0, col: 1}, pt.y);
		pt_mat.set(MatPos{row: 0, col: 2}, pt.z);
		pt_mat.set(MatPos{row: 0, col: 3}, 1.0);
		
		
		// rotate to plane x, z
		// | 1		0		0		0 |
		// | 0		c_z/d	c_y/d	0 |
		// | 0		-c_y/d	c_z/d	0 |
		// | 0		0		0		1 |
		// where:
		// c_y is the vector projected on the y axis.
		// c_z is the vector projected on the x axis.
		// d is \sqrt(c_y^2 + c_z^2)
		
		let axis_x = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let axis_y = Cartesian3D{x: 0.0, y: 1.0, z: 0.0};
		let axis_z = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let c_y = pt_center.dot(&axis_y) / axis_y.magnitude();
		let c_z = pt_center.dot(&axis_z) / axis_z.magnitude();
		let d = (c_y * c_y + c_z * c_z).sqrt();
		
		let mut rot_yz : Matrix<4,4> = Matrix::new();
		rot_yz.set(MatPos{row: 0, col: 0}, 1.0);
		rot_yz.set(MatPos{row: 1, col: 1}, c_z / d);
		rot_yz.set(MatPos{row: 1, col: 2}, c_y / d);
		rot_yz.set(MatPos{row: 2, col: 1}, -c_y / d);
		rot_yz.set(MatPos{row: 2, col: 2}, c_z / d);
		rot_yz.set(MatPos{row: 3, col: 3}, 1.0);
		
		// project to axis z
		// | c_z	0		c_x		0 |
		// | 0		1		0		0 |
		// | -c_x	0		c_z		0 |
		// | 0		0		0		1 |
		// where:
		// c_z = vector projected onto z axis.
		// c_x = vector projected on x axis.
		let c_x = pt_center.dot(&axis_x) / axis_x.magnitude();
		let mut rot_z : Matrix<4,4> = Matrix::new();
		rot_z.set(MatPos{row: 0, col: 2}, c_z);
		rot_z.set(MatPos{row: 1, col: 0}, c_x);
		rot_z.set(MatPos{row: 1, col: 1}, 1.0);
		rot_z.set(MatPos{row: 2, col: 0}, -c_x);
		rot_z.set(MatPos{row: 2, col: 2}, c_z);
		rot_z.set(MatPos{row: 3, col: 3}, 1.0);
		
		let rot = rot_yz * rot_z;
		let out = pt_mat * rot;
		
		//
		// let mut project : Matrix<4,4> = Matrix::new();
		
		
		return PixelWeighted{x: out.get(MatPos{row:0,col:0}), y: out.get(MatPos{row:0,col:1})};
	}
	
	
}