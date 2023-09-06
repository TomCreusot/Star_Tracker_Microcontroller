use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::ffi::OsStr;

use fitsio::FitsFile;
use star_tracker_lib::util::aliases::Decimal;

use star_tracker_lib::util::units::Equatorial;
// use star_tracker_lib::util::units::AngleAxis;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Pixel;

use crate::io::StarError;
use crate::io::Sample;

impl Sample
{
	/// Iterates the samples folder recursively to find all corr, log, image pairs and log files.
	/// The structure for loading should be:
	/// - samples
	///     - lens_1
	///          - *.log  // A file with log at the end.
	///          - *.fits // The corr.fits file from astrometry.net which applies to all the images in this folder.
	///          - *.png  // An image taken by the lens.
	///          - *.jpg  // Another image taken by the lens.
	///          - *.gif  // Another other image taken by the lens.
	///
	///     - lens_...
	pub fn load_samples ( ) -> Vec<Sample>
	{
		let path = "samples/";
		let mut val = Vec::new();

		std::fs::create_dir_all("samples/").unwrap();
		Sample::load_samples_rec(path, "".to_string(), &mut val);
		return val;
	}



	/// Uses the corr.fits file to find the corresponding star positions.
	/// Each element provided is the coordinates of the image, the coordinates of the star and how far they are.
	pub fn get_corr ( &self ) -> Option<Vec<StarError>>
	{
		let f = FitsFile::open(self.file_cor.clone());
		if let Ok(mut fits) = f
		{
			if let Ok(hdu) = fits.hdu(1)
			{
				let mut output : Vec<StarError> = Vec::new();

				let field_x   : Vec<f64> = hdu.read_col(&mut fits, "field_x")  .expect("Invalid fits.");
				let field_y   : Vec<f64> = hdu.read_col(&mut fits, "field_y")  .expect("Invalid fits.");
				let index_x   : Vec<f64> = hdu.read_col(&mut fits, "index_x")  .expect("Invalid fits.");
				let index_y   : Vec<f64> = hdu.read_col(&mut fits, "index_y")  .expect("Invalid fits.");
				let field_ra  : Vec<f64> = hdu.read_col(&mut fits, "field_ra") .expect("Invalid fits.");
				let field_dec : Vec<f64> = hdu.read_col(&mut fits, "field_dec").expect("Invalid fits.");
				let index_ra  : Vec<f64> = hdu.read_col(&mut fits, "index_ra") .expect("Invalid fits.");
				let index_dec : Vec<f64> = hdu.read_col(&mut fits, "index_dec").expect("Invalid fits.");


				for i in 0..field_x.len()
				{
					let img_px   = Vector2{x: field_x[i], y: field_y[i]};
					let real_px  = Vector2{x: index_x[i], y: index_y[i]};

					let img_eq_ra   = Degrees(field_ra[i]  as Decimal).to_radians();
					let img_eq_dec  = Degrees(field_dec[i] as Decimal).to_radians();
					let real_eq_ra  = Degrees(index_ra[i]  as Decimal).to_radians();
					let real_eq_dec = Degrees(index_dec[i] as Decimal).to_radians();

					let img_eq   = Equatorial{ra: img_eq_ra,  dec: img_eq_dec};
					let real_eq  = Equatorial{ra: real_eq_ra, dec: real_eq_dec};

					let error_px = (img_px - real_px).magnitude();
					let error_eq = img_eq.angle_distance(real_eq);

					let star = StarError
					{
						image_px: img_px, real_px: real_px, error_px: error_px,
						image_eq: img_eq, real_eq: real_eq, error_eq: error_eq,
					};
					output.push(star);
				}
				return Some(output);
			}
		}
		return Option::None;
	}




	/// Gets the size of the image from the log file.
	pub fn get_image_size ( &self ) -> Option<Pixel>
	{
		let mut file = File::open(self.file_log.clone()).ok()?;
		let mut json_str = String::new();
		file.read_to_string(&mut json_str).ok()?;
		let json: serde_json::Value = serde_json::from_str(&json_str).ok()?;

		let img: &serde_json::Value = json.get("image_size")?;
		let x = img.get("x")?.as_i64()? as usize;
		let y = img.get("y")?.as_i64()? as usize;

		return Some(Pixel{x: x, y: y});
	}


	/// Gets the diagonal field of view of the lens.
	/// Uses the log file.
	pub fn get_fov ( &self ) -> Option<Radians>
	{
		let mut file = File::open(self.file_log.clone()).ok()?;
		let mut json_str = String::new();
		file.read_to_string(&mut json_str).ok()?;
		let json: serde_json::Value = serde_json::from_str(&json_str).ok()?;

		return
			Some(Degrees(json.get("fov_deg")?.as_f64()? as Decimal).to_radians());
	}


	/// Gets the center of the image from the log file.
	pub fn get_center ( &self ) -> Option<Equatorial>
	{
		let mut file = File::open(self.file_log.clone()).ok()?;
		let mut json_str = String::new();
		file.read_to_string(&mut json_str).ok()?;
		let json: serde_json::Value = serde_json::from_str(&json_str).ok()?;

		let center: serde_json::Value = json.get("center")?.clone();
		let ra    = center.get("ra_degrees")?.as_f64()? as Decimal;
		let dec   = center.get("dec_degrees")?.as_f64()? as Decimal;

		return Some(Equatorial{ra: Degrees(ra).to_radians(), dec: Degrees(dec).to_radians()});
	}












	/// Recursively loads all the samples from the sample directory.
	fn load_samples_rec ( path: &str, dark_frame: String, to_add: &mut Vec<Sample> )
	{
		let mut cor_file  = "".to_string();
		let mut log_file  = "".to_string();
		let mut dark_file = dark_frame;
		let mut img_file  = Vec::new();


		for f in std::fs::read_dir(path).unwrap()
		{
			let file = f.unwrap().path();
			if file.is_file()
			{
				let extension = Path::new(&file).extension().and_then(OsStr::to_str).unwrap();
				let name = file.file_prefix().unwrap();
				if extension == "fits"  { 
					cor_file = file.as_path().to_str().unwrap().to_string(); 
				}
				if extension == "json"  { log_file = file.as_path().to_str().unwrap().to_string(); }
				if name == "dark_frame" 
				{ 
					dark_file = file.as_path().to_str().unwrap().to_string(); 
				}
				else if extension == "png" || extension == "jpg" || extension == "gif"
					|| extension == "bmp" || extension == "jpeg"
				{
					img_file.push(file.as_path().to_str().unwrap().to_string());
				}
			}
		}

		// Recurse into other files.
		// This happens second so that if there is a dark frame, that will be read first.
		for f in std::fs::read_dir(path).unwrap()
		{
			let file = f.unwrap().path();
			if file.is_dir()
			{
				Sample::load_samples_rec(file.to_str().unwrap(), dark_file.clone(), to_add);
			}
		}

		if cor_file != "" || img_file.len() != 0 || log_file != ""
		{
			let sample = Sample{
				dir: path.to_string(),
				file_cor:  cor_file,
				file_img:  img_file,
				file_log:  log_file,
				file_dark: dark_file,
			};
			to_add.push(sample);
		}
	}


}
