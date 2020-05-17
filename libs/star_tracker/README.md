# libs/star_tracker
This package is to have points input, these will then be compared with each other to find unique angles.

## How To Use
In the *BUILD* file, input:
```
deps = ["//libs/star_tracker/..."]
```
At the top of the desired file, input:
```
#include "libs/star_tracker/star_set.h"
using namespace star_tracker;
```

### Build
Input the following into your command line:
```
bazel build //libs/star_tracker/...
```

### Test
Input the following into your command line:
```
bazel test //libs/star_tracker/... --test_output=all
```


## Files
* star_set.h
* star_set.cc
* find_elements.h
* database_array.h
<br /><br />
* star_set_test.cc
* find_elements.cc
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
