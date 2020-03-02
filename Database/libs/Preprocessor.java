/**
 *	Converts a database of mag,ra,dec to angle,ra,dec.
 *
 *	Reference:
 *		The database ideal for use is: @see<a href="http://www.astronexus.com/hyg"</a>.
 *		The readme will give valuable infomation about the use of this program.
 *
 * Requires:
 *		A csv file with 3 columns containing in order:
 *			apperant magnitude, right asscention, declination.
 *		The first star is assumed to be headers so it will not be included.
 *
 * Note:
 *		The Right Ascension and Declination are expressed as angles, however
 *		they can be calculated as vectors without needing conversion
 *		as long as they are only compared with other angles.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */

import java.util.*;


/**
 * This class does as the description in README.md.
 */

public class Preprocessor
{
	// Bash Colors
	public static final String NORMAL = "\u001B[0m";
	public static final String BOLD = "";
	public static final String GREEN = "\u001B[32m";



	/**
	 *	A sample program.
	 *	@param args The command line arguments in order of:
	 *		- The database file.
	 *		- The output file.
	 *		- The fov.
	 *		- The magnitude cutoff.
	 *		- The number of stars surrounding the pilot.
	 */

	public static void main ( String [] args )
	{
		if ( args.length == 5)
		{
			String databaseName		= args[0];
			String outputName 		= args[1];
			double fov 				= Double.parseDouble(args[2]);
			double maxMag			= Double.parseDouble(args[3]);
			int pilotSets			= Integer.parseInt(args[4]);

			System.out.println("Reading: ");
			LinkedList<String> lines = FileIO.readFile(databaseName);
			lines.removeFirst(); // Header
			System.out.println("Processing: ");
			LinkedList<Star> processed = preprocess(lines, maxMag);
			System.out.println("Sorting: ");
			sortStar(processed);
			System.out.println("Calculating: ");
			LinkedList<StarSet> values = getAngles(processed, pilotSets, fov);
			System.out.println("Wrapping: ");
			lines = postProcess(values);

			FileIO.writeFile(outputName, lines);
		}
		else
		{
			System.out.println
			(
				"Error, Please enter: \n" +
				"\t- The database to read from.\n" +
				"\t- The name of the file to write to.\n" +
				"\t- The feild of view of the camera.\n" +
				"\t- The maximum magnitude to permit.\n" +
				"\t- The number of stars to sample around the pilot.\n"
			);
		}
	}




	/**
	 * Converts the lines to decimal values and removes all which are invalid.
	 * @param lines		The lines of the file.
	 * @param cutOff	The maximum magnitude permitted.
	 * @return The stars derived from the lines.
	 */

	public static LinkedList<Star> preprocess
									( LinkedList<String> lines, double cutOff )
	{
		LinkedList<Star> list = new LinkedList<Star>();
		ListIterator<String> it = lines.listIterator();

		while ( it.hasNext() )
		{
			String [] stars = it.next().split(",");
			double mag = Double.parseDouble(stars[0]);
			if ( mag < cutOff )
			{
				Star s = new Star(mag, 	Double.parseDouble(stars[1]),
										Double.parseDouble(stars[2]));
				list.add(s);
			}
		}
		return list;
	}




	/**
	 * Calls java comparator sort to sort the list.
	 * @param list	The list to sort.
	 */

	public static void sortStar ( LinkedList<Star> list )
	{
		Collections.sort(list, new Comparator<Star>()
		{
			@Override
			public int compare ( Star o1, Star o2 )
			{
				double val = (o1.attribute - o2.attribute);
				return (val > 0 ? 1 : -1);
			}
		});
	}




	/**
	 * Calls java comparator sort to sort the list for star sets (polymorphism does not work with generics?).
	 * @param list	The list to sort.
	 */

