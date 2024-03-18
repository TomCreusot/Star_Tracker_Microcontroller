# Star Tracker Microcontroller
This project contains all the code to implement a star tracker for a satellite.  

The software is designed to run on high-powered microcontrollers for low-earth orbit cube satellites.  
It can also be used on a standard computer using a Ubuntu distribution (or windows through wsl).  


I have recorded a set of videos available on [youtube here](https://www.youtube.com/playlist?list=PLzWAMhj2ND0TXXEOIabAPgrrbsrhF2XHW) that you should look at before getting started.  
It explains star trackers, the code and any considerations you should make.  


## Implementation
This is a modular library and binary where tracking algorithms can quickly be swapped out.  
It is written in Rust with no_std so it can be run on a microcontroller, however, it can also run in Ubuntu (see star_tracker_nix).  
In star_tracker_lib, the following methods are implemented:  
- Thresholding:   Nilback (adaptive) or Percent (Binary) thresholds are implemented
- Blob Detection: Grassfire (consuming)
- Tracking Mode:  Pyramid Method
- Database:       K-Vector
- Voting:         QUaternion ESTimator Attitude Determination


## How To Use
Ensure you have the correct programs installed, if you have Ubuntu you can type [`./setup.sh`](setup.sh) in the working directory and it should download everything automatically.  
If there are any compile errors in the following programs, it is likely you are missing some c libraries and need to install them.  
You should also ensure that cargo and rustup are installed and the rustup version is set to `nightly`.

### [star_tracker_nix](star_tracker_nix/)
Contained is a set of example programs and useful analysis software.
To access these, you need to cd into star_tracker_nix.

[**demo**](star_tracker_nix/src/bin/demo.rs)  
In star_tracker_nix, run `cargo run --bin demo /`  
This runs a demo program that goes through the images in samples/.  
These are real images from a range of lenses and fields of view.  
There are some variables that can be changed inside of the demo code to make it run; faster, more reliably, more confident...  
You can also select specific images to test, by changing the command line argument from `/` to the name of the file, you can view that specific file.  

[**simulation**](star_tracker_nix/src/bin/simulation.rs)  
In star_tracker_nix, run `cargo run --bin simulatuion'  
This will simulate taking a set of photos around the celestial sphere.  
It has a variety of variables such as error, accepted wait time, field of view, and magnitude ... it is not set to anything special, play around with them.   
This is used to help identify ways to improve the algorithm and to make sure your hardware is up to spec.  


[**sky_survey**](star_tracker_nix/src/bin/sky_survey.rs)   
In star_tracker_nix, run `cargo run --bin sky_survey`  
This will analyze the Hypacarros database to find what magnitude is required for a specific lens for 100% sky coverage.  
It may take a while to run.  
If you want to select specific plot points, you will need to go into the code.  

[**magnitude**](star_tracker_nix/src/bin/magnitude.rs)  
In star_tracker_nix, run `cargo run --bin magnitude`  
This will look at all the corr.fits files inside of samples/ and will identify what the dullest stars were.  
This is useful to check the spec of your hardware.  

[**corr_analyser**](star_tracker_nix/src/bin/corr_analyser.rs)  
In star_tracker_nix, run `cargo run --bin corr_analyser`  
This will look at all the corr.fits files inside of samples/ and will identify the error of each lens.  
This is important as if enough stars are outside your error range, the algorithm will fail. 

### embedded
So you like my code enough to use it...  
It is up to you to select the HAL and how to flash onto the microcontroller.  
To get my library on it, you need to put this inside of your cargo.toml in your project.  
I have not tested this on a microcontroller yet.  
```
[dependencies.star_tracker_lib]
path = "../star_tracker_lib" # The location of star_tracker_lib relative to this cargo.
default_features = false

# If you are using a 64-bit system, swap "bit_32" for "bit_64".
# If you are designing this for the computer add "nix" to the features.
features = ["bit_32"] 
```


# Navigation
The library is located in [star_tracker_lib](star_tracker_lib/).  
This is divided into the modules:
* [image_processing](star_tracker_lib/src/image_processing/) for blob detection and image thresholding.
* [projection](star_tracker_lib/src/projection) for projecting the points on the image into 3d local points.
* [tracking_mode](star_tracker_lib/src/tracking_mode) for identifying the stars.
* [attitude_determination](star_tracker_lib/src/attitude_determination) for finding the center with the given stars.
* [util](star_tracker_lib/src/util) for any useful non-module-specific implementations such as an array list and units.
  
An implementation for the computer is located inside [star_tracker_nix](star_tracker_nix/) along with sample code.

A not-finished implementation for the STM32H7 is under development in [star_tracker_embed](star_tracker_embed/).  
Once again *not finished*.



## Considerations
This is a project designed by an engineering student over 4 years.  
I am not an expert, if you use this and something breaks, that's not my fault.  
I have not got this working on a microcontroller as flashing microcontrollers in Rust is hard.  
Although I have a very high test coverage, I cannot account for everything, it is up to you to ensure the tests match your standard.

## Testing
To test this I used [llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).

## Built With
* cargo nightly

## Documentation
The library has been extensively documented with step-by-step tutorials for each module.   
To access this, you need to compile the docs:
```
cd star_tracker_lib
cargo doc --no-deps --open 
```

## Author
* Tom Creusot (tomcreusot2000@gmail.com)
