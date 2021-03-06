extern crate star_tracker;
use star_tracker::nix::Star;
use star_tracker::util::aliases::Decimal;

fn main ( )
{
	const FOV : f32 = 0.3490658504;
	const CUT_OFF_MAG : f32 = 5.5;
	println!("Reading File");
	let mut stars : Vec<Star> = Star::stars_from_csv("hygdata_v3.csv", CUT_OFF_MAG, 7, 8, 13);
	for e in &stars
	{
		// println!("{}",e.magnitude);
	}
	
	println!("Found: {} stars.", stars.len());
	println!("Sorting By Magnitude");
	stars.sort_unstable();
	
	println!("Generating Separation Histogram:");
	let sep_hist = separation_histogram(FOV, &stars);
	println!();

	
	println!("Creating Sets");
	
	
	println!("Converting to String");
	
	
	println!("Writing to File");
}


fn separation_histogram ( fov: Decimal, stars: &Vec<Star> )
{
	let mut hist = [0; 20];
	for ii in 0..stars.len()
	{
		let mut cur = 0;
		for jj in 0..stars.len()
		{
			if ii != jj
			{
				let p = stars[ii].position.clone();
				let o = stars[jj].position.clone();
				if p.angle_distance(o) < fov / 2.0
				{
					cur += 1;
				}
			}
		}
		let percent = (ii as f32 / stars.len() as f32 * 100.0) as i32;
		if percent % 5 == 0
		{
			print!("\r{}%    ", (ii as f32 / stars.len() as f32 * 100.0) as i32);
		}
		hist[std::cmp::min(cur, hist.len() - 1)] += 1;
	}
	println!();
	for i in 0..hist.len() - 1
	{
		println!("{}: {}", i, hist[i]);
	}
	println!("Over {}: {}", hist.len(), hist[hist.len() - 1]);
}
