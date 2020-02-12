***WIP (NOT FINNISHED)***

## TODO
* Get Working.
* Test Harness for Database and StarTracker/database.
* Embedded Implementation for StarTracker/get_image.
* Find the apparent offset of actual pixel and distortion due to lens.
* Account for hot pixels.
* Find centre of camera from pixel off centre.

# Star Tracker
This software is to find the looking direction of a camera given a collection of stars.
It is designed to run on high powered microcontrollers for low earth orbit cube satellites.

The method used is the Pyramid method which:
* Takes 4 stars.
* The brightest star is known as the reference/pilot star.
* The angle between the other 2 stars and the furthest from the pilot star is found.
* This is then compared to a database which will store an angle and the position of the pilot star.

## How To Use



# Navigation
The software is divided into 2 sections; StarTracker and Database.
StarTracker is the software to be uploaded to the microcontroller.
Database is a program designed to convert a database of stars into a readable format.

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0
* java jdk-headless 1.8.0_232
* java jre 1.8.0_232

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
