//! This is a set of units and coordinates which are used throughout the project.  
//!
//! Try to use these as much as possible as it is more explicit and avoids silly mistakes.  
pub mod unit_ops;
pub mod angles;
pub mod vector2;
pub mod vector3;
pub mod quaternion;
pub mod equatorial;
pub mod crp;
pub mod matrix;
pub mod bit_field;
pub mod angle_axis;

use crate::core_include::*;

use crate::util::aliases::Decimal;
use crate::util::aliases::UInt;

// use serde::Deserialize;

//###############################################################################################//
//										---	Angles ---
//###############################################################################################//

/// An angle in radians.  
///
/// Radians are the default measurement of angles.  
/// Degrees and Hours are only used as a user interface.
/// ```
/// use star_tracker_lib::util::units::Radians;
/// use star_tracker_lib::util::test::TestEqual;
/// use star_tracker_lib::util::aliases::DECIMAL_PRECISION;
///
/// let angle = Radians(1.2345);
/// angle.to_degrees();                                  // Easy to read format.
/// angle.to_hours();                                    // Standard Equatorial format.
/// 
/// angle.sin();                                         // Easier than angle.0.sin().
/// angle.cos();                                         // Easier than angle.0.cos().
///
/// assert_eq!(angle + Radians(1.0), Radians(2.2345));   // Can add standard decimal.
/// assert_eq!(angle - Radians(1.0), Radians(0.2345));   // Can multiply radians.
///
/// assert_eq!(angle * 2.0,          Radians(2.469));    // Can multiply standard decimal.
/// assert_eq!(angle * Radians(2.0), Radians(2.469));    // Can multiply radians.
///
/// assert_eq!(angle / 2.0,          Radians(0.61725));  // Can multiply standard decimal.
/// assert_eq!(angle / Radians(2.0), Radians(0.61725));  // Can multiply radians.
///
/// assert_eq!(-angle,               Radians(-1.2345));  // Can flip the sign.
/// assert_eq!((-angle).abs(),       1.2345);            // Can perform abs function.
///
/// assert_eq!(angle, angle + Radians(DECIMAL_PRECISION / 2.0)); // Does equality of near numbers.
/// angle.test_close(&Radians(2.0), 1.0);               // Does equality with test precision.
///
/// // For any other features, you can extract the decimal with:
/// angle.0;
///
/// println!("{}", angle);                              // Prints neatly "1.23r".
/// println!("{:?}", angle);                            // Prints in long form "1.2345".
/// ```
#[derive(Debug, Copy, Clone, PartialOrd)]
// #[derive(Deserialize)]
pub struct Radians ( pub Decimal );

/// An angle in degrees.  
///
/// Use degrees for user interface.  
/// Radians are used in the library.
/// ```
/// use star_tracker_lib::util::units::Degrees;
/// use star_tracker_lib::util::units::Radians;
/// use star_tracker_lib::util::test::TestEqual;
/// use star_tracker_lib::util::aliases::DECIMAL_PRECISION;
///
/// const angle_c: Radians = Degrees(123.4).as_radians();// Can be converted to radians as const.
///
/// let angle = Degrees(1.2345);
/// angle.to_radians();                                  // Standard library format.
/// angle.to_hours();                                    // Standard Equatorial format.
/// 
/// angle.sin();                                         // Easier than angle.0.sin().
/// angle.cos();                                         // Easier than angle.0.cos().
///
/// assert_eq!(angle + Degrees(1.0), Degrees(2.2345));   // Can add standard decimal.
/// assert_eq!(angle - Degrees(1.0), Degrees(0.2345));   // Can multiply radians.
///
/// assert_eq!(angle * 2.0,          Degrees(2.469));    // Can multiply standard decimal.
/// assert_eq!(angle * Degrees(2.0), Degrees(2.469));    // Can multiply radians.
///
/// assert_eq!(angle / 2.0,          Degrees(0.61725));  // Can multiply standard decimal.
/// assert_eq!(angle / Degrees(2.0), Degrees(0.61725));  // Can multiply radians.
///
/// assert_eq!(-angle,               Degrees(-1.2345));  // Can flip the sign.
/// assert_eq!((-angle).abs(),       1.2345);            // Can perform abs function.
///
/// assert_eq!(angle, angle + Degrees(DECIMAL_PRECISION / 2.0)); // Does equality of near numbers.
/// angle.test_close(&Degrees(2.0), 1.0);                // Does equality with test precision.
///
/// // For any other features, you can extract the decimal with:
/// angle.0;
///
/// println!("{}", angle);                               // Prints neatly "1.23d".
/// println!("{:?}", angle);                             // Prints in long form "1.2345".
/// ```
#[derive(Debug, Copy, Clone, PartialOrd)]//, PartialEq)]
pub struct Degrees ( pub Decimal );


