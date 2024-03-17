# Star Tracker Database
This library is designed to allow the user to generate k-vector databases for their project.  
The database can be generated into a rust or json file for embedded applications or directly generated for computers.  

## Embedded
To generate a database for an embedded application, it cannot be created at runtime due to computational complexity and memory capacity.  
This library allows you to create the database as a rust file to be compiled with the project or a json which can be uploaded.  
To do this, you need to run the contained binary `database_json` or `database_rust`.  
These binaries can be called by:

### Json
```
cargo run --bin database_json {{path to output}} {{path to config file}}        # Verbose
# OR
cargo run --bin database_json {{path to output}} {{path to config file}} -q     # Quiet Mode
```

### Rust
```
cargo run --bin database_rust {{path to output}} {{path to config file}}        # Verbose
# OR
cargo run --bin database_rust {{path to output}} {{path to config file}} -q     # Quiet Mode
```

### Command Line
The binary requires;
- The output file location and name. 
  For rust, this should be inside the src or lib project directory or lower.  
  For json, this should be accessible by the uploader.  

- The config file for constructing the database.


### Config File
The input config file is a json file specifying how the database should be constructed.  
Most values are optional as an *ideal* value has been found, this is not the *perfect* value, but usually good enough.  
Below is a list of parameters that can be input:  
**REQUIRED:**
- `fov_deg`:
  The diagonal field of view of the sensor.
  This has to be accurate, use nova.astrometry.net to get the best result.

- `angle_tol_deg`: 
  The error of pixels.
  You can either use nova.astrometry.net corr.fits file with corr_analyser.
  The error ideally should be equal or less than 0.1 degrees.
  The bigger the tolerance, the longer the search takes.

**OPTIONAL:**
- region_size_deg:
  To reduce the database, the sky is divided up into regions.
  Each region will have a limited amount of stars to ensure that there is enough stars at the poles and not too many in the milky way.
  Default = fov / 2.

- region_num_stars:
  Once there is this many stars in a region, all duller stars in this region will be removed.
  Default = 8.

- chunk_size:
  How far a star can be linked to another until they are too far away.
  If this is too big, you are introducing error which will make the search longer.
  If it is too small, you are limiting the amount of stars that can be matched.

- magnitude_max:
  The dullest star brightness allowed.
  By having this too high, the database will be bigger, if the magnitude is too low, you wont have enough coverage.
  The default value is calculated based on the input field of view, you probably should just use that.

**OPTIONAL FOR ONLY RUST:**
- mem_section_k_vector:
  In a microcontroller, the memory can be fragmented.
  Sometimes you need to specify memory locations `#[link_section = ".my_section"]`.
  If you want the k_vector to be stored somewhere specific, specify `.my_section`.
- mem_section_pairs:
  In a microcontroller, the memory can be fragmented.
  Sometimes you need to specify memory locations `#[link_section = ".my_section"]`.
  If you want the star pairs to be stored somewhere specific, specify `.my_section`.
- mem_section_catalogue:
  In a microcontroller, the memory can be fragmented.
  Sometimes you need to specify memory locations `#[link_section = ".my_section"]`.
  If you want the catalogue to be stored somewhere specific, specify `.my_section`.






## Unix
A database can be generated at runtime using a linux environment.  
To do this, follow the steps of database_rust.rs binary or the docs.  
Its pretty easy... 





## Built With
cargo 1.74.0-nightly (e6aabe8b3 2023-09-26)

## Documentation
The library has been extensively documented with step-by-step tutorials for each module.   
To access this, you need to compile the docs:
```
cargo doc --no-deps --open 
```

## Author
* Tom Creusot
