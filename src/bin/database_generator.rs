extern crate star_tracker;
use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;
use star_tracker::config::TrackingModeConstructConstsStruct;
use star_tracker::config::TrackingModeConstructConsts;
// use star_tracker::config::TrackingModeConstsStruct;
// use star_tracker::config::TrackingModeConsts;
// use star_tracker::config::DATABASE_ANGLE_TOLERANCE;
use star_tracker::nix::Star;
use star_tracker::nix::Io;
use star_tracker::nix::Template;
use star_tracker::tracking_mode::database::StarDatabaseElement;
use star_tracker::tracking_mode::database::KVector;

fn main ( )
{
	// Read CSV.
	println!("Reading CSV Database        ...");
	let mut rdr = Io::get_csv(NixConstsStruct::HYG_DATABASE_PATH, NixConstsStruct::HYG_DATABASE_URL);
	let iter = rdr.deserialize();
	
	// Create Star List +
	// Remove anything above the cutoff magnitude +
	// Sort in order of magnitude.
	println!("Filtering CSV Database      ...");
	let mut stars : Vec<Star> = Vec::new();
	for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		if star.mag < TrackingModeConstructConstsStruct::MAGNITUDE_MAX
		{
			stars.push(star);
		}
	}
	println!("\t- Found {} valid stars.", stars.len());
	
	println!("Sorting                     ...");
	stars.sort();
	
	// Create Star Database Element list.
	println!("Generating Star Pairs       ...");
	let mut star_pairs : Vec<StarDatabaseElement> = 
		StarDatabaseElement::create_list ( TrackingModeConstructConstsStruct::FOV, &stars );
	star_pairs.sort();
	println!("\t- Found {} pairs.", star_pairs.len());
	
	// Sort Star Database Element list.
	// Create K Vector
	println!("Generating KVector          ...");
	let k_vect = KVector::new(TrackingModeConstructConstsStruct::BINS_NUM, star_pairs[0].dist.0 as f64, star_pairs[star_pairs.len() - 1].dist.0 as f64);
	let bins : Vec<usize> = k_vect.generate_bins(&star_pairs)
		.expect("Not enough elements in the database.");
	

	println!("Generate Strings            ...");
	let mut bins_str : Vec<String> = Vec::with_capacity(bins.len());
	let mut pairs_str : Vec<String> = Vec::with_capacity(star_pairs.len());
	let mut stars_str : Vec<String> = Vec::with_capacity(stars.len());
	for e in &bins
	{
		bins_str.push(format!("{},", e).to_string());
	}
	for e in &star_pairs
	{
		pairs_str.push(format!("StarPair::<usize>({}, {}),", e.pair.0, e.pair.1).to_string());
	}
	for e in &stars
	{
		stars_str.push(format!("Equatorial{{ra: Radians({}f32), dec: Radians({}f32)}},", e.pos.ra, e.pos.dec).to_string());
	}
	// let star_pairs_str : Vec<String> = ;
	// Parse to Template File.
	println!("Parsing File                ...");
	let mut template_file = Io::read_file("src/config/template.txt");
	let mut template = Template::new();
	template.add_patten("FOV".to_string(), 				format!("{}f32", TrackingModeConstructConstsStruct::FOV).to_string());
	template.add_patten("MAGNITUDE".to_string(),	 	format!("{}f32", TrackingModeConstructConstsStruct::MAGNITUDE_MAX).to_string());
	template.add_patten("BIN_SIZE".to_string(), 		format!("{}", TrackingModeConstructConstsStruct::BINS_NUM).to_string());
	template.add_patten("K_LOOKUP".to_string(), 		k_vect.to_string());
	template.add_patten("K_VECTOR_SIZE".to_string(),	format!("{}", bins.len()).to_string());
	template.add_patten("STAR_PAIR_SIZE".to_string(), 	format!("{}", star_pairs.len()).to_string());
	template.add_patten("CATALOGUE_SIZE".to_string(), 	format!("{}", stars.len()).to_string());
	template.replace_lines(&mut template_file);
	template.replace_line_with_vec(&mut template_file, &"K_VECTOR_ELEMENTS".to_string(), &bins_str);
	template.replace_line_with_vec(&mut template_file, &"STAR_PAIR_ELEMENTS".to_string(), &pairs_str);
	template.replace_line_with_vec(&mut template_file, &"CATALOGUE_ELEMENTS".to_string(), &stars_str);
	
	Io::write_file("src/tracking_mode/database/array_database.rs", &template_file);
	println!("DONE");
}