
public abstract class UnitTest
{
	// Displays the test number.
	public static int testNum = 0;
	public static int testsPassed = 0;


	//Unix color codes
	public static final String ANSI_RESET = "\u001B[0m";
	public static final String ANSI_RED = "\u001B[31m";
	public static final String ANSI_GREEN = "\u001B[32m";
	public static final String ANSI_WHITE = "\u001B[37m";

	public static final String ANSI_BLACK_BACKGROUND = "\u001B[40m";
	public static final String ANSI_RED_BACKGROUND = "\u001B[41m";


	/**
	 * Runs the unit test.
	 * @return True if all the tests passed
	 */

	public static boolean run ( )
	{
		return false;
	}



	protected static void printHeader ( String className, String testName )
	{
		System.out.println("\n\n-------  " + className + " | " + testName + "  ------------------");
	}





	protected static boolean printPass ( String test, boolean pass )
	{
		System.out.println("Test: " + testNum + ", \n" + test);

		if ( pass )	System.out.println(ANSI_GREEN + "PASSED" + ANSI_RESET);
		else		System.out.println(ANSI_RED_BACKGROUND + ANSI_WHITE + "FAILED" + ANSI_RESET);
		System.out.println();
		testNum++;
		testsPassed += (pass ? 1 : 0);
		return pass;
	}



	/**
	* Eases comparisons.
	* @param s A star.
	* @param d A double.
	* @return true if equal.
	*/

	protected static boolean equal ( Star s, double d )
	{
		return Math.abs(s.attribute - d) < 0.001d;
	}






	/**
	* Eases comparisons.
	* @param s A double.
	* @param d A double.
	* @return true if equal.
	*/

	protected static boolean equal ( double d1, double d2 )
	{
		return Math.abs(d1 - d2) < 0.001d;
	}

}
