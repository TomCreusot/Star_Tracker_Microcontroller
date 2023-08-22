#![feature(let_chains)]
extern crate opencv;
extern crate star_tracker_nix;
extern crate star_tracker_lib;

use opencv::core::*;
use opencv::imgcodecs::*;
use opencv::calib3d::*;
use opencv::prelude::MatTrait;
use opencv::core::Point2f;
use opencv::core::Mat;
use opencv::imgcodecs::imread;
use opencv::imgproc::{self, get_perspective_transform, warp_perspective};
use opencv::highgui::imshow;
use opencv::highgui::wait_key;



use star_tracker_nix::io::Sample;

use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::Blob;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Vector3;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::projection::ExtrinsicParameters;
use star_tracker_lib::projection::SpaceWorld;


fn main ( ) -> opencv::Result<()>
{
	let samples = Sample::load_samples();

	// Loop through all the folders.
	for sample in samples
	{
		let cor = sample.get_corr();
		let fov = sample.get_fov();
		let center = sample.get_center();
		let size_img_px = sample.get_image_size();
		println!("{:?} {:?} {:?}", fov, center ,size_img_px);
		if let Some(cor) = cor && let Some(fov_diag) = fov && let Some(size_img) = size_img_px && let Some(center) = center
		{
			// Read Corr file
			println!("file: {}", sample.dir);

			let mut points_image: Vector<Vector<Point2f>> = Vector::new();
			let mut points_real : Vector<Vector<Point3f>> = Vector::new();

			let mut vec_img : Vector<Point2f> = Vector::new();
			let mut vec_rea : Vector<Point3f> = Vector::new();
			let projection_extrinsic = ExtrinsicParameters::look_at(center, Equatorial::north()).unwrap();

			let error_old = 0.0;
			for i in 0..cor.len()
			{
				let forward = Vector3{x: 0.0, y: 0.0, z: 1.0};
				let real = SpaceWorld(cor[i].real_eq.to_vector3());
				let real = projection_extrinsic.to_image(real).0;
				let mut real_z = Point3f::new((real.x / real.z) as f32, (real.y / real.z) as f32, 1.0);

				vec_img.push(Point2f::new(cor[i].image_px.x as f32, cor[i].image_px.y as f32));
				vec_rea.push(real_z);
			}
			points_image.push(vec_img.clone());
			points_real.push(vec_rea.clone());


			// Apply distortion to image.
			for img_dir in sample.file_img
			{
				let mut img_org = imread( &img_dir, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
				let mut img_cor = imread( &img_dir, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
				let mut img_un  = imread( &img_dir, ImreadModes::IMREAD_COLOR as i32 ).unwrap();

				// Create matrices to hold the camera matrix and distortion coefficients
			    let mut camera_matrix = unsafe
				{
					Mat::new_nd_vec(&Vector::from_slice(&[3, 3]), CV_32F)?
				};
				*camera_matrix.at_2d_mut::<f32>(0, 0)? = 1.0;
				*camera_matrix.at_2d_mut::<f32>(0, 1)? = 0.0;
				*camera_matrix.at_2d_mut::<f32>(0, 2)? = 0.0;

				*camera_matrix.at_2d_mut::<f32>(1, 0)? = 0.0;
				*camera_matrix.at_2d_mut::<f32>(1, 1)? = 1.0;
				*camera_matrix.at_2d_mut::<f32>(1, 2)? = 0.0;

				*camera_matrix.at_2d_mut::<f32>(2, 0)? = 0.0;
				*camera_matrix.at_2d_mut::<f32>(2, 1)? = 0.0;
				*camera_matrix.at_2d_mut::<f32>(2, 2)? = 1.0;

			    let mut dist_coeffs = Mat::default();

				let size = Size::new(img_org.cols() as i32, img_org.rows() as i32);

				let criteria = TermCriteria::new(TermCriteria_Type::COUNT as i32 +TermCriteria_Type::EPS as i32, 30, 0.000001)?;

				// Calibrate the camera
				calibrate_camera(
					&points_real,
					&points_image,
					size,
					&mut camera_matrix,
					&mut dist_coeffs,
					&mut Mat::default(),
					&mut Mat::default(),
					CALIB_USE_INTRINSIC_GUESS,// + CALIB_FIX_PRINCIPAL_POINT,
					criteria,
				)?;

				for x in 0..camera_matrix.cols()
				{
					for y in 0..camera_matrix.rows()
					{
						print!("{}  ", camera_matrix.at_2d::<f64>(x, y).unwrap());
					}
					println!("");
				}
				for i in 0..dist_coeffs.cols()
				{
					println!("{:?}", dist_coeffs.at_2d::<f64>(0, i).expect("?"));
				}

				let mut undistorted_image = Mat::default();
				let mut undistorted_points : Vector<Point2f> = Vector::new();

				opencv::calib3d::undistort_points(&points_image.get(0)?, &mut undistorted_points, &camera_matrix, &dist_coeffs, &Vector::<Point2f>::new(), &Vector::<Point2f>::new());
				opencv::calib3d::undistort(&img_org, &mut img_un, &camera_matrix, &dist_coeffs, &camera_matrix);
				// undistort(&img_org, &mut undistorted_image, &camera_matrix, &dist_coeffs, &camera_matrix)?;
				for x in 0..img_org.cols()
				{
					for y in 0..img_org.rows()
					{
						let c = img_org.at_2d_mut::<opencv::core::Point3_<u8>>(y as i32, x as i32).expect("?");
						c.x = c.x.saturating_sub(10).saturating_mul(5);
						c.y = c.y.saturating_sub(10).saturating_mul(5);
						c.z = c.z.saturating_sub(10).saturating_mul(5);
						let c = img_un.at_2d_mut::<opencv::core::Point3_<u8>>(y as i32, x as i32).expect("?");
						c.x = c.x.saturating_sub(10).saturating_mul(5);
						c.y = c.y.saturating_sub(10).saturating_mul(5);
						c.z = c.z.saturating_sub(10).saturating_mul(5);
						let c = img_cor.at_2d_mut::<opencv::core::Point3_<u8>>(y as i32, x as i32).expect("?");
						c.x = c.x.saturating_sub(10).saturating_mul(5);
						c.y = c.y.saturating_sub(10).saturating_mul(5);
						c.z = c.z.saturating_sub(10).saturating_mul(5);
					}
				}
				
				
				let mut error_old = 0.0;
				let mut error_new = 0.0;
				for i in 0.. cor.len()
				{
					let thickness = 1;
					let pt_i = cor[i].image_px;
					let pt_r = cor[i].real_px;
					let pt_u = undistorted_points.get(i)?;
					
					let radius    = ((pt_i - pt_r).magnitude() * 3.0).round() as i32;

					let e_o = (pt_i - pt_r).magnitude();
					let e_n = (Vector2{x: pt_u.x as Decimal, y: pt_u.y as Decimal} - pt_r).magnitude();
					println!("Old Error:\t{}\nNew Error:\t{}\n", e_o, e_n);
					error_old += e_o;
					error_new += e_n;

					let color = Scalar::new(200.0, 100.0, 100.0, 0.0); // (BGR format)
					let pt_i = Point::new(pt_i.x.round() as i32, pt_i.y.round() as i32);
					let _= opencv::imgproc::circle(&mut img_cor, pt_i, radius, color, thickness, 1,0);

					let color = Scalar::new(100.0, 100.0, 200.0, 0.0); // (BGR format)
					let pt_r = Point::new(pt_r.x.round() as i32, pt_r.y.round() as i32);
					let _= opencv::imgproc::circle(&mut img_cor, pt_r, radius, color, thickness, 1,0);


					let radius = (e_n - e_o) * 20.0;
					let mut color = Scalar::new(0.0, 200.0, 200.0, 0.0);
					if radius < 0.0 { color = Scalar::new(200.0, 200.0, 0.0, 0.0); } // (BGR format)
					let pt_u = Point::new(pt_u.x.round() as i32, pt_u.y.round() as i32);
					let _= opencv::imgproc::circle(&mut img_cor, pt_u, e_n.round() as i32, color, thickness + 1, 1,0);
					
					//let _= o
				}
				error_old /= cor.len() as Decimal;
				error_new /= cor.len() as Decimal;
				println!("Average:\nOld Error:\t{}\nNew Error:\t{}", error_old, error_new);


				imshow("distorted", &img_org)?;
				imshow("corrected", &img_cor)?;
				imshow("undistorted", &img_un)?;
				println!("UNDISTORTED\n\n");
				wait_key(0)?;
			}

		}
	}
	return Ok(());
}
