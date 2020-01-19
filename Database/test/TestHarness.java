

public class TestHarness
{

	/**
	 * Tests all the unit tests together.
	 * To add a test, ignore the make file, just put the call here.
	 */

	public static void main ( String [] args )
	{
		TreeTest.run();
		StarTest.run();

		System.out.println("\n\n" + UnitTest.testsPassed + " of " + UnitTest.testNum + " passed\n\n");
	}



}
