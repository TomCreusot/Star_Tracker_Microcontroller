# libs/util
This package provides useful objects and typedefs used through out the project.
This helps to reduce coupling as the util classes are data structures.

## How To Use
In the *BUILD* file, input:
```
deps = ["//libs/util:point", "//libs/util:array_list", "//libs/util:util"]
```
At the top of the desired file, input:
```
#include "libs/util/utils.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"
#include "libs/util/properties.h"
```

### Build
Input the following into your command line:
```
bazel build //libs/util/...
```

### Test
Input the following into your command line:
```
bazel test //libs/util/... --test_output=all
```

## Files
* point.h
* array_list.h
* array_list_mock.h
* util.h
<br /><br />
* point_test.cc
* array_list_test.cc
* array_list_mock_test.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Authors
* Tom Creusot - *Initial Creation*
