import junit.framework.TestCase;
import org.junit.*;


public class TestStar extends TestCase
{
	@Test
	public static void testDefaultConstructor ( )
	{
		Star star = new Star();
		Point p = new Point();
		assertEquals("attribute", 0d, star.attribute, 0.001);
		assertEquals("main", p, star.main);
	}



	@Test
	public static void testAlternateConstructor ( )
	{
		Star star = new Star(123, 4, 3);
		Point p = new Point(4, 3);
		assertEquals("attribute", 123d, star.attribute, 0.001);
		assertEquals("main", p, star.main);
	}


	@Test
	public static void testToString ( )
	{
		Star s1 = new Star(123, 4, 3);
		Star s2 = new Star(-123, -4, -3);
		Star s3 = new Star(0, 0, 0);
		Star s4 = new Star(0, Double.NaN, Double.POSITIVE_INFINITY);
		assertEquals("Normal", "123.0, 4.0, 3.0", s1.toString());
		assertEquals("Negative", "-123.0, -4.0, -3.0", s2.toString());
		assertEquals("Zero", "0.0, 0.0, 0.0", s3.toString());

		try
		{
			s4.toString();
			assertTrue("nan, inf", false);
		}
		catch ( ArithmeticException e ) {}
	}

	@Test
	public static void testFiniteDouble ( )
	{
		assertTrue("0",			Star.finiteDouble(0d));
		assertTrue("negative",	Star.finiteDouble(-10000));
		assertTrue("positive",	Star.finiteDouble(10000));
		assertFalse("+infinite", Star.finiteDouble(Double.POSITIVE_INFINITY));
		assertFalse("-infinite", Star.finiteDouble(Double.NEGATIVE_INFINITY));
		assertFalse("nan",		Star.finiteDouble(Double.NaN));
	}

}
