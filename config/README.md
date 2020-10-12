# Config
This stores all the current settings of the program.  
Feel free to change any accordingly, then recompile the code.  

## How To Use
Change the values in the file on the RHS of each variable to fit your requirements.  
Save the file.  
Recompile (make run, make database, etc).  

### To import
In C++ file:  
```#include "config/{file}.h"```  
In bazel:  
```deps=["config:{file}"]```

## Files
- database.h: Anything related to the construction of the database.
- demo.h: Anything specifically related to the demo code.
- runtime.h: Anything directly related to the main code.


## Built With
* g++	7.4.0
* bazel	2.0.0

## Authors
* Tom Creusot - *Initial Creation*
