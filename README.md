***WIP (NOT FINNISHED)***

## TODO
* Get Working.
* Find the apparent offset of actual pixel and distortion due to lens. - may not be necessary with fov.
* Account for hot pixels.
* Find centre of camera from pixel off centre.

# Star Tracker
This software is to find the looking direction of a camera given a collection of stars.
It is designed to run on high powered microcontrollers for low earth orbit cube satellites.

The method used:
* Takes 4 stars.
* The brightest star is known as the reference/pilot star.
* The angle between the other 2 stars and the farthest from the pilot star is found.
* This is then compared to a database which will store an angle and the position of the pilot star and the farthest.
* The software looks for a cluster of stars and generates a probability from this.
* The highest probability is likely the actual position.

## How To Use
Ensure you have the correct programs installed as listed bellow.
Type:
* `make run` To compile and run the simulation software for the microcontroller. [demo](out/demo)
* `make database` To compile and run the software to generate a database. [database_generator](out/database_generator)
* `make docs` To generate a doxygen html file [and go to](documentation/).
* `make test` To test all the test harnesses created.
* `make lcov` To generate a line coverage report for tests [and go to](coverage/).

Both [demo](out/demo) and [database_generator](out/database_generator) require a [config file](out/config.properties) to run properly.

# Navigation
The code is located in [libs](libs/), this is divided into the directories:
* [image_processing](libs/image_processing) for blob detection and image thresholding.
* [nix](libs/nix) for any unix specific implementation, this is for generating the database, reading png's and reading config files.
* [star_tracker](libs/star_tracker) for finding matches of unique star sets in a database.
* [util](libs/util) for any useful non-project specific implementations such as an array list and point class.

[libs](libs/) contains [database_generator.cc](libs/database_generator.cc) which generates a database compatible with the star_tracker code to be used by the microcontroller program.
It also contains [demo.cc](libs/demo.cc) which is the program in the implementation of unix showing how to implement it for a microcontroller.

## Built With
* cargo 1.51.0-nightly (783bc43c6 2021-01-20)

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
