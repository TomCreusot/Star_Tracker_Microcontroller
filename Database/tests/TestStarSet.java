import junit.framework.TestCase;
import org.junit.*;


public class TestStarSet extends TestCase
{
	@Test
	public static void testDefaultConstructor ( )
	{
		StarSet set = new StarSet();
		assertEquals("attribute", 0.0d, set.attribute, 0.0001d);
		assertEquals("pilot", new Point(), set.main);
		assertEquals("opposite", new Point(), set.opposite);
	}


	@Test
	public static void testAlternateConstructor ( )
	{
		StarSet set = new StarSet(123, 1, 2, 3, 4);
		assertEquals("attribute", 123, set.attribute, 0.0001d);
		assertEquals("pilot", new Point(1, 2), set.main);
		assertEquals("opposite", new Point(3, 4), set.opposite);
	}

	@Test
	public static void testAlternateConstructor2 ( )
	{
		Star p = new Star(0, 0, 1);
		Star o = new Star(0, 10000, 1);
		Star s1 = new Star(0, -10, 11);
		Star s2 = new Star(0, -23.22, 1001);
		StarSet set = new StarSet(p, o, s1, s2);
		assertEquals("attribute", 0.098440278, set.attribute, 0.000001);
		assertEquals("opposite", o.main, set.opposite);
		assertEquals("pilot", p.main, set.main);


		set = new StarSet(p, s1, s2, o);
		assertEquals("unordered attribute", 0.098440278, set.attribute, 0.0001);
		assertEquals("unordered opposite", o.main, set.opposite);
		assertEquals("unordered pilot", p.main, set.main);
	}


	@Test
	public static void testToString ( )
	{
		StarSet s1 = new StarSet(123, 1, 2, 3, 4);
		StarSet s2 = new StarSet(-123, -1, -2, -3, -4);
		StarSet s3 = new StarSet(0, 0, 0, 0, 0);
		StarSet s4 = new StarSet(Double.NaN, Double.NaN, Double.NaN,
														Double.NaN, Double.NaN);
		StarSet s5 = new StarSet(Double.POSITIVE_INFINITY,
							Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY,
							Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY);
		StarSet s6 = new StarSet(Double.NaN, 0, 0, 0, 0);

		assertEquals("Normal", "123.0, 1.0, 2.0, 3.0, 4.0", s1.toString());
		assertEquals("Negative","-123.0, -1.0, -2.0, -3.0, -4.0",s2.toString());
		assertEquals("Zero", "0.0, 0.0, 0.0, 0.0, 0.0", s3.toString());

		try
		{
			s4.toString();
			s5.toString();
			s6.toString();
			assertTrue("nan, inf", false);
		}
		catch ( ArithmeticException e ) {}
	}



	@Test
	public static void testSortFuthest ( )
	{
		Star p = new Star(55, 0, 0);
		Star s0 = new Star(66, 100, 130);
		Star s1 = new Star(77, -10, -123);
		Star s2 = new Star(88, 0, 0);

		Star[] set = StarSet.sortFuthest(p, s0, s1, s2);

		assertEquals("Already Sorted s0", s0.attribute, set[0].attribute, 0.1d);
		assertEquals("Already Sorted s1", s1.attribute, set[1].attribute, 0.1d);
		assertEquals("Already Sorted s2", s2.attribute, set[2].attribute, 0.1d);

		set = StarSet.sortFuthest(p, s0, s2, s1);

		assertEquals("Already Sorted_s0", s0.attribute, set[0].attribute, 0.1d);
		assertEquals("Already Sorted_s1", s1.attribute, set[2].attribute, 0.1d);
		assertEquals("Already Sorted_s2", s2.attribute, set[1].attribute, 0.1d);


		set = StarSet.sortFuthest(p, s1, s0, s2);

		assertEquals("Swapped 1,0 s0", s0.attribute, set[0].attribute, 0.1d);
		assertEquals("Swapped 1,0 s1", s1.attribute, set[1].attribute, 0.1d);
		assertEquals("Swapped 1,0 s2", s2.attribute, set[2].attribute, 0.1d);


		set = StarSet.sortFuthest(p, s2, s1, s0);

		assertEquals("Swapped 2,0 s0", s0.attribute, set[0].attribute, 0.1d);
		assertEquals("Swapped 2,0 s1", s1.attribute, set[1].attribute, 0.1d);
		assertEquals("Swapped 2,0 s2", s2.attribute, set[2].attribute, 0.1d);
	}



	@Test
	public static void testFindAngle ( )
	{
		Star o = new Star(1, 0, 0);
		Star b = new Star(2, 0, 0);
		Star c = new Star(3, 0, 0);
		double angle;

		try
		{
			angle = StarSet.findAngle(o, b, c);
			assertTrue("undefined", false);
		}
		catch ( ArithmeticException e ) {}

		/*	o		b/c
		 */
		o = new Star(1, -10, 0);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("0 angle", 0, angle, 0.00001d);


		/*
		 *	b
		 *
		 *	o		c
		 */

		b = new Star(2, -10, 10);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("90 angle", Math.PI / 2, angle, 0.00001d);


		/*
		 *	b
		 *
		 *	o
		 *	c
		 */

		c = new Star(3, -10, -5);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("180 angle", Math.PI, angle, 0.00001d);

		/*
		 * o = -10, 0
		 * b = -10, 10
		 * c = 0, 10
		 *
		 *	b		c
		 *
		 *	o
		 */

		c = new Star(3, 0, 10);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("45 angle", Math.PI / 4, angle, 0.00001d);

		o = new Star(1, -10, 0);
		b = new Star(1, -10, 10);
		c = new Star(3, -23.22, 8.21);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("random decimal angle 1", 1.015057844, angle, 0.00001d);

		o = new Star(1, 10000, 0);
		b = new Star(1, -10, 10);
		c = new Star(3, -23.22, 8.21);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("random decimal angle 2", 0.000179903, angle, 0.00001d);


		o = new Star(1, 10000, 0);
		b = new Star(1, -10, 10);
		c = new Star(3, -23.22, 100000);
		angle = StarSet.findAngle(o, b, c);
		assertEquals("random decimal angle 3", 1.469898778, angle, 0.00001d);

	}
}
