# Star Tracker Nix
This is a set of platform dependant unix code and binaries.   
Use this to construct a database and run tests.  
Also if you need a star tracker but don't need it to be on a microcontroller, maybe you can just use this.  
Here are some executables:  
[**demo**](src/bin/demo.rs)  
In star_tracker_nix, run `cargo run --bin demo /`  
This runs a demo program that goes through the images in samples/.  
These are real images from a range of lenses and fields of view.  
There are some variables that can be changed inside of the demo code to make it run; faster, more reliably, more confident...  
You can also select specific images to test, by changing the command line argument from `/` to the name of the file, you can view that specific file.  

[**simulation**](src/bin/simulation.rs)  
In star_tracker_nix, run `cargo run --bin simulatuion'  
This will simulate taking a set of photos around the celestial sphere.  
It has a variety of variables such as error, accepted wait time, field of view, and magnitude ... it is not set to anything special, play around with them.   
This is used to help identify ways to improve the algorithm and to make sure your hardware is up to spec.  


[**sky_survey**](src/bin/sky_survey.rs)   
In star_tracker_nix, run `cargo run --bin sky_survey`  
This will analyze the Hypacarros database to find what magnitude is required for a specific lens for 100% sky coverage.  
It may take a while to run.  
If you want to select specific plot points, you will need to go into the code.  

[**magnitude**](src/bin/magnitude.rs)  
In star_tracker_nix, run `cargo run --bin magnitude`  
This will look at all the corr.fits files inside of samples/ and will identify what the dullest stars were.  
This is useful to check the spec of your hardware.  

[**corr_analyser**](src/bin/corr_analyser.rs)  
In star_tracker_nix, run `cargo run --bin corr_analyser`  
This will look at all the corr.fits files inside of samples/ and will identify the error of each lens.  
This is important as if enough stars are outside your error range, the algorithm will fail. 
