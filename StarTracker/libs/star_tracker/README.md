# libs/star_tracker
This package is to have points input, these will then be compared with each other to find unique angles.

## How To Use
In the *BUILD* file, input:
```
deps = ["//libs/star_tracker:star_tracker", "//libs/star_tracker:combo"]
```
At the top of the desired file, input:
```
#include "libs/star_tracker/star_tracker.h"
using namespace star_tracker;
```

### Build
Input the following into your command line:
```
bazel build //libs/star_tracker:star_tracker
bazel build //libs/star_tracker:combo
```

### Test
Input the following into your command line:
```
bazel test //libs/star_tracker:star_tracker_test --test_output=all
bazel test //libs/star_tracker:combo_test --test_output=all
```


## Files
* star_tracker.h
* star_tracker.cc
* combo.cc
<br /><br />
* combo_test.cc
* star_tracker_test.cc
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
