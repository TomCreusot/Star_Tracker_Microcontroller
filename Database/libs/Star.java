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
	// apperant Magnitude/angle, Right Asscention, Declination
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
	* Alternate Constructor.
	* This constructor is only to aid the testing of difference classes.
	* @param a  What all the values should be
	*/

	public Star ( double a )
	{
		attribute = a;
		ra = a;
		dec = a;
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

		double hyp = s1.distanceFromPilot(pilot);
		double adj = s2.distanceFromPilot(pilot);
		double opp = s3.distanceFromPilot(pilot);

		// a is node2 to node3
		double a = s2.distanceFromPilot(s3);
		// b is node1 to node3
		double b = s1.distanceFromPilot(s3);
		// c is node1 to node2
		double c = s1.distanceFromPilot(s2);

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
	* Returns an array of bytes representing this.
	* @param numAtt 	The number of bytes for the attribute.
	* @param numRa	The number of bytes for the right ascension.
	* @param numDec	The number of bytes for the declination.
	*/
/*
	public byte[] toBinary ( int numAtt, int numRa, int numDec )
	{
		byte [] array = new byte[numAtt + numRa + numDec];
		for ( int i = 0; i < numAtt; i++ )
		{

		}
		for ( int i = 0; i < numRa; i++ )
		{

		}
		for ( int i =0; i < numDec; i++ )
		array[]
	}
*/



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
