# libs/image_processing
This package is to manage blob image detection.
It holds a class for storing images in byte monochrome and a class for storing details about significant pixels in the image.
Through running, you can input an image and output all *blobs* in the image.


## How To Use
In the *BUILD* file, input:
```
deps = [
	"//libs/image_processing:image"
	"//libs/image_processing:blob",
	]
```
At the top of the desired file, input:
```
#include "libs/image_processing/image.h"
#include "libs/image_processing/blob.h"
```

### Build
Input the following into your command line:
```
bazel build //libs/image_processing/...
```

### Test
Input the following into your command line:
```
bazel test //libs/image_processing/... --test_output=all
```

## Files
* blob.h
* blob.cc
* image.h
* image.cc
<br /><br />
* image_test.cc
* blob_test.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
