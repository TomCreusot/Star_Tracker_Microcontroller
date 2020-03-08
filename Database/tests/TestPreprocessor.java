import java.util.*;

import junit.framework.TestCase;
import org.junit.*;


public class TestPreprocessor extends TestCase
{
	@Test
	public static void testRun ( )
	{
	assertTrue(false);
	}

	@Test
	public static void testGetAngles ( )
	{
		assertTrue(false);
	}



	@Test
	public static void testFindClosestBrightest ( )
	{
		double fov = 5;
		Star pilot = new Star(-100, 1, 0);
		LinkedList<Star> stars = new LinkedList<Star>();
		stars.add(	new Star(-10, 6, 5)		);	// INVALID:	Outside range.
		stars.add(	new Star(0, 4, 3)		);	// VALID:	Normal.
		stars.add(	new Star(2, 10, 5)		);	// INVALID:	Above bounds.
		stars.add(	new Star(3, 6, 0)		);	// INVALID:	On Bounds.
		stars.add(	new Star(6, 1, 0)		);	// VALID:	On position.
		stars.add(	new Star(5, 5.9, 0)		);	// VALID:	Bellow Bounds.
		stars.add(	new Star(100, 32, 32)	);	// INVALID:	Way Off

		// Default
		int numStars = 3;
		Star[] val = Preprocessor.findClosestBrightest(pilot, stars,
																numStars, fov);
		assertEquals(val.length, 3);
		assertEquals(val[0].attribute, 0, 0.01);
		assertEquals(val[1].attribute, 6, 0.01);
		assertEquals(val[2].attribute, 5, 0.01);

		// Not enough valid
		numStars = 4;
		val = Preprocessor.findClosestBrightest(pilot, stars, numStars, fov);
		assertTrue("When numStars exceed valid, should return null", val==null);
	}



	@Test
	public static void testFindCombinations ( )
	{
		assertTrue(false);
	}











	//////////////////////////////////////////////////////////////////////////
	// 																		//
	// 																		//
	// 						Sorting and Converting							//
	// 																		//
	//																		//
	//////////////////////////////////////////////////////////////////////////


	@Test
	public static void testPreprocess ( )
	{
		double cutoff = 10;
		LinkedList<String> lines = new LinkedList<String>();
		lines.add("1,2,3");
		lines.add("2.1 , 2 , 3");
		lines.add("3 , -10000 , 3");
		lines.add("4 , NAN , 3");
		lines.add("-5 , -10000 , 3");
		lines.add("cutoff , 0 , 3");
		lines.add("10 , -1 , -3");
		lines.add("NAN , -1 , -3");
		lines.add("0 , -1 , -3");
		LinkedList<Star> processed = Preprocessor.preprocess(lines, cutoff);
		assertEquals("basic", processed.removeFirst().attribute, 1, 0.1);
		assertEquals("spaces", processed.removeFirst().attribute, 2.1, 0.01);
		assertEquals("large negative",
							processed.removeFirst().main.ra, -10000, 0.1);

		assertEquals("-'ve' number",processed.removeFirst().attribute, -5, 0.1);
		assertEquals("standard",processed.removeFirst().attribute, 0, 0.1);
		assertEquals("should be empty", processed.size(), 0);
	}



	@Test
	public static void testSortStar ( )
	{
		LinkedList<Star> star = new LinkedList<Star>();
		star.add(	new Star(5, 5, 5)		);
		star.add(	new Star(1.1, 1, 1)		);
		star.add(	new Star(-1, -1, -1)	);
		star.add(	new Star(0.9, 1, 1)		);
		star.add(	new Star(0, 0, 0)		);
		star.add(	new Star(1, 1, 1)		);
		Preprocessor.sortStar(star);

		assertEquals(star.removeFirst().attribute, -1, 0.01);
		assertEquals(star.removeFirst().attribute, 0, 0.01);
		assertEquals(star.removeFirst().attribute, 0.9, 0.01);
		assertEquals(star.removeFirst().attribute, 1, 0.01);
		assertEquals(star.removeFirst().attribute, 1.1, 0.01);
		assertEquals(star.removeFirst().attribute, 5, 0.01);
		assertEquals(star.size(), 0);
	}



	@Test
	public static void testSortSet ( )
	{
		LinkedList<Star> star = new LinkedList<Star>();
		star.add(	new StarSet(5, 5, 5, 5, 5)			);
		star.add(	new StarSet(1.1, 1, 1, 1, 1)		);
		star.add(	new StarSet(-1, -1, -1, -1, -1)		);
		star.add(	new StarSet(0.9, 1, 1, 1, 1)		);
		star.add(	new StarSet(0, 0, 0, 0, 0)			);
		star.add(	new StarSet(1, 1, 1, 1, 1)			);
		Preprocessor.sortStar(star);

		assertEquals(star.removeFirst().attribute, -1, 0.01);
		assertEquals(star.removeFirst().attribute, 0, 0.01);
		assertEquals(star.removeFirst().attribute, 0.9, 0.01);
		assertEquals(star.removeFirst().attribute, 1, 0.01);
		assertEquals(star.removeFirst().attribute, 1.1, 0.01);
		assertEquals(star.removeFirst().attribute, 5, 0.01);
		assertEquals(star.size(), 0);
	}



	@Test
	public static void testSetToString ( )
	{
		LinkedList<StarSet> stars = new LinkedList<StarSet>();
		stars.add(new StarSet(-1, -2, -3, -4, -5));
		stars.add(new StarSet(0, 0, 0, 0, 0));
		stars.add(new StarSet(1, 2, 3, 4, 5));
		stars.add(new StarSet(2, 3, 4, 5, 6));
		stars.add(new StarSet(3, 4, 5, 6, 7));
		stars.add(new StarSet(4, 5, 10000, -1, 2));

		LinkedList<String> str = Preprocessor.setToString(stars);

		assertEquals(str.removeFirst(), "-1.0, -2.0, -3.0, -4.0, -5.0");
		assertEquals(str.removeFirst(), "0.0, 0.0, 0.0, 0.0, 0.0");
		assertEquals(str.removeFirst(), "1.0, 2.0, 3.0, 4.0, 5.0");
		assertEquals(str.removeFirst(), "2.0, 3.0, 4.0, 5.0, 6.0");
		assertEquals(str.removeFirst(), "3.0, 4.0, 5.0, 6.0, 7.0");
		assertEquals(str.removeFirst(), "4.0, 5.0, 10000.0, -1.0, 2.0");
	}

}
