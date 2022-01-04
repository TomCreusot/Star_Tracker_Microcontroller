# Code Conventions
This file is to help explain some rules used in this project to ensure safety and a simplified appearance.  


# Syntax
The syntax follows the rust style:
* `non_constant_variable`
* `normal_function`
* `ClassTraitEnumElementOrType`
* `CONSTANT_VARIABLES`



# Directory Structure
All files and directories use snake_case.  
Each directory represents a module and a `mod.rs` file will be located inside.  
The mod.rs files will have all the classes and traits defined in this file.  
All struct specific code will be located in a separate file usually named by the struct it is implementing for.  
  
To access any functionality:
`use star_tracker::module_name::Class`


# Safety and Errors
For error handling, panic! is not sufficient.  
Rust allows the use of Err() and None.  
Under [util/err.rs](util/err.rs), there is an enum containing a set of potential errors and a type alias which has the error type of the Errors enum.  
Use this to return errors or options.  

Times to return an error:
- When you would throw an exception
- When an input would cause a panic
e.g. out of bounds exeption, null pointer exception...
      
   
Times not to return an error:
- When rust wouldn't
e.g. Vec.get(index) does not have an error return (get is usualy checked before use).
- When the bounds are known at runtime
e.g. Matrix<Row, Column> and Image<X, Y> have fixed bounds, get and set are obvious and will return the same result every time.
- When it will be ignored every time
If there is no point in having an exception.