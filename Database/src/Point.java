/**
 * Stores the right asscention and declination of a star.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		27/2/2020
 */


public class Point
{
	double ra, dec;

	/**
	 *	Default Constructor (set to 0).
	 */

	public Point ( )
	{
		ra = 0;
		dec = 0;
	}



	/**
	 *		Alternate Constructor
	 *		@param ra	The right ascension.
	 *		@param dec	The declination.
	 */

	public Point ( double ra, double dec )
	{
		this.ra = ra;
		this.dec = dec;
	}



	/**
	 *		Copy Constructor.
	 *		@param val	The point to copy.
	 */

	public Point ( Point val )
	{
		this.ra = val.ra;
		this.dec = val.dec;
	}





	/**
	* Finds the distance between the points.
	*
	* @param other	The point to compare to.
	* @return		The distance in the current units.
	*/

	public double distance ( Point other )
	{
		double val = Math.hypot( other.ra - ra, other.dec - dec );
		if ( Double.isInfinite(val) || Double.isNaN(val)  )
		{
			throw new ArithmeticException("Cannot calculate distance: " +
			" ra: " + other.ra + ", " + ra + "\tdec: " + other.dec + "," + dec);
		}
		return val;
	}



	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	public String toString ( )
	{
		return ra + ", " + dec;
	}




	/**
	 * The Object equals method.
	 * @param obj 	The object to compare to.
	 * @return 		True if equal.
	 */

	public boolean equals ( Object obj )
	{
		if ( obj instanceof Point )
		{
			Point p = (Point) obj;
			double t = 0.000001d;
			return Math.abs(p.ra - ra) < t && Math.abs(p.dec - dec) < t;
		}
		return false;
	}
}
