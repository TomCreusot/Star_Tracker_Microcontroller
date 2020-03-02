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
	 * Constructs the class by solving the attribute.
	 *
	 * @param pilot		The brightest star.
	 * @param s1		A random star.
	 * @param s2		An other random star.
	 * @param s3		An other other random star.
	 */

	public StarSet ( Star pilot, Star s1, Star s2, Star s3 )
	{
		super();
		super.pilot.ra = pilot.pilot.ra;
		super.pilot.dec = pilot.pilot.dec;
		opposite = new Point();

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

		opposite = new Point(s1.pilot);

		//If our assumption is wrong, swap.
		if (adj > hyp && adj > opp)
		{
			double temp = a;
			a = b;
			b = temp;
			opposite = new Point(s2.pilot);

		}
		else if (opp > hyp)
		{
			double temp = a;
			a = c;
			c = temp;
			opposite = new Point(s3.pilot);
		}

		super.attribute = Math.acos((b * b + c * c - a * a) / (2d * b * c));
	}


	/**
	* Provides a csv formatted string.
	* @return A valid csv row string.
	*/

	@Override
	public String toCSVString ( )
	{
		return super.toCSVString() + "," + opposite.toCSVString();
	}

}
