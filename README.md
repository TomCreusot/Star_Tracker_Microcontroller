***WIP (NOT FINNISHED)***

# Star Tracker
This software is to find the looking direction of a camera given a collection of stars.
It is designed to run on high powered microcontrollers for low earth orbit cube satellites.

## How To Use
Ensure you have the correct programs installed as listed bellow.


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

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