/// An angle in 24 hours.  
///
/// Equatorial uses this format.  
/// This is used when reading in star locations or to output a value which can be used in another program.  
/// The other angle formats have more control.  
/// Use radians for the main library.  
/// Use degrees for readability.
/// ```
/// use star_tracker_lib::util::units::Hours;
/// use star_tracker_lib::util::test::TestEqual;
/// use star_tracker_lib::util::aliases::DECIMAL_PRECISION;
///
/// let angle = Hours(1.2345);
/// angle.to_radians();                                  // Standard library format.
/// angle.to_degrees();                                  // Human readable.
///
/// assert_eq!(-angle,         Hours(-1.2345));          // Can flip the sign.
/// assert_eq!((-angle).abs(), 1.2345);                  // Can perform abs function.
///
/// assert_eq!(angle, Hours(angle.0 + DECIMAL_PRECISION / 2.0)); // Does equality of near numbers.
/// angle.test_close(&Hours(2.0), 1.0);                  // Does equality with test precision.
///
/// // For any other features, you can extract the decimal with:
/// angle.0;
///
/// println!("{}", angle);                               // Prints neatly "1.23d".
/// println!("{:?}", angle);                             // Prints in long form "1.2345".
/// ```
#[derive(Debug, Copy, Clone, PartialOrd)]//, PartialEq)]
pub struct Hours ( pub Decimal );



//###############################################################################################//
//										---	Vector2 ---
//###############################################################################################//

/// Alias for Vector2Int (x: uusize, y: usize).  
///
/// Used to specify a position in an image or the size of an image.  
pub type Pixel = Vector2Int;


/// An integer version of Vector2.  
///
/// Mainly used for Pixel.  
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2Int
{
	pub x: usize,
	pub y: usize,
}

/// 2D Cartesian Coordinates in Decimal format.  
///
/// Useful for specifying locations of an image which are between pixels.  
/// Vector2 has a variety of functions, including standard operators.  
///
/// # Equality
/// TestEqual and PartialEq are implemented to allow comparisons of equal vectors.  
/// This ensures that if an equation and expected are off by a few bits, it is still valid.  
/// ```
/// use star_tracker_lib::util::units::Vector2;
/// use star_tracker_lib::util::aliases::DECIMAL_PRECISION;
/// use star_tracker_lib::util::test::TestEqual;
/// 
/// let vec_1 = Vector2{x: 1.0, y: 2.0}; 
/// let vec_2 = Vector2{x: 1.0 + DECIMAL_PRECISION / 2.0, y: 2.0 + DECIMAL_PRECISION / 2.0}; 
/// let vec_3 = Vector2{x: 2.0, y: 3.0}; 
///
/// // A slight tolerance for standard equality.
/// assert_eq!(vec_1, vec_2);
///
/// // Can provide a tolerance if needed.
/// assert!( vec_1.test_close(&vec_3, 1.0001)); // True
/// assert!(!vec_1.test_close(&vec_3, 0.9999)); // False
/// ```
///
/// # Operators
/// There are standard operators for basic maths.  
/// ```
/// use star_tracker_lib::util::units::Vector2;
/// 
/// let vec = Vector2{x: 1.0, y: 2.0}; 
/// // Addition / Subtraction 
/// assert_eq!(vec + vec,     Vector2{x:  2.0, y:  4.0});
/// assert_eq!(vec - vec,     Vector2{x:  0.0, y:  0.0});
///	assert_eq!(-vec,          Vector2{x: -1.0, y: -2.0});
///
/// // Multiplication / Division
///	assert_eq!(vec * 2.0,   Vector2{x:  2.0, y:  4.0});
///	assert_eq!(vec / 2.0,   Vector2{x:  0.5, y:  1.0});
/// ```
/// 
/// # Other Operators
/// There are other operates provided for vector maths such as:
/// * magnitude
/// * normalize  // mutates the actual object.
/// * normalized // returns the result.
/// * dot
/// * to_vector3
/// * to_pixel
#[derive(Copy, Clone)]
pub struct Vector2
{
	pub x: Decimal,
	pub y: Decimal,
}

