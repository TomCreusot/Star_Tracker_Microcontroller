import junit.framework.TestCase;
import org.junit.*;


public class TestPoint extends TestCase
{

	@Test
	public static void testDefaultConstructor ( )
	{
		Point p = new Point();
		assertEquals("ra", 0d, p.ra, 0.001);
		assertEquals("dec", 0d, p.dec, 0.001);
	}




	@Test
	public static void testAlternateConstructor ( )
	{
		Point p = new Point(1, 2);
		assertEquals("ra", 1d, p.ra, 0.001);
		assertEquals("dec", 2d, p.dec, 0.001);
	}




	@Test
	public static void testDistance ( )
	{
		Point p1 = new Point(0, 0);
		Point p2 = new Point(3, 4);
		Point p3 = new Point(-3, -4);


		assertEquals("Equal", p1.distance(p1), 0, 0.001);
		assertEquals("Normal", p1.distance(p2), 5, 0.001);
		assertEquals("Negative", p2.distance(p3), 10, 0.001);

		try
		{
				p1.distance(new Point(Double.NaN, 0));
				assertTrue("Failed to handle nan", false);
		}
		catch ( ArithmeticException e )	{}
		try
		{
				p1.distance(new Point(Double.POSITIVE_INFINITY,
													Double.NEGATIVE_INFINITY));
				assertTrue("Failed to handle infinity", false);
		}
		catch ( ArithmeticException e )	{}
	}




	@Test
	public static void testToString ( )
	{
		Point p1 = new Point(0, 0);
		Point p2 = new Point(3, 4);
		Point p3 = new Point(-3, -4);

		assertEquals("Equal",		p1.toString(), "0.0, 0.0");
		assertEquals("Positive",	p2.toString(), "3.0, 4.0");
		assertEquals("Negative",	p3.toString(), "-3.0, -4.0");
	}





}
