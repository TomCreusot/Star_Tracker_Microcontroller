# libs/image_processing
This package provides machine vision blob detection.

## How To Use
In the *BUILD* file, input:
```
deps = [
	"//libs/ImageProcessing:image_processing"
	"//libs/image_processing:Blob",
	]
```
At the top of the desired file, input:
```
#include "../image_processing/image_processing"
#include "../image_processing/Blob"
```

### Build
Input the following into your command line:
```
bazel build //libs/image_processing:ImageProcessing
bazel build //libs/ImageProcessing:Blob
```

### Test
Input the following into your command line:
```
bazel test //libs/Utils:PointTest --test_output=errors
bazel test //libs/ImageTest --test_output=errors
```

## Files
* ImageProcessing.hpp
* ImageProcessing.cpp
* Blob.hpp
* Blob.cpp
*
* ImageProcessingTest.cc
* BlobTest.cc
*
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Contributing

## Authors
* Tom Creusot - *Stuff*
