//! util contains any tools which may be important for general use in this project.  
//! Everything inside this module is expected to be reused.  
//! This includes;
//! * Aliases of datatypes to decide precision.
//! * Coordinate systems that will be used.
//! * Types of storage lists.

pub mod list;
pub mod units;
pub mod aliases;
pub mod err;
pub mod test;
pub mod linear_lookup;