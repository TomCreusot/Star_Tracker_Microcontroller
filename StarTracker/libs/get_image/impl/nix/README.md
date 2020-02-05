# GetImage ** nix **
This implementation will read a bitmap 24 file using the EasyBMP and return it as an image.


## How To Use



### Test
bazel test //libs/GetImage/impl/nix:TestGetImage.cc

The image file 'test-img.bmp' contains pixels of value:

255						|240								|		200 \n
150R, 150G, 0B (100)	|		150R, 0G, 150B (100)		|		0


## Files
* GetImage.hpp
* GetImage.cpp
*
* GetImageTest.cc
* test_img.bmp
*
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0
* EasyBMP

## Contributing

## Authors
* Tom Creusot - *Stuff*
