***WIP (NOT FINNISHED)***

# Star Tracker
This software is to find the looking direction of a camera given a collection of stars.
It is designed to run on high powered microcontrollers for low earth orbit cube satellites.

## How To Use
* Ensure you have the correct programs installed, if you have ubuntu you can type ``./setup.sh` and it should download everything automatically.
* Configure the variables of your sensor by modifying [mod.rs](src/config/mod.rs).
* run the desired program in [bin](src/bin) by typing `cargo run --bin [program]`

### `cargo run --bin demo`
This runs a demo program using the config variables specified, it will read the specified image, output an image with the stars highlighted and then identify the pointing location.  
*RUN DATABASE_GENERATOR FIRST*


### `cargo run --bin database_generator`
Generates the database for demo with the specified variables

### `cargo run --bin analyse_database`
Uses the specified variables to calculate expected size of the database and how reliable the database is.

### `cargo run --bin spectral_test`
Uses the database to find the number of stars in different spectral classes which can be copied into a spreadsheet.

### `cargo run --bin starfield`
Generates a sample star image to test the software.

### `cargo run --bin sky_survey`
Creates a set of tables showing how much coverage you will have at specific magnitudes and brightness's.
This program takes some time to run, if you want to adjust the time, you can alter the number of data points.

# Navigation
The code is located in [src](src/), this is divided into the directories:
* [image_processing](src/image_processing) for blob detection and image thresholding.
* [nix](src/nix) for any unix specific implementation, this is for generating the database, reading png's and reading config files.
* [star_tracker](src/star_tracker) for finding matches of unique star sets in a database.
* [util](src/util) for any useful non-project specific implementations such as an array list and point class.

[src](src/) contains [database_generator.cc](src/database_generator.cc) which generates a database compatible with the star_tracker code to be used by the microcontroller program.
It also contains [demo.cc](src/demo.cc) which is the program in the implementation of unix showing how to implement it for a microcontroller.

## Built With
* cargo 1.51.0-nightly (783bc43c6 2021-01-20)
To implement this method, use command:  
```
rustup default nightly-2021-08-20-x86_64-unknown-linux-gnu
```
Any version between 2021-03-20 and 2021-08-31 should work.

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
