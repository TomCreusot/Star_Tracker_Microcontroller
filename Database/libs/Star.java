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
	public Point pilot;


	/**
	 * Default Constructor (sets everything to 0).
	 */

	public Star ( )
	{
		attribute = 0;
		pilot = new Point();
	}


	/**
	 * Alternate Constructor.
	 * @param attribute_	Sets value for the attribute.
	 * @param ra_			Sets the value for pRa (right ascension).
	 * @param dec_			Sets the value for pDec (declination).
	 */

	public Star ( double attribute, double ra, double dec )
	{
		attribute = attribute;
		pilot = new Point(ra, dec);
	}




	/**
	* Finds the distance from the specified star (in degrees?).
	*
	* @param pilot	The star to compare to.
	* @return		The distance in the current units.
	*/

	public double distanceFromPilot ( Star pilot )
	{
		return Math.hypot(	this.pilot.ra - pilot.pilot.ra,
							this.pilot.dec - pilot.pilot.dec	);
	}




	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	public String toCSVString ( )
	{
		return attribute + "," + pilot.toCSVString();
	}
}
