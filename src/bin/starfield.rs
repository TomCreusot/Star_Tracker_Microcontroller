extern crate star_tracker;

use star_tracker::nix::Star;
use star_tracker::nix::NixImage;
use star_tracker::image_processing::Image;
use star_tracker::util::aliases::Decimal;
use star_tracker::util::units::Pixel;
use star_tracker::util::units::AngleAxis;
use star_tracker::util::units::Equatorial;
use star_tracker::util::units::Cartesian3D;
use star_tracker::util::units::Radians;
use star_tracker::util::units::Degrees;
use star_tracker::util::units::Matrix;
use star_tracker::util::units::MatPos;
use star_tracker::util::units::PixelWeighted;

use star_tracker::nix::Io;
use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;
// use star_tracker::config::TrackingModeConstructConstsStruct;
// use star_tracker::config::TrackingModeConstructConsts;
// use star_tracker::config::starfield;

// Input:
// * fov
// * intens_min_mag (brightest)
// * intens_max_mag (dullest)
// * min_mag
// * intens_max_mag
// * noise_max
// * chance_noise
//
// * rotation (quaternion)
fn main ( )
{
	let fov        : Radians     = Degrees(90.0).to_radians();
	let cutoff_mag : Decimal     = 1.5;
	let dir        : Cartesian3D = Equatorial{ra: Degrees(00.0).to_radians(), dec: Degrees(00.0).to_radians()}.to_cartesian3();
	let angle_axis : AngleAxis   = AngleAxis::look_at(dir, Cartesian3D{x: 0.0, y:0.0, z:1.0});

	let img_size = Pixel{x: 1000, y: 1000};
	let principle_point = PixelWeighted{x: img_size.x as Decimal / 2.0, y: img_size.y as Decimal / 2.0};
    let mut img = NixImage::new(img_size);


	let camera_transform : Matrix<4,4> = angle_axis.to_matrix();
	let mut projection   : Matrix<4,4> = Matrix::identity();
	let focal_length     : Decimal     = 900.0;
	let r                : Decimal     = -1.0 / focal_length;

	projection.set(MatPos{row: 2, col: 2}, 0.0);
	projection.set(MatPos{row: 3, col: 2}, r);

	let mut rdr = Io::get_csv(NixConstsStruct::HYG_DATABASE_PATH, NixConstsStruct::HYG_DATABASE_URL);
	let iter = rdr.deserialize();


	let extrinsic = CameraExtrinsicParameters::look_at(dir);
	let intrinsic = CameraIntrinsicParameters::new(fov, 1.0, img_size, principle_point);
	let transform = CameraTransformation::new(intrinsic, extrinsic);

	let mut i = 0;
    for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		let point = star.pos.to_cartesian3() * 10000.0;
        if star.mag < cutoff_mag && 0.0 < point.dot(&dir)// && star_point.angle_distance(dir) < fov
        {
			// point = angle_axis.to_quaternion().rotate_point(point);
			// let x = (point.x / 1.0) * 500.0 + 500.0;
			// let y = (point.y / 1.0) * 500.0 + 500.0;
			// let image_frame = PixelWeighted{x: x, y: y};
			let image_frame  : PixelWeighted = transform.apply(point);
			let px        : Pixel = Pixel { x: (image_frame.x + 500.0) as usize, y: (image_frame.y + 500.0) as usize};
			if (&img as &dyn Image).valid_pixel(px)
			{
				let mut color = [0, 0, 255];
				if ( star.mag < cutoff_mag / 5.0 )		{	color = [255, 0, 0];	}
				else if ( star.mag < cutoff_mag / 4.0 )	{	color = [255, 125, 0];	}
				else if ( star.mag < cutoff_mag / 3.0 )	{	color = [50, 255, 50];	}
				else if ( star.mag < cutoff_mag / 2.0 )	{	color = [0, 255, 255];	}
				else									{	color = [0, 0, 255];	}
				img.draw_points(px, ((cutoff_mag - star.mag) as u32 + 2) * 3, color);
			}
			
			i+=1;
			println!("{} \t\t\t{:?}, {}", i, image_frame, star.mag);
        }
	}
	
	img.img_rgb.save("results/star_field.png").expect("Could not save.");



}













pub struct CameraTransformation
{
	/// The transformation which converts a point in 3d to a 2d point on the sensor.
	pub transformation: Matrix<4,4>,
}

impl CameraTransformation
{
	pub fn new ( intrinsic: CameraIntrinsicParameters, extrinsic: CameraExtrinsicParameters )->Self
	{
		println!("\n\nINTRINSIC\n{}", intrinsic.to_image());
		println!("\n\nEXTRINSIC\n{}", extrinsic.transformation);
		println!("\n\nCOMBINED\n{}", intrinsic.to_image() * extrinsic.transformation);
		return Self{transformation: intrinsic.to_image() * extrinsic.transformation};
	}
	
