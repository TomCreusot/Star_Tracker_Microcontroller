# libs/database
This package is designed to compare calculated angles with database angles to create a probability factor of what the actual star is.

## How To Use
Go to [Database](../../Database), generate the c++ header file:
```
name: "database"
array_name: "array"
```
Insert it into this directory.

In the *BUILD* file, input:
```
deps = ["//libs/database:find_elements", "//libs/database:angle_stat"]
```
At the top of the desired file, input:
```
#include "libs/database/find_elements.h
#include "libs/database/angle_stat.h
```

### Build
Input the following into your command line:
```
bazel build //libs/database:find_elements
bazel build //libs/database:angle_stat
```

### Test
Input the following into your command line:
```
bazel test //libs/database:find_elements --test_output=all
bazel test //libs/database:angle_stat --test_output=all
```

## Files
* find_elements.h
* find_elements.cpp
* angle_stat.h
* angle_stat.cpp
<br /><br />
* find_elements_test.cc
* angle_stat.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Authors
* Tom Creusot - *Initial Creation*
