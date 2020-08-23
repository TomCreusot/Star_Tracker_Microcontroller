# libs/properties
This is to have a set of constants required for running the program.
These constants should be set

## How To Use
In the *BUILD* file, input:
```
deps = ["//libs/properties/..."]
```
At the top of the desired file, input:
```
#include "libs/properties/properties.h"
using namespace properties;
```

### Build
Input the following into your command line:
```
bazel build //libs/properties/...
```

### Test
Input the following into your command line:
```
bazel test //libs/properties/... --test_output=all
```


## Files
* properties.h
* properites_template.cc
<br /><br />
* BUILD
* README.md

## Built With
* g++	7.4.0
* bazel	2.0.0

## Contributing

## Authors
* Tom Creusot - *Initial Creation*
