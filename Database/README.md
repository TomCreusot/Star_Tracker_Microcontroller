# Database
This part of the project is designed to convert the database into a format easily read by the main project microcontroller.
The way this is done is:
	1. Having a source database of apparent magnitude, right ascension and declination of star charts.
	2. Converting it to angle, right ascention and declination.
	3. Filling in a template of an array header file.




## Getting Started
type: `bazel run //src:Main` to run the main program or go into src and type `javac *.java; java Main config.properties`.

### Config
In src/ there will be a file `config.properties`, this file has all the requirements to construct the database:
* *database*	The location of the mag,ra,dec database to convert.
* *fov*			The diagonal field of view of the camera.
* *cutoff-mag*	The maximum magnitude to include.
* *pilot-sets*	The number of stars to sample for each pilot star angle calculation.
* *array-name*	The name of the array to call in the c code.
* *template*	The template file to read from.
* *output*		Where to save the new database.




### Prerequisites
This requires java 8, if you are using unix you should be able to use:
```
sudo apt install openjdk-8-jdk-headless
```

The tests use JUnit, with unix use:
```
sudo apt install junit
```

It also requires a database of stars, perhaps [Astronexus](http://www.astronexus.com/hyg)
This database can be installed with the command:
```
make install
```
The database uses decimal degrees (easier to read by the computer).

It is important ***NOT*** to sort the database magnitude in order, the sorting algorithm requires a randomised list.




## Detailed Description

### Preprocessor
This converts a database of mag,ra,dec to angle,ra,dec.

### FillTemplate
This fills the template file with the array and several required variables to compile the c code.



## Running the tests
type: `bazel test //tests/...`




## Built with
* java openjdk 8 headless
* JUnit

## Contributing

## Authors
* **Tom Creusot** - *stuff*

## Other readings
