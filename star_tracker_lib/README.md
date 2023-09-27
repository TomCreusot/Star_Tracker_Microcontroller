# Star Tracker Lib
This library contains all the required code for the device that runs the star tracker.  
Please refer to the [docs](star_tracker_lib/target/doc/star_tracker_lib/index.html) once downloaded (github doesn't host the webpage so you need to download it first).  
Contained are the modules:  

## [image_processing](src/image_processing/)
This contains the code to store, threshold and blob detect an image.  
It is designed in a way where each part can be swapped with your own custom implementation.
The implemented methods include:
- Niblack Thresholding (adaptive)
  This method generates a grid of thresholds instead of using a single global threshold but at the speed of a global threshold.
  
- Percent Threshold (Global)
  A basic global thresholding method.
  
- Grass Fire Blob Detection
  A way of finding all the stars in the image.
  This is a consuming method that will modify the image as it goes (make a copy first).

- Basic Image
  A 2D fixed array that stores bytes.  
  You can also write your own implementation using the Image trait.

## [projection](src/projection) 
This is for projecting the points of the image into 3D local points.
This uses the intrinsic/extrinsic method and does not contain anything to counter lens distortion.  
Through testing, it was found that the star tracker algorithm was robust enough to not need to account for lens distortion.


## [tracking_mode](src/tracking_mode) 
This module identifies stars using the Pyramid method.  
The pyramid is a robust method which;
  1. Will match the arc angles of 3-star pairs in the image with pairs in the database.
  2. Check if the stars can form a triangle (each star pair combines with another).
  3. Check the triangle is not flipped.
  4. Find the arc angles between a 4th star and the other 3 and match it to the database.


## [attitude_determination](src/attitude_determination) 
This module takes in the observed/expected star positions and decides on a center point and rotation.  
This uses the QUEST, QUaternion Estimator Method.  
This model is a fast way of estimating the center which outputs a Quaternion rotation.

## [util](src/util) 
This module contains anything that is not module-specific such as;
  - Array Lists / Vectors
  - Units
  - Coordinates
  - ...
