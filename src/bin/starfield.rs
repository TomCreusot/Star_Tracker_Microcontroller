extern crate star_tracker;

use star_tracker::nix::Star;
use star_tracker::nix::NixImage;
// use star_tracker::nix::Color;
use star_tracker::image_processing::Image;
use star_tracker::util::aliases::Decimal;
use star_tracker::util::units::Pixel;
use star_tracker::util::units::Equatorial;
use star_tracker::util::units::Radians;
use star_tracker::util::units::Degrees;
// use star_tracker::util::units::Vector2;

use star_tracker::nix::Io;
use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;

use star_tracker::projection::IntrinsicParameters;
use star_tracker::projection::ExtrinsicParameters;
use star_tracker::projection::SpaceWorld;

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
	let cutoff_mag : Decimal     = 4.0;
	// let dir        : Equatorial  = Equatorial{ra: Degrees(90.0).to_radians(), dec: Degrees(-16.0).to_radians()};
	let dir        : Equatorial  = Equatorial{ra: Degrees(180.0).to_radians(), dec: Degrees(50.0).to_radians()};

	// Intrinsic Parameters
	let fov = Degrees(90.0).to_radians();

	// Construct Matrix
	let extrinsic = ExtrinsicParameters::look_at(dir, Equatorial{ra: Radians(0.0), dec: Degrees(90.0).to_radians()});
	let intrinsic = IntrinsicParameters::from_fov(fov, img.height() as Decimal);
	// let transform = Transformation{intrinsic: intrinsic, extrinsic: extrinsic};


	let iter = rdr.deserialize();
	for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		let point = SpaceWorld(star.pos.to_vector3());
		if star.mag < cutoff_mag
		{
			let size = cutoff_mag - star.mag;
			let red = 150_u8.saturating_add((star.mag * 10.0) as u8);
			img.draw_star(point, size + 3.0, [red, 50, 255], intrinsic, extrinsic);
		}
	}

	/*let mut i = 0;
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

		}*/
	// }

	img.img_rgb.save("results/starfield/star_field.png").expect("Could not save.");



}
