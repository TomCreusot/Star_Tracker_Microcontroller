#![allow(unused_imports)]
#![macro_use]
//! This is an integration test of the whole of the tracking_mode module.
//! This includes the construction, verification and searching of the database to find specific stars.
//! This also provides a step by step guide to use the tracking mode algorithm.
//!

#[macro_use]
extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate star_tracker_database;
extern crate image;
extern crate rand;
extern crate opencv;

use rand::prelude::*;

use std::time::Duration;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::aliases::M_PI;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Hours;
use star_tracker_lib::util::units::Match;
use star_tracker_lib::util::units::Vector3;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Quaternion;
use star_tracker_lib::util::linear_lookup::LinearLookup;
use star_tracker_lib::image_processing::Image;

use star_tracker_lib::projection::ExtrinsicParameters;
use star_tracker_lib::projection::SpaceWorld;

use star_tracker_lib::attitude_determination::Quest;
use star_tracker_lib::attitude_determination::AttitudeDetermination;

use star_tracker_database::distribution::Distribute;
use star_tracker_database::io::Star;
use star_tracker_database::io::Io;
use star_tracker_database::tracking_mode::DatabaseGenerator;

use star_tracker_nix::tracking_mode::AbandonSearchTimeoutFailure;
use star_tracker_nix::image_processing::CVImage;


use opencv::highgui::wait_key;
use opencv::highgui::imshow;


pub fn main ( )
{
	let exclusive_folders: Vec<String> = std::env::args().collect();
	let samples = star_tracker_nix::io::Sample::load_samples();
	let mut names = Vec::new();
	let mut average = Vec::new();
	for sample in samples
	{
		for image_index in 0..sample.file_img.len()
		{
			let mut is_exclusive = false;
			for i in 1..exclusive_folders.len()
			{
				is_exclusive |= sample.file_img[image_index].contains(&exclusive_folders[i]);
			}
			if !is_exclusive { continue; }
			println!("{:?}", sample.file_img[image_index]);

			let img = CVImage::read(&sample.file_img[image_index]);

			let mut hist = [0; 256];
			let _ = img.histogram(&mut hist);

			names.push(sample.file_img[image_index].clone());

			let mut mean = 0;
			let mut num_elements = 0;
			for i in 0..hist.len()
			{
				num_elements += hist[i];
				mean += hist[i] * i as u64;
			}
			average.push(mean as f64 / num_elements as f64);

		}
	}

	for i in 0..average.len()
	{
		let mut mean = 0;
		println!("{}, {}", names[i], average[i]);
	}

}