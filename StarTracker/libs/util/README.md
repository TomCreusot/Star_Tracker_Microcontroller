# libs/util
This package provides useful objects and typedefs used through out the project.

## How To Use
In the *BUILD* file, input:
```
deps = ["//libs/util:point", "//libs/util:array_list", "//libs/util:util"]
```
At the top of the desired file, input:
```
#include "libs/util/point.hpp"
#include "libs/util/array_list.hpp"
#include "libs/util/utils.hpp"
```

### Build
Input the following into your command line:
```
bazel build //libs/util:point
bazel build //libs/util:array_list
```

### Test
Input the following into your command line:
```
bazel test //libs/util:point_test --test_output=all
bazel test //libs/util:array_list_test --test_output=all
```

## Files
* point.h
* array_list.h
* util.h
<br /><br />
* point_test.cc
* array_list_test.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Authors
* Tom Creusot - *Initial Creation*
