# libs/nix
This is stuff which will only work on a nix system.

## How To Use
In the *BUILD* file, input:
```
deps = [//libs/nix:properties", "//libs/nix:star"]
```
At the top of the desired file, input:
```
#include "libs/nix/star.h"
#include "libs/nix/properties.h"
```

### Build
Input the following into your command line:
```
bazel build //libs/nix/...
```

### Test
Input the following into your command line:
```
bazel test //libs/nix/... --test_output=all
```

## Files
* properties.h
* properties.c
* star.h
* star.cc
* fill_template.h
* fill_template.cc
* linked_list.h
* get_image.h
* get_image.cc
<br /><br />
* star_test.cc
* properties_test.cc
* linked_list_test.cc
* star_test.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0
* gtest	1.10.0

## Authors
* Tom Creusot - *Initial Creation*
