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
		if (args.length > 2)
		{
			long time = System.currentTimeMillis();
			double cutOffMag = Double.parseDouble(args[0]);

			int pilotSubs = Integer.parseInt(args[1]);

			System.out.println(
				"Reading file for: "+
				"\n\tCut of magnitude: " + cutOffMag +
				"\n\tStars surounding pilot: " + pilotSubs +
				"\n\tFile: " + args[2] );

			LinkedList<Star> stars = FileIO.readFile(args[2], cutOffMag);

			System.out.println("\n" + stars.size() + " valid stars.");
			LoadingBar load = new LoadingBar(stars.size(), 200, '#');

			double roof = cutOffMag + 1f;
			LinkedList<Star> output = new LinkedList<Star>();

			while ( stars.size() > pilotSubs )
			{
				Star p = findBrightest(stars, roof);
				roof = p.attribute;

				Star[] s = findClosest(p, stars, pilotSubs);
				combinations(p, s, output);

				stars.remove(p);
				load.load(1);
			}
			System.out.println("\nNumber of angles: " + output.size() + ", time taken: " + (System.currentTimeMillis() - time) + " ms\n");
			FileIO.writeToFile("angle,ra,dec", output, "output-" + cutOffMag + "-" + pilotSubs + ".csv");
		}
		else
		{
			System.out.println(
			"Please input the command line arguments (in order):" +
			"\n\t - The apperant magnitude cut off." +
			"\n\t - The number of stars to associate with one pilot star." +
			"\n\t - The file name.");
		}

	}













	/**
	 * Finds the brightest star which is less than "roof".
	 *
	 * @param stars	The list of stars.
	 * @param roof	The maximum value (Exclusively that the star can have).
	 * @return		The star which is the brightest within the parameters.
	 */

	public static Star findBrightest ( LinkedList<Star> stars, double roof )
	{
		ListIterator<Star> it = stars.listIterator(0);
		Star brightest = it.next();

		while ( it.hasNext() )
		{
			Star cur = it.next();
			if ( cur.attribute > brightest.attribute && cur.attribute < roof )
			{
				brightest = cur;
			}
		}
		return brightest;
	}






	/**
	 * Finds the closest stars to the specified declination and ascention.
	 * These are in order of smallest to largest distance.
	 *
	 * @param pilot	The pilot star.
	 * @param stars The stars to observe.
	 * @param num	The number of stars to add.
	 * @return		An array of the closest stars.
	 */

	public static Star[] findClosest ( Star pilot, LinkedList<Star> stars, int num )
	{
		Star[] close = new Star[num];
		double lastDist = 0;
		double curDist = 0;

		for ( int i = 0; i < num; i++ )
		{
			ListIterator<Star> it = stars.listIterator(0);
			curDist = Double.MAX_VALUE;

			while ( it.hasNext() )
			{
				Star cur = it.next();
				double dist = cur.distanceFromPilot(pilot);

				if ( dist > lastDist && dist < curDist  )
				{
					curDist = dist;
					close[i] = cur;
				}
			}

			lastDist = curDist;
		}
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
		for (int ii = 1; ii < otherStars.length; ii++)
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
