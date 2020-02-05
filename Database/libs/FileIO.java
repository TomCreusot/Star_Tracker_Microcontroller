/*
 *	Peforms read/write functionality to specified files.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */

import java.io.*;
import java.util.*;


public class FileIO
{
	/**
	 * Reads in a csv file.
	 *
	 * This requires a csv file format in the order of:
	 * magnitude, right ascension and declination.
	 * The first line will not be read.
	 * Any magnitude/attribute above "cutOff" will not be included.
	 *
	 * @param fileName 	The name of the file.
	 * @param cutOff	If the magnitude exceeds this, it will not be included.
	 * @return			The stars which are valid.
	 */

	public static LinkedList<Star> readFile ( String fileName, double cutOff )
	{
		LinkedList<Star> list = new LinkedList<Star>();
		try
		{
			FileInputStream strm = new FileInputStream(fileName);
			InputStreamReader rdr = new InputStreamReader(strm);
			BufferedReader buf = new BufferedReader(rdr);
			buf.readLine();	//Header

			String[] stars;
			String star = buf.readLine();
			while ( star != null )
			{
				stars = star.split(",");
				double mag = Double.parseDouble(stars[0]);
				if ( mag < cutOff )
				{
					Star s = new Star(mag, Double.parseDouble(stars[1]), Double.parseDouble(stars[2]));
					list.add(s);
				}
				star = buf.readLine();
			}
			strm.close();
		}
		catch ( IOException e )
		{
			throw new IllegalArgumentException("ERROR: Could not read file!");
		}
		return list;
	}





	/**
	* Writes the data to a csv database (with a header row).
	*
	* @param header 	The text to display at the top.
	* @param data		The data to be stored in the database.
	* @param fileName	The name of the file.
	*/

	public static void writeToFile ( String header, LinkedList<Star> data, String fileName )
	{
		try
		{
			FileOutputStream strm = new FileOutputStream(fileName);
			PrintWriter writer = new PrintWriter(strm);

			ListIterator<Star> it = data.listIterator(0);
			writer.println(header);
			while ( it.hasNext() )
			{
				Star star = it.next();
				writer.println(star.toCSVString());
			}
			writer.close();
			strm.close();
		}
		catch ( IOException e )
		{
			System.out.println(e.toString());
			throw new IllegalArgumentException("ERROR: could not write to file!");
		}
	}



	/**
	* Writes the data to a file.
	*
	* @param data		The data to be stored in the database.
	* @param fileName	The name of the file.
	*/

	public static void writeToFile ( LinkedList<String> data, String fileName )
	{
		try
		{
			FileOutputStream strm = new FileOutputStream(fileName);
			PrintWriter writer = new PrintWriter(strm);

			ListIterator<String> it = data.listIterator(0);
			while ( it.hasNext() )
			{
				writer.println(it.next());
			}
			strm.close();
		}
		catch ( IOException e )
		{
			System.out.println(e.toString());
			throw new IllegalArgumentException("ERROR: could not write to file!");
		}
	}
}
