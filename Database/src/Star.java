/**
 * Stores data on a star, this corresponds to an attribute, and its location.
 * The attribute depends on the use, for a standard star, this would be magnitude.
 * For a computed star, this will be the angle from the pyramid method.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */

public class Star
{
	public double attribute;
	public Point main;


	/**
	 * Default Constructor (sets everything to 0).
	 */

	public Star ( )
	{
		attribute = 0;
		main = new Point();
	}


	/**
	 * Alternate Constructor.
	 * @param attribute_	Sets value for the attribute.
	 * @param ra_			Sets the value for pRa (right ascension).
	 * @param dec_			Sets the value for pDec (declination).
	 */

	public Star ( double attribute, double ra, double dec )
	{
		this.attribute = attribute;
		main = new Point(ra, dec);
	}



	/**
	 * Alias to main.distance().
	 * @param s		The star to compare to.
	 * @return		The distance to the other star.
	 */

	public double distance ( Star s )
	{
		return main.distance(s.main);
	}



	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	public String toString ( )
	{
		String s = attribute + ", " + main.toString();

		if ( 	!(finiteDouble(attribute) &&
		 		finiteDouble(main.ra) &&
				finiteDouble(main.dec))		)
		{
			throw new ArithmeticException("Found NaN or Inf in toString: " + s);
		}
		return s;
	}



	/**
	 * Checks if a double is a valid number.
	 * @param d	The variable to check.
	 * @return	If d is not infinite or nan.
	 */

	public static boolean finiteDouble ( double d )
	{
		return !(Double.isNaN(d) || Double.isInfinite(d));
	}

}
