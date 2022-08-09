extern crate star_tracker;

use star_tracker::nix::Star;
use star_tracker::nix::NixImage;
use star_tracker::nix::Color;
use star_tracker::image_processing::Image;
use star_tracker::util::aliases::Decimal;
use star_tracker::util::units::Pixel;
use star_tracker::util::units::Equatorial;
use star_tracker::util::units::Radians;
use star_tracker::util::units::Degrees;
use star_tracker::util::units::Vector2;

use star_tracker::nix::Io;
use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;

use star_tracker::projection::Transformation;
use star_tracker::projection::IntrinsicParameters;
use star_tracker::projection::ExtrinsicParameters;
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
	let img_size = Pixel{x: 1000, y: 1000};
	let mut img = NixImage::new(img_size);

	let mut rdr = Io::get_csv (
		NixConstsStruct::HYG_DATABASE_PATH,
		NixConstsStruct::HYG_DATABASE_FILE,
		NixConstsStruct::HYG_DATABASE_URL );


	// Extrinsic Parameters
	let cutoff_mag : Decimal     = 2.5;
	let dir        : Equatorial  = Equatorial{ra: Degrees(90.0).to_radians(), dec: Degrees(-16.0).to_radians()};

	// Intrinsic Parameters
	// let fov        : Radians     = Degrees(90.0).to_radians();
	// Vertical Field of View (Top of image to bottom or left to right)
	//    FOV		|	Focal Length
	// 10 degrees	|	5700.0
	// 20 degrees   |   2830.0
	// 30 degrees   |   1860.0
	// 40 degrees   |   1370.0
	// 50 degrees   |	1070.0
	// 60 degrees   |	864.0
	// 70 degrees   |	710.0
	// 80 degrees   |   594.0
	let fov = Degrees(90.0).to_radians();

	// Construct Matrix
	let extrinsic = ExtrinsicParameters::look_at(dir, Equatorial{ra: Radians(0.0), dec: Degrees(90.0).to_radians()});
	let intrinsic = IntrinsicParameters::from_fov(fov, img.height() as Decimal);
	let transform = Transformation{intrinsic: intrinsic, extrinsic: extrinsic};

	println!("10d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(10.0).to_radians()}.to_vector3()));
	println!("20d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(20.0).to_radians()}.to_vector3()));
	println!("30d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(30.0).to_radians()}.to_vector3()));
	println!("40d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(40.0).to_radians()}.to_vector3()));
	println!("50d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(50.0).to_radians()}.to_vector3()));
	println!("60d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(60.0).to_radians()}.to_vector3()));
	println!("70d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(70.0).to_radians()}.to_vector3()));
	println!("80d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(80.0).to_radians()}.to_vector3()));
	println!("90d {:?}", transform.to_image(Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(90.0).to_radians()}.to_vector3()));

	println!("");
	println!("");
	println!("Acher {:?}", transform.to_image(Equatorial{ra: Degrees(24.43).to_radians(), dec: Degrees(-57.23).to_radians()}.to_vector3()));
	println!("");
	println!("");


	let mut i = 0;
	let iter = rdr.deserialize();
	for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		let point = star.pos.to_vector3();

		if star.mag < cutoff_mag && star.pos.angle_distance(dir) < fov / 2.0//0.0 < point.dot(dir.to_vector3())
		{
			let plane : Vector2 = transform.to_image(point);
			let px    : Pixel = Pixel { x: (plane.x) as usize, y: (plane.y) as usize};
			if (&img as &dyn Image).valid_pixel(px)
			{
				let color;
				match i
				{
					0 => { color = Color::Black;	}
					1 => { color = Color::Red;		}
					2 => { color = Color::Green;	}
					3 => { color = Color::Blue;		}
					4 => { color = Color::Orange;   }
					5 => { color = Color::Turquoise;}
					6 => { color = Color::Purple;	}
					7 => { color = Color::Maroon;	}
					8 => { color = Color::GreenDark;}
					9 => { color = Color::Navy;		}
					10 => { color = Color::Pink;	}
					11 => { color = Color::Lime;	}
					12 => { color = Color::Sky;		}
					13 => { color = Color::Yellow;	}
					14 => { color = Color::Lavender;}
					_ => { color = Color::Grey;		}
				}
				img.draw_points(px, ((cutoff_mag - star.mag) as u32 + 2) * 10, color.get_color());
				i+=1;
				println!("{} {} \t {:.2}\t{:?}    \t{}    \t{:?}",
				i, star.pos, star.pos.angle_distance(dir).to_degrees().0, plane, star.name, color);
			}

		}
	}

	img.img_rgb.save("results/star_field.png").expect("Could not save.");



}
