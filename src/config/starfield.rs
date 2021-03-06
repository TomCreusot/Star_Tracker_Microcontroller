/// This struct is required to run starfield.
/// 
///
///
///


// * rotation (quaternion)
struct Starfield
{
	/// A quaternion discribing the 
	rotation : Quaternion,

	/// The intensity of the dullest star.
	intens_max_mag : Byte,
	/// The intensity of the brightest star.
	intens_min_mag : Byte,
	
	/// The maximum variation in intensity from the actual to real magnitude.
	intens_var : Byte,
	
	/// The brightest intensity.
	min_mag : Decimal,
	/// The dullest intensity.
	max_mag : Decimal,
	
	
	/// The maximum value of noise on the image.
	noise_max : Byte,
	/// The percentage chance of a pixel to have noise.
	chance_noise : Decimal,

	/// The size of the image.
	resolution : Cartesian2D<Decimal>,
}