//###############################################################################################//
//										---	Vector3 ---
//###############################################################################################//
/// A 3d cartesian vector/point.  
///
/// This is to represent cartesian coordinates for doing angle calculations.  
/// Cartesian is ideal over Equatorial in most circumstances as Equatorial has limited functionality.  
/// Equatorial coordinates must first be converted to cartesian for most calculations.  
/// Use Vector3 when you need to work with the coordinates.  
/// Use Equatorial when you need to store it as it has 1 less value.  
/// # Equality
/// TestEqual and PartialEq are implemented to allow comparisons of equal vectors.  
/// This ensures that if an equation and expected are off by a few bits, it is still valid.  
/// ```
/// use star_tracker_lib::util::units::Vector3;
/// use star_tracker_lib::util::aliases::DECIMAL_PRECISION;
/// use star_tracker_lib::util::test::TestEqual;
/// 
/// let vec_1 = Vector3{x: 1.0, y: 2.0, z: 3.0}; 
/// let vec_2 = Vector3{
/// 	x: 1.0 + DECIMAL_PRECISION / 2.0, 
/// 	y: 2.0 + DECIMAL_PRECISION / 2.0,
/// 	z: 3.0 + DECIMAL_PRECISION / 2.0}; 
/// let vec_3 = Vector3{x: 2.0, y: 3.0, z: 4.0}; 
///
/// // A slight tolerance for standard equality.
/// assert_eq!(vec_1, vec_2);
///
/// // Can provide a tolerance if needed.
/// assert!( vec_1.test_close(&vec_3, 1.0001)); // True
/// assert!(!vec_1.test_close(&vec_3, 0.9999)); // False
/// ```
///
/// # Operators
/// There are standard operators for basic maths.  
/// ```
/// use star_tracker_lib::util::units::Vector3;
/// 
/// let vec = Vector3{x: 1.0, y: 2.0, z: 3.0}; 
/// // Addition / Subtraction 
/// assert_eq!(vec + vec, Vector3{x:  2.0, y:  4.0, z:  6.0});
/// assert_eq!(vec - vec, Vector3{x:  0.0, y:  0.0, z:  0.0});
///	assert_eq!(-vec,      Vector3{x: -1.0, y: -2.0, z: -3.0});
///
/// // Multiplication / Division
///	assert_eq!(vec * 2.0,   Vector3{x:  2.0, y:  4.0, z:  6.0});
///	assert_eq!(vec / 2.0,   Vector3{x:  0.5, y:  1.0, z: 1.5});
/// ```
/// 
/// # Other Operators
/// There are other operates provided for vector maths such as:
/// * magnitude
/// * normalize  // mutates the actual object.
/// * normalized // returns the result.
/// * dot
/// * cross
/// * to_vector2
/// * to_equatorial
/// * to_matrix_row
/// * to_matrix_column
/// * to_matrix_column_homo // An extra row with 1 in it.
#[derive(Copy, Clone)]
pub struct Vector3
{
	pub x: Decimal,
	pub y: Decimal,
	pub z: Decimal,
}