	pub fn apply ( &self, point: Cartesian3D ) -> PixelWeighted
	{
		let point_mat : Matrix<4,1>   = point.to_matrix_column_homo();
		println!("{}", point_mat);
		let out_mat   : Matrix<4,1>   = self.transformation * point_mat;
		println!("{}", out_mat);
		let out_3d    : Cartesian3D   = out_mat.to_cartesian3();
		let out_px    : PixelWeighted = PixelWeighted{x:out_3d.x as Decimal, y:out_3d.y as Decimal};
		return out_px;
	}
}



pub struct CameraExtrinsicParameters
{
	pub transformation: Matrix<4, 4>,
	// pub position : Cartesian3D,
	// pub rotation : 
}

impl CameraExtrinsicParameters
{
	pub fn new ( ) -> Self
	{
		let mut identity : Matrix<4,4> = Matrix::identity();
		return Self{transformation:identity};
	}
	
	pub fn look_at ( point: Cartesian3D ) -> Self
	{
		let orientation = AngleAxis::look_at(point, Cartesian3D{x:0.0, y:0.0, z:1.0});
		
		let identity : Matrix<4,4> = Matrix::identity();
		let mut w : Matrix<4,4> = Matrix::new();
		w.set(MatPos{row: 1, col: 0}, -orientation.axis.z);
		w.set(MatPos{row: 2, col: 0}, orientation.axis.y);
		
		w.set(MatPos{row: 0, col: 1}, orientation.axis.z);
		w.set(MatPos{row: 2, col: 1}, -orientation.axis.x);
		
		w.set(MatPos{row: 0, col: 2}, -orientation.axis.y);
		w.set(MatPos{row: 1, col: 2}, orientation.axis.x);
		
		// return CameraExtrinsicParameters
		let mat = identity + w*orientation.angle.0.sin() + w*w*(1.0-orientation.angle.0.cos());
		return Self{transformation: mat};
	}
}



/// Intrinsic are values defining how light passes through the pinhole and contacts the sensor.
/// It assumes that the sensor is at the origin so the extrinsic transformation must be applied.
pub struct CameraIntrinsicParameters
{
	/// Distance (in units) between image plane and projection center *camera_constant*.
	pub focal_length    : Decimal,
	
	/// The size (in units) of each pixel.
	pub pixel_scale     : PixelWeighted,
	
	/// The center point of the image.
	pub principle_point : PixelWeighted,
	
	/// How skewed the image is.
	pub shear           : Decimal,
}

impl CameraIntrinsicParameters
{
	///
	///
	///
	pub fn new ( fov : Radians, dist: Decimal, num_pixels : Pixel, principle_point: PixelWeighted ) -> Self
	{
		let mut mat : Matrix<4,4> = Matrix::identity();
		
		// let focal_length  = 1.0; // 2 unknowns can be solved by setting random value.
		let mut focal_length  = 0.1;
		let pixel_scale_x = (0.5 * fov.0).tan() * focal_length; // (num_pixels.x as Decimal);
		let pixel_scale_y = (0.5 * fov.0).tan() * focal_length; // (num_pixels.y as Decimal);
		// let pixel_scale   = PixelWeighted{x: pixel_scale_x, y: pixel_scale_y};
		let mut pixel_scale   = PixelWeighted{x: 500.0, y: 500.0};
		
		return CameraIntrinsicParameters{focal_length: focal_length, pixel_scale: pixel_scale, principle_point: principle_point, shear: 0.0};
	}
	
	

	
	/// Converts parameters to a transformation matrix which convert a 3d image to 2d.
	/// The camera must be at the origin with no rotation.
	pub fn to_image ( &self ) -> Matrix<4,4>
	{
/*		let mut mat : Matrix<3,3> = Matrix::identity();
		// Scale
		mat.set(MatPos{row: 0, col: 0}, self.focal_length / self.pixel_scale.x);
		mat.set(MatPos{row: 1, col: 1}, self.focal_length / self.pixel_scale.y);
		
		// Center Point
		mat.set(MatPos{row: 0, col: 2}, self.principle_point.x);
		mat.set(MatPos{row: 1, col: 2}, self.principle_point.y);
*/
		let mut mat_project : Matrix<4,4> = Matrix::new();
		mat_project.set(MatPos{row: 0, col: 0}, self.focal_length);
		mat_project.set(MatPos{row: 1, col: 1}, self.focal_length);
		mat_project.set(MatPos{row: 2, col: 2}, self.focal_length);
		mat_project.set(MatPos{row: 3, col: 2}, 1.0);
		
		let mut mat_translate : Matrix<4,4> = Matrix::identity();
		mat_translate.set(MatPos{row: 0, col: 3}, self.principle_point.x);
		mat_translate.set(MatPos{row: 1, col: 3}, self.principle_point.y);
		
		let mut mat_scale : Matrix<4,4> = Matrix::identity();
		mat_scale.set(MatPos{row: 0, col: 0}, self.pixel_scale.x);
		mat_scale.set(MatPos{row: 1, col: 1}, self.pixel_scale.y);
		
		let mut mat_shear : Matrix<4,4> = Matrix::identity();
		mat_shear.set(MatPos{row: 0, col: 1}, self.shear);
		
		return mat_project * mat_scale * mat_shear;// * mat_translate;
	}
	
	// pub fn apply ( &)
}