	public static void sortSet ( LinkedList<StarSet> list )
	{
		Collections.sort(list, new Comparator<StarSet>()
		{
			@Override
			public int compare ( StarSet o1, StarSet o2 )
			{
				// System.out.println(o1.attribute + "  " + o2.attribute);
				double val = (o1.attribute - o2.attribute);
				//if (Math.abs(val) < 0.000001) System.out.println(val);
				return (val < 0 ? 1 : -1);
			}
		});
	}




	/**
	 *	Converts the list of stars back into a text array and sorts.
	 *	@param stars 	The stars to convert.
	 *	@return			The csv description of the stars.
	 */

	public static LinkedList<String> postProcess ( LinkedList<StarSet> stars )
	{
		LinkedList<String> strings = new LinkedList<String>();
		sortSet( stars );
		Iterator<StarSet> it = stars.listIterator();
		while ( it.hasNext() )
		{
			strings.add(it.next().toCSVString());
		}
		return strings;
	}




	/**
	 * Returns the angles.
	 *
	 * @param stars 		The magnitude, ra, dec of all the stars to check.
	 * @param pilotSets		The number of stars surrounding the pilot to use.
	 * @param radiusPilot	The angle distance between the pilot and the chosen stars.
	 * @return Every possible star set.
	 */

	public static LinkedList<StarSet> getAngles ( LinkedList<Star> stars,
											int pilotSets, double radiusPilot )
	{
		LinkedList<StarSet> output = new LinkedList<StarSet>();
		// Max number of stars
		System.out.println(stars.size() + " to go.");

		while ( stars.size() > 0 )
		{
			// Always the brightest.
			Star pilot = stars.getFirst();

			// Gets all required stars to calculate angle for.
			Star[] s=findClosestBrightest(pilot, stars, pilotSets, radiusPilot);

			// If there were enough stars in the range, find all angles.
			if ( s != null )
			{
				combinations(pilot, s, output);
			}
			// Remove from queue making search parameters smaller.
			stars.removeFirst();

			// Every 100 stars show progress.
			if( stars.size() % 100 == 0) System.out.println(stars.size() + " to go.");
		}
		return output;
	}




	/**
	 * Finds the brightest stars within the specified bounds from the pilot star.
	 *
	 * @param pilot		The pilot star.
	 * @param stars 	The stars to observe.
	 * @param num		The number of stars to add.
	 * @param radius	The maximum distance the stars can be from the pilot.
	 * @return			An array of the closest stars OR null if not all values found.
	 */

	public static Star[] findClosestBrightest ( Star pilot, LinkedList<Star> stars, int num, double radius )
	{
		Star[] close = new Star[num];
		ListIterator<Star> it = stars.listIterator();
		boolean found = true;
		Star cur = null;

		it.next(); // This is the pilot.
		for ( int i = 0; i < num && found; i++ )
		{
			found = false;

			// Goes from the brightest star trying to find one which fulfils the bounds.
			while ( it.hasNext() && !found )
			{
				cur = it.next();
				found |= ( cur.distanceFromPilot(pilot) < radius );
			}
			if ( found ) close[i] = cur;
		}

		if (!found) close = null;

		return close;
	}




	/**
	 * Solves all the combinations of stars in the sample group.
	 * Similar to This is simmilar to ../src/StarTracker/StarTracker.cpp: findAngles().
	 *
	 * @param pilot 		The pilot star of the cluster.
	 * @param otherStars 	The other stars to find an angle from.
	 * @param compStars		The list to be appended to.
	 */

	public static void combinations ( Star pilot, Star[] otherStars, LinkedList<StarSet> compStars )
	{
		for (int ii = 0; ii < otherStars.length; ii++)
			for (int jj = ii + 1; jj < otherStars.length; jj++)
				for (int kk = jj + 1; kk < otherStars.length; kk++)
				{
					StarSet star = new StarSet(pilot, otherStars[ii], otherStars[jj], otherStars[kk]);

					if ( !(Double.isInfinite(star.attribute) || Double.isNaN(star.attribute)) )
					{
						compStars.add(star);
					}
				}
	}
}
