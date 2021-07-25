extern crate star_tracker;
use std::mem::size_of_val;
use star_tracker::tracking_mode::database::array_database::K_VECTOR_DATABASE;
use star_tracker::tracking_mode::database::array_database::STAR_PAIR_DATABASE;
use star_tracker::tracking_mode::database::array_database::CATALOGUE_DATABASE;

// use star_tracker::tracking_mode::StarPair;

use star_tracker::util::units::Equatorial;
// use star_tracker::util::units::Radians;

fn main ( )
{

	println!("\n\n");
	size_test ( );
	k_vector_bin_test ( );
	star_pair_test ( );
	star_catalogue_test ( );
}


fn size_test ( )
{
	println!("--- MEMORY USAGE ---");
	
	let size_k_vector_database  = size_of_val(&K_VECTOR_DATABASE);
	let size_star_pair_database = size_of_val(&STAR_PAIR_DATABASE);
	let size_catalogue_database = size_of_val(&CATALOGUE_DATABASE);
	println!("Size K Vector Bins: {} Bytes", size_k_vector_database);
	println!("Size Star Pairs:    {} Bytes", size_star_pair_database);
	println!("Size Catalogue:     {} Bytes", size_catalogue_database);
	let size_total = size_k_vector_database + size_star_pair_database + size_catalogue_database;
	println!("Total Size:         {} kB", size_total / 1000);
	// println!("Size Pairs: {}", size_of(STAR_PAIR_DATABASE));
	// println!("Size Catalogue: {}", size_of(CATALOGUE_ELEMENTS));
	
	println!("\n\n");
		
}



fn k_vector_bin_test ( )
{
	println!("--- K-Vector Bin ---");
	
	let length					= K_VECTOR_DATABASE.len();
	let bin_length				= length - 1;
	let mut min_populated		= K_VECTOR_DATABASE[0];
	let mut max_populated		= K_VECTOR_DATABASE[0];
	let mut num_empty			= 0;
	let mut weighted_populated	= 0;
	let num_elements		= K_VECTOR_DATABASE[length - 1] - K_VECTOR_DATABASE[0];
	
	for i in 1..length
	{
		let separation = K_VECTOR_DATABASE[i] - K_VECTOR_DATABASE[i - 1];
		if min_populated > separation
		{
			min_populated = separation;
		}
		if max_populated < separation
		{
			max_populated = separation;
		}
		if separation == 0
		{
			num_empty += 1;
		}
		weighted_populated += i * separation;
	}
	println!("SIZE:                      {}", length);
	println!("BINS:                      {}", bin_length);
	println!("MIN POPULATED:             {}", min_populated);
	println!("MAX POPULATED:             {}", max_populated);
	println!("% EMPTY (LOW IS BETTER):   {} %", num_empty as f32 / (length - 1) as f32 * 100.0);
	
	weighted_populated /= num_elements;
	let off_center = bin_length as f32 / 2.0 - weighted_populated as f32;
	let rat_off_center = off_center / bin_length as f32 * 2.0;
	let rat_leaning = rat_off_center;
	println!("CLUSTER (IDEAL 0%):       {} %", rat_leaning * 100.0);
	
	println!("\n\n");
}




fn star_pair_test ( )
{
	println!("--- Star Pair Test ---");
	
	let mut num_overlap = 0;
	let mut reference_same = 0;
	let mut outside_bounds = 0;
	for ii in 0..STAR_PAIR_DATABASE.len()
	{
		let a = STAR_PAIR_DATABASE[ii];
		if a.0 == a.1
		{
			reference_same += 1;
		}
		if a.0 > CATALOGUE_DATABASE.len() - 1 || a.1 > CATALOGUE_DATABASE.len() - 1
		{
			outside_bounds += 1;
		}
		
		for jj in ii + 1..STAR_PAIR_DATABASE.len()
		{
			let b = STAR_PAIR_DATABASE[jj];
			if (a.0 == b.0 && a.1 == b.1) || (a.0 == b.1 && a.1 == b.0)
			{
				num_overlap += 1;
			}
			
		}
	}
	
	println!("NUM PAIRS:                   {}", STAR_PAIR_DATABASE.len());
	println!("AVERAGE REF/STAR             {}", STAR_PAIR_DATABASE.len() / CATALOGUE_DATABASE.len());
	println!("COPIES:  (IDEAL 0)           {}", num_overlap);
	println!("INVALID: (IDEAL 0)           {}", reference_same);
	println!("INVALID POSITION: (IDEAL 0)  {}", outside_bounds);
	
	
	println!("\n\n");
}




fn star_catalogue_test ( )
{
	println!("--- Star Catalogue Test ---");
	
	let mut outside_range_ra = 0;
	let mut outside_range_dec = 0;
	let mut similar_place = 0;
	
	for ii in 0..CATALOGUE_DATABASE.len()
	{
		let a = CATALOGUE_DATABASE[ii];
		
		if Equatorial::range_ra().end().0 <= a.ra.0 || a.ra.0 < Equatorial::range_ra().start().0
		{
			outside_range_ra += 1;
		}
		if Equatorial::range_dec().end().0 <= a.dec.0 || a.dec.0 < Equatorial::range_dec().start().0
		{
			outside_range_dec += 1;
		}
		
		for jj in (ii+1)..CATALOGUE_DATABASE.len()
		{
			let b = CATALOGUE_DATABASE[jj];
			
			if (a.ra - b.ra).0.abs() < 0.00001 && (a.dec - b.dec).0.abs() < 0.000001
			{
				similar_place += 1;
			} 
		}
	}
	
	println!("NUM STARS:              {}", CATALOGUE_DATABASE.len());
	println!("OUTSIDE RANGE RA:       {}", outside_range_ra);
	println!("OUTSIDE RANGE DEC:      {}", outside_range_dec);
	println!("SAME PLACE:             {}", similar_place);
	println!("\n\n");
}