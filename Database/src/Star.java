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
	// abs Magnitude/angle, Right Asscention, Declination
	public double attribute, ra, dec;


	/**
	 * Default Constructor.
	 */

	public Star ( )
	{
		attribute = 0;
		ra = 0;
		dec = 0;
	}


	/**
	 * Alternate Constructor.
	 * @param attribute_	Sets value for the attribute.
	 * @param ra_			Sets the value for ra (right ascension).
	 * @param dec_			Sets the value for dec (declination).
	 */

	public Star ( double attribute_, double ra_, double dec_ )
	{
		attribute = attribute_;
		ra = ra_;
		dec = dec_;
	}


	/**
	 * Constructs the class by solving the attribute.
	 * This is simmilar to ../src/StarTracker/StarTracker.cpp: findAngle().
	 *
	 * @param pilot		The brightest star.
	 * @param s1		A random star.
	 * @param s2		An other random star.
	 * @param s3		An other other random star.
	 */

	public Star ( Star pilot, Star s1, Star s2, Star s3 )
	{
		//cosine rule: A = acos((b^2 + c^2 - a^2) / 2bc)
		//a is farthest node from pilot.
		double hyp = Math.hypot(s1.ra - pilot.ra, s1.dec - pilot.dec);
		double adj = Math.hypot(s2.ra - pilot.ra, s2.dec - pilot.dec);
		double opp = Math.hypot(s3.ra - pilot.ra, s3.dec - pilot.dec);

		//Assume node1 is a, node2 is b, node3 is c.
		double a = Math.hypot(s2.ra - s3.ra, s2.dec - s3.dec);
		double b = Math.hypot(s1.ra - s2.ra, s1.dec - s2.dec);
		double c = Math.hypot(s1.ra - s3.ra, s1.dec - s3.dec);

		//If our assumption is wrong, swap.
		if (adj > hyp && adj > opp)
		{
			double temp = a;
			a = b;
			b = temp;
		}
		else if (opp > hyp)
		{
			double temp = a;
			a = c;
			c = temp;
		}

		ra = pilot.ra;
		dec = pilot.dec;
		attribute = Math.acos((b * b + c * c - a * a) / (2d * b * c));
	}




	/**
	* Finds the distance from the specified star (in degrees?).
	*
	* @param pilot	The star to compare to.
	* @return		The distance in the current units.
	*/

	public double distanceFromPilot ( Star pilot )
	{
		return Math.hypot(ra - pilot.ra, dec - pilot.dec);
	}




	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	public String toCSVString ( )
	{
		return attribute + "," + ra + "," + dec;
	}


	/**
	 * Provides an easy to read version of the class.
	 * @return The string.
	 */

	 @Override
	 public String toString ( )
	 {
		 return attribute + ", " + ra + ", " + dec;
	 }
}
