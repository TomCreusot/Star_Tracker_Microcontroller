/*
 *	This file is designed to test /Database/src/Star.java
 *
 *	It tests the methods:
 *		- distanceFromPilot
 *		- toCSVString
 *		- Alternate Constructor
 *
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */

public class StarTest extends UnitTest
{

	/**
	 * Call to test Tree.java
	 * @return True if all tests pass.
	 */

	public static boolean run ( )
	{
		printHeader("Star.java", "StarTest.java");
		boolean test = true;
		test &= printPass("distanceFromPilot", testDistanceFromPilot());
		test &= printPass("testAlternateConstructor", testAlternateConstructor());

		return test;
	}


	/**
	 * Tests distance from pilot.
	 *
	 *		- Standard 3, 4, 5 triangle
	 *		- Negative numbers
	 *		- sqrt(0)
	 *
	 * @return True if all tests pass.
	 */

	public static boolean testDistanceFromPilot ( )
	{
		boolean valid = true;
		Star star, pilot;

		// 3, 4, 5 triangle
		// (3 - 0)ra, (4 - 0)dec
		star = new Star(10, 3, 4);
		pilot = new Star(100, 0, 0);
		valid &= 5 == star.distanceFromPilot(pilot);

		// (10 - 7)ra, (100 - 96)dec
		star = new Star(10, 10, 100);
		pilot = new Star(100, 7, 96);
		valid &= 5 == star.distanceFromPilot(pilot);

		// (-1 - 2)ra, (-3 - 1)dec
		star = new Star(10, -1, -3);
		pilot = new Star(100, 2, 1);
		valid &= 5 == star.distanceFromPilot(pilot);


		// 0 0 0 triangle
		// (0 - 0)ra, (0 - 0)dec
		star = new Star(10, 0, 0);
		pilot = new Star(100, 0, 0);
		valid &= 0 == star.distanceFromPilot(pilot);
		return valid;
	}




	/**
	 * Tests toCSVString so that the csv read/write is correct.
	 *
	 * @return True if all tests pass.
	 */

	public static boolean testToCSVString ( )
	{
		boolean valid = true;
		Star star = new Star(0, -0.0000000003d, 10000000d);

		String[] csv = star.toCSVString().split(",");

		valid &= equal(star.attribute, 	Double.parseDouble(csv[0]));
		valid &= equal(star.ra, 		Double.parseDouble(csv[1]));
		valid &= equal(star.dec, 		Double.parseDouble(csv[2]));
		return valid;
	}




	/**
	 * Tests Alternate Constructor (Star pilot, Star s1, Star s2, Star s3)
	 *
	 * @return True if all tests pass.
	 */

	 public static boolean testAlternateConstructor ( )
	 {
		 boolean valid = true;
		 Star pilot = new Star(0);
		 Star hypot = new Star(0, 0, 10);
		 Star adjac = new Star(0, -4, 7);
		 Star oppos = new Star(0, 8, 4);

		 Star pyramid = new Star(pilot, hypot, adjac, oppos);
		 valid &= equal(pyramid, 1.8545904360032242d);
		 return valid;
	 }
}