//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//
/// Equatorial Coordinates are coordinates which define a point on a unit sphere.  
///
/// Right Ascension (ra) is defined as the angle around the equator from 0 to 2PI (Don't use 0h to 24h or 0 to 360 deg).  
/// Declination (dec) is defined as the angle from -PI to PI (Don't use -90 to 90 deg).  
/// Equatorial must be converted to cartesian Vector3 to perform any equations.  
/// Use Equatorial as an efficient way of storing positions as it is 2 numbers instead of 3.  
#[derive(Copy, Clone)]
// #[derive(Deserialize)]
pub struct Equatorial
{
	/// Right Ascention (along the equator [0, 360)).
	pub ra: Radians,
	/// Declination (from pole to pole [-90, 90]).
	pub dec: Radians,
}


//###############################################################################################//
//										--- Angle Axis ---
//###############################################################################################//
/// An axis describing the pivot point and an angle specifying how much to rotate around.  
///
/// This has a singularity at an angle of 0 and 180 degrees.  
/// The singularity occurs as each rotation can be represented as the opposite axis with the opposite angles.  
/// e.g.  
/// `angle: 120d, axis: (0, 0, 1) is the same as angle: -120d, axis: (0, 0, -1)`.
#[derive(Copy, Clone)]
pub struct AngleAxis
{
	pub angle: Radians,
	pub axis : Vector3,
}


//###############################################################################################//
//										---	Quaternion ---
//###############################################################################################//
/// Represents a 3D rotation **without** singularity.  
///
/// This method uses the LEFT HAND COORDINATE SYSTEM.  
/// This is the most ideal and confusing coordinate system.  
#[derive(Debug, Copy, Clone)]
pub struct Quaternion
{
	pub w: Decimal,
	pub x: Decimal,
	pub y: Decimal,
	pub z: Decimal,
}


//###############################################################################################//
//								---	classical rodrigues parameters ---
//###############################################################################################//
/// A CRP is an old method of describing a rotation (poorly).  
///
/// It is considered as a sphere cut in half on a plane.  
/// The top of a hemisphere is projecting onto the plane where the point is.  
/// There is a singularity if the point to project is at the projection point as it cannot be projected on the plane (infinity).  
/// There is not much information on this.  
///   
/// This coordinate system is an essential part of the quest algorithm (lib crate).  
/// The only implementation for this structure is to convert a set of numbers into a quaternion.  
/// [This is the code used](https://github.com/risherlock/Wahba/blob/master/matlab/algorithms/quest1981.m) for the quest algorithm.  
/// [Equation 69](https://malcolmdshuster.com/Pub_1981a_J_TRIAD-QUEST_scan.pdf#page=6) has the equation used to convert a crp to a quaternion.  
#[derive(Debug, Copy, Clone)]
pub struct CRP
{
	pub x: Decimal,
	pub y: Decimal,
	pub z: Decimal,
}






