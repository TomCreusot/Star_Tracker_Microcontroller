# Star Tracker
This part of the project is designed to be installed on the microcontroller to run the star tracker method.
This uses the pyramid method, this method:
 * Finds 4 of the brightest stars in an image.
 * Finds the furthest star from the brightest (pilot).
 * Finds the angle of the furthest star in relation to the other 2 dull stars.
 * Compares to a database of all the stars in the night sky.
 * Once it has found enough matches, it will know the coordinates of the pilot star.
 * Finding the angle between the pilot star and the centre of the frame will find the pointing position.




## Getting Started
To install all the required libraries use:
'''
make install
'''




### Prerequisites
This project used c++ (g++) 7.4.0
'''
sudo apt install g++
'''

It also requires an image manipulation program written in c++, I used [EasyBMP](https://github.com/aburgh/EasyBMP), this mostly works, however it requires a .bmp image, standard windows conversion does not work, use paint and save as a bmp 24.
This library can be installed and integrated with the command:
'''
make install
'''





## Detailed Description

### ImageProcessing
This is in charge of machine vision, it will read in an array (byte image) and find details on the stars.
Ideally, calls should be in the order of:
 * adaptiveThreshold	*Blackens all the pixels which are considered 'background'*.
 * findBlobs			*Finds all the 'blob' (stars) in the image*.
 * getMainPoints		*Reduces the number of blobs to a specified number*.





## Running the tests




## Built with
* g++ 7.4.0


## Contributing


## Authors
* **Tom Creusot** - *stuff*


## Other readings
