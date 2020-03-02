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
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	public String toCSVString ( )
	{
		return ra + "," + dec;
	}
}
