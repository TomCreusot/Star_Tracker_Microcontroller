import org.junit.*;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;
import static org.junit.Assert.*;

@RunWith(JUnit4.class)

public class StarsTest
{
	@Test
	public void testDistanceFromPilot ( )
	{
		// 3, 4, 5 triangle
		// (3 - 0)ra, (4 - 0)dec
		star = new Star(10, 3, 4);
		pilot = new Star(100, 0, 0);
		assertEquals(5, star.distanceFromPilot(pilot));

		// (10 - 7)ra, (100 - 96)dec
		star = new Star(10, 10, 100);
		pilot = new Star(100, 7, 96);
		assertEquals(5, star.distanceFromPilot(pilot));

		// (-1 - 2)ra, (-3 - 1)dec
		star = new Star(10, -1, -3);
		pilot = new Star(100, 2, 1);
		assertEquals(5, star.distanceFromPilot(pilot));


		// 0 0 0 triangle
		// (0 - 0)ra, (0 - 0)dec
		star = new Star(10, 0, 0);
		pilot = new Star(100, 0, 0);
		assertEquals(0, star.distanceFromPilot(pilot));
	}


}
