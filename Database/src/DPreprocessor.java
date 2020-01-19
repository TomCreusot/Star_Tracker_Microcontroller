/*
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

public class DPreprocessor
{
	/**
	 * Yep, this function is probably needed.
	 */

	public static void main ( String[] args )
	{
		if (args.length > 3)
		{
			long time = System.currentTimeMillis();
			double cutOffMag = Double.parseDouble(args[0]);
			int pilotSets = Integer.parseInt(args[1]);
			double radiusPilot = Double.parseDouble(args[2]);

			System.out.println(
				"Reading file for: "+
				"\n\tCut of magnitude: " + cutOffMag +
				"\n\tStars surounding pilot: " + pilotSets +
				"\n\tRadius (degrees) from pilot: " + radiusPilot +
				"\n\tFile: " + args[3] );

			LinkedList<Star> stars = sortedList(FileIO.readFile(args[3], cutOffMag));

			System.out.println("\n" + stars.size() + " valid stars.");

			LinkedList<Star> output = getAngles(stars, pilotSets /*+1*/, radiusPilot);

			System.out.println("\nNumber of angles: " + output.size() + ", time taken: " + (System.currentTimeMillis() - time) + " ms\n");
			FileIO.writeToFile("angle,ra,dec", output, "output-" + cutOffMag + "-" + pilotSets + "-" + radiusPilot + ".csv");
		}
		else
		{
			System.out.println(
			"Please input the command line arguments (in order):" +
			"\n\t - The apperant magnitude cut off." +
			"\n\t - The number of stars to associate with one pilot star." +
			"\n\t - The distance a star can be from a pilot in degrees." +
			"\n\t - The file name.");
		}

	}


	/**
	 * Returns the angles
	 *
	 * @param stars 		The magnitude, ra, dec of all the stars to check.
	 * @param pilotSets		The number of stars surrounding the pilot to use.
	 * @param radiusPilot	The angle distance between the pilot and the chosen stars.
	 *
	 */

	public static LinkedList<Star> getAngles ( LinkedList<Star> stars, int pilotSets, double radiusPilot )
	{
		LoadingBar load = new LoadingBar(stars.size(), 200, '#');
		LinkedList<Star> output = new LinkedList<Star>();

		LinkedList<String> debug = new LinkedList<String>();

		while ( stars.size() > 0 )
		{
			Star pilot = stars.getFirst();

			Star[] s = findClosestBrightest(pilot, stars, pilotSets, radiusPilot);
			if ( s != null )
			{
				combinations(pilot, s, output);

				String d = pilot.attribute + "," + pilot.ra + "," + pilot.dec + ", , , , ";
				for ( int i = 0; i < s.length; i++ )
				{
					d += s[i].attribute + "," + s[i].ra + "," + s[i].dec + ", , , ,";
				}
				debug.add(d);
			}
			else debug.add("");

			stars.removeFirst();
			load.load(1);
		}
		FileIO.writeToFile(debug, "debug.csv");
		return output;
	}


	/**
	 * Returns a sorted linked list of stars attributes.
	 *
	 * @param stars The unsorted array.
	 * @return The sorted version of stars.
	 */

	public static LinkedList<Star> sortedList ( LinkedList<Star> stars )
	{
		Tree tree = new Tree(stars);
		return tree.inOrderTraversal();
	}




	/**
	 * Finds the brightest stars within the specified bounds from the pilot star.
	 *
	 * @param pilot	The pilot star.
	 * @param stars The stars to observe.
	 * @param num	The number of stars to add.
	 * @param radus The maximum distance the stars can be from the pilot.
	 * @return		An array of the closest stars OR null if not all values found.
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

	public static void combinations ( Star pilot, Star[] otherStars, LinkedList<Star> compStars )
	{
		for (int ii = 0; ii < otherStars.length; ii++)
			for (int jj = ii + 1; jj < otherStars.length; jj++)
				for (int kk = jj + 1; kk < otherStars.length; kk++)
					compStars.add(new Star(pilot, otherStars[ii], otherStars[jj], otherStars[kk]));
	}
}










/**
 * This class is to give a graphical representation of how long is left.
 */

class LoadingBar
{
	// The current position in loading and the maximum (end).
	private double current, max;

	// How many dots.
	private int numIndicators;
	// How many dots are already on the screen.
	private int alreadyPrinted;

	// The symbol to use.
	private char symbol;

	/**
	 * Alturnate Constructor.
	 *
	 * @param 	max_ 	The maximum value.
	 * @param	numIndicators_
	 */

	public LoadingBar ( double max_, int numIndicators_, char symbol_ )
	{
		current = 0;
		max = max_;
		numIndicators = numIndicators_;
		symbol = symbol_;

		for ( int i = 0; i < numIndicators; i++ )
		{
			System.out.print(symbol);
		}
		System.out.println();
	}


	/**
	 * If the progression is enough, it will print a character.
	 *
	 * @param c	The iterator.
	 */

	public void load ( int c )
	{
		current += c;
		if ( current > alreadyPrinted * max / numIndicators )
		{
			System.out.print(symbol);
			alreadyPrinted++;
		}
	}

}
