/**
 * Stores:							\n
 *		- An angle.					\n
 *		- The pilot coordinates.	\n
 *		- The opposites coordinates.\n
 *
 * This allows for storing all relivant infomation on the pyramid method.
 *
 * Using subclassing will help for polymorphism through sorting etc.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		26/2/2020
 */

class StarSet extends Star
{

	// The star which holds the angle.
	public Point opposite;



		/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
		|															 |
------ -----		-----	Constructs	----				----- -------
		|															 |
		\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

	/**
	 *	Default Constructor (sets everything to 0).
	 */

	 public StarSet ( )
	 {
		 super();
		 opposite = new Point();
	 }


	/**
	 *	Alternate Constructor.
	 *	@param attribute	The angle.
	 *	@param pilotRa		The pilots right ascension.
	 *	@param pilotDec		The pilots declination.
	 *	@param oppositeRa	The opposite star's right ascention.
	 *	@param oppositeDec	The opposite star's declination.
	 */

	public StarSet ( 	double attribute,
						double pilotRa, double pilotDec,
						double oppositeRa, double oppositeDec )
	{
		super(attribute, pilotRa, pilotDec);
		opposite = new Point(oppositeRa, oppositeDec);
	}



	/**
	 *	Alternate Constructor.
	 *	Automticaly finds the angle and opposite star.
	 *
	 *	@param pilot	The brightest star.
	 *	@param s0		Another star.
	 *	@param s1		Another other star.
	 *	@param s2		Another other other star.
	 */

	 public StarSet ( Star pilot, Star s0, Star s1, Star s2 )
	 {
		 super();
		 Star[] s = sortFuthest(pilot, s0, s1, s2);
		 attribute = findAngle(s[0], s[1], s[2]);
		 main = pilot.main;
		 opposite = s[0].main;
	 }



	/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
	|															 |
------ -----			-----	Other	----				----- -------
	|															 |
	\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	@Override
	public String toString ( )
	{
		String s = super.toString() + ", " + opposite.toString();

		if ( 	s.equals("") ||
				!(finiteDouble(opposite.ra) || finiteDouble(opposite.dec)) )
		{
			throw new ArithmeticException("Found NaN or Inf in toString: " + s);
		}

		return s;
	}






		/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
		|															 |
------ -----			-----	Static Methods	----			----- -------
		|															 |
		\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/





	/**
	 * Creates an array that contains the futhest star from the pilot at [0].
	 * @param pilot	The reference to get distances.
	 * @param s0	The first star to compare.
	 * @param s1	The second star to compare.
	 * @param s2	The third star to compare.
	 * @return 		The stars with the futhest first.
	 */

	public static Star[] sortFuthest ( Star pilot, Star s0, Star s1, Star s2 )
	{
		Star [] stars = new Star[3];

		double dist1 = s0.distance(pilot);
		double dist2 = s1.distance(pilot);
		double dist3 = s2.distance(pilot);

		if ( dist1 > dist2 && dist1 > dist3 )
		{
			stars[0] = s0;
			stars[1] = s1;
			stars[2] = s2;
		}
		else if ( dist2 > dist3 )
		{
			stars[0] = s1;
			stars[1] = s0;
			stars[2] = s2;
		}
		else
		{
			stars[0] = s2;
			stars[1] = s1;
			stars[2] = s0;
		}
		return stars;
	}



	/**
	 * Finds the angle between 2 stars and the futhest from the pilot.
	 * @param opp		The star futhest from the pilot.
	 * @param s1		The other star.
	 * @param s2		The other other star.
	 * @return			The angle between s2 \ opposite / s3
	 */

	public static double findAngle ( Star opp, Star s1, Star s2 )
	{
		//cosine rule: A = acos((b^2 + c^2 - a^2) / 2bc)

		// a is 2 to 3
		double a = s1.distance(s2);
		// b is 1 to 3
		double b = opp.distance(s2);
		// c is 1 to 2
		double c = opp.distance(s1);

		double angle = Math.acos((b * b + c * c - a * a) / (2d * b * c));

		if ( !finiteDouble(angle) )
							throw new ArithmeticException("undefined angle");

		return angle;
	}



	/**
	 * Finds the distance the opposite star is from the center.
	 * @param opposite	The star futhest from the pilot.
	 * @param s1		The other star.
	 * @param s2		The other other star.
	 * @return			The distance.
	 */

	public static void distanceFromCenter ( Star opposite, Star s1, Star s2)
	{
	}

}
