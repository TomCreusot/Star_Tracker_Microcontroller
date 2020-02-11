# Database
This part of the project is designed to convert the database into a format easily read by the main project microcontroller.
The way this is done is:
	1. Having a source database of apparent magnitude, right ascension and declination of star charts.
	2. Calling CSVToCArray with the correct command line arguments.




## Getting Started
Use bazel build //libs:csvToCArray




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

### DPreprocessor.exe
This program is designed to read a database in the format of:
csv, with 3 columns in order of:
Apparent magnitude, Right Ascension, Declination.
The first row will be assumed as headers and will be ignored.

It will convert it into a more useful format for the main program.
This format is:
* The angle of the furthest star from the pilot star in the sample.
* The right ascension of the pilot star.
* The declination of the pilot star.

The pyramid method will be calculated on a specified amount of the closest stars to the pilot.

The program will also exclude any stars above a specified apparent magnitude.
The pilot stars will not be reused, however the others can be.

Depending on how large the database is, this may take a long time.

### DToTree.exe
This program converts the database specified into a balanced tree format from the first column.
It only accepts 3 doubles.
This should not take nearly as long as DPreprocessor.exe

### DToBinary.exe
This converts the database into a binary format.




## Running the tests





## Built with
* java openjdk 8 headless
* JUnit

## Contributing

## Authors
* **Tom Creusot** - *stuff*

## Other readings
