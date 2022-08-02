# Lens Calibration
A lens is not perfect and moving a pixel from the center is not always the same angle.  
The lens must be calibrated.  
By running the system through astronomy.net, a corr.fits file is generated.  
Put the corr.fits file in the corr_files folder and the image as image.jpg.  
Run the program:
```
reset;clear; cargo run --bin lens_calibration
```
This will output a set of images in the folder *out*:
* `img.png` - This shows
* `img_curve.png` - This shows