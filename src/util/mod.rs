//! util contains any tools which may be important for general use in this project.  
//! Everything inside this module is expected to be used in multiple modules.  
//! 
//! This includes;
//! * Types of storage lists.
//! * Aliases of datatypes to decide precision.
//! * Coordinate systems and units that will be used.
//! * Types of errors.
//! * Methods for testing.

pub mod list;
pub mod units;
pub mod aliases;
pub mod err;
pub mod test;
pub mod linear_lookup;
