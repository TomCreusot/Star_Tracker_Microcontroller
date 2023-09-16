#![feature(path_file_prefix)]

extern crate star_tracker_lib;
extern crate serde_json;
extern crate fitsio;
extern crate opencv;
extern crate serde;
extern crate curl;
extern crate rand;

pub mod image_processing;
pub mod tracking_mode;
pub mod util;
pub mod io;