//###############################################################################################//
//										---	Matrix ---
//###############################################################################################//
/// An n x m matrix declared on the stack.  
/// Use for an extensive set of matrix equations and modifications.  
/// # Some Uses
/// ```
/// use star_tracker_lib::util::units::Matrix;
/// use star_tracker_lib::util::units::MatPos;
/// use star_tracker_lib::util::units::Vector3;
/// use star_tracker_lib::util::aliases::Decimal;
///
/// let mut matrix: Matrix<3, 2> = Matrix::new(); // A 0 matrix 3 high (3 rows), 2 wide (2 columns).
/// assert_eq!(matrix.width(), 2);
/// assert_eq!(matrix.height(), 3);
///
/// let eye:    Matrix<2, 2> = Matrix::identity(); // An identity 2x2 matrix.
/// assert_eq!(eye.get(MatPos{row: 0, col: 0}), 1.0);
/// assert_eq!(eye.get(MatPos{row: 0, col: 1}), 0.0);
/// assert_eq!(eye.get(MatPos{row: 1, col: 0}), 0.0);
/// assert_eq!(eye.get(MatPos{row: 1, col: 1}), 1.0);
/// assert_eq!(eye.trace(), 2.0); // You can do trace of any size. 
/// assert_eq!(eye.determinate(), 1.0); // You can do determinate of up to and including 4x4. 
///
/// matrix.set(MatPos{row: 1, col: 0}, 10.0); // sets row 1, col 0 to 10.
/// let mut trans: Matrix<2, 3> = matrix.transposed(); // You can transpose.
///
/// trans.insert(MatPos{row: 0, col: 1}, &eye); // Insert the identity matrix into this one.
/// // Trans (after insert):
/// // 1	10	0
/// // 0	1	0
///
/// // Matrix multiplication can be done for: 3x3, 3x4, 4x4
/// // There is also adjoint.
/// 
///
/// // You can create matrices from single values and vectors:
/// let small_mat: Matrix<1,1> = Matrix::from_decimal(10.0);
/// let decimal  : Decimal     = small_mat.to_decimal();     // 10.0.
///
/// let vec_mut  : Matrix<3,1> = Vector3::to_matrix_column(&Vector3{x: 10.0, y: 20.0, z: 30.0}); // row vec.
/// let vector   : Vector3     = vec_mut.to_vector3();
///
/// let vec_mut_homo : Matrix<4,1> = Vector3::to_matrix_column_homo(&Vector3{x: 10.0, y: 20.0, z: 30.0}); // homogenous column matrix.
/// let vector       : Vector3     = vec_mut_homo.to_vector3(); // divides by the 4th value.
///
/// 
/// ```
/// Column is the width or x.  
/// Row    is the height or y.  
#[derive(Copy, Clone)]
pub struct Matrix <const ROW: usize, const COLUMN: usize>
{
	/// The matrix.
	matrix: [[Decimal; COLUMN]; ROW],
}


/// The coordinates for the matrix.  
///
/// Remember rows go down (y), columns go across (x).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MatPos
{
	pub row: usize,
	pub col: usize,
}




//###############################################################################################//
//										---	BitField ---
//###############################################################################################//
/// Stores a bit field and can do bit operations easily.
/// Example
/// ```
/// use star_tracker_lib::util::units::BitField;
/// use star_tracker_lib::util::units::BitCompare;
///
/// let mut field_lsb : BitField = BitField(0b0011); 
/// let field_msb : BitField = BitField(0b1100); 
/// let field_on  : BitField = BitField(0b1111); 
/// let field_off : BitField = BitField(0b0000);
///
/// // Any ensures at least one bit is the same:
/// assert!(field_lsb.compare(BitCompare::Any(field_on)));   // 0011 & 1111 = true 
/// assert!(!field_lsb.compare(BitCompare::Any(field_off))); // 0011 & 0000 = false
/// assert!(!field_lsb.compare(BitCompare::Any(field_msb))); // 0011 & 1100 = false
///
/// // All ensures all bits are equal:
/// assert!(!field_lsb.compare(BitCompare::All(field_on)));   // 0011 == 1111 = false
/// assert!(field_lsb.compare(BitCompare::All(field_lsb)));  // 0011 == 0011 = true
///
/// // Bits can easily be set:
/// field_lsb.set(3, true); // 0111.
/// field_lsb.set(4, true); // 1111.
/// assert!(!field_lsb.compare(BitCompare::All(field_on)));   // 1111 == 1111 = true
///
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BitField ( pub UInt );

/// Specifies how to compare bit fields.  
/// Refer to [BitField](crate::util::units::BitField) for uses.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BitCompare
{
	/// Bit Or.
	Any(BitField),
	/// Bit And.
	All(BitField),
}




//###############################################################################################//
//										--- Match ---
//###############################################################################################//
/// A predicted input, possible match output and a weighting of uncertaincy.  
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Match <T>
{
	/// The values to be identified.
	pub input: T,
	/// The values from the database.
	pub output: T,
	/// The likelyhood of accuracy (futher from the center should have a lower accuracy).
	pub weight: Decimal,
}

