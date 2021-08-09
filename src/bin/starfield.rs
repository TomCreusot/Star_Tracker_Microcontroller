fn main ( )
{
	println!("WIP");
}
/*
// extern crate star_tracker;
// use star_tracker::nix::Star;
// use star_tracker::util::aliases::Decimal;
//use star_tracker::config::starfield;

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
    const PT = Equatorial{ra: 0, dec: 0};
	const FOV : f32 = 0.3490658504;
	const CUT_OFF_MAG : f32 = 5.5;

	println!("Reading File");
	let mut stars : Vec<Star> = Star::stars_from_csv("hygdata_v3.csv", CUT_OFF_MAG, 7, 8, 13);

    println!("Constructing Image");
    let img = NixImage::new();

    for e in &stars
	{
        if ( e.angular_distance() < PT )
        {

        }
	}



}
*/
