/**
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
	 * Reads a file as a list of strings.
	 * @param file 	The file to read.
	 * @return 		The lines.
	 */

	public static LinkedList<String> readFile ( String file )
	{
		LinkedList<String> lines = new LinkedList<String>();
		try
		{
			FileInputStream strm = new FileInputStream(file);
			InputStreamReader rdr = new InputStreamReader(strm);
			BufferedReader buf = new BufferedReader(rdr);

			String line = buf.readLine();
			while ( line != null )
			{
				lines.add(line);
				line = buf.readLine();
			}
		} catch ( IOException e) { e.printStackTrace(); }
		return lines;
	}



	/**
	* Reads a properties file (file_name.properties).
	* @param file	The file to read from.
	* @return The properties from the config file.
	*/

	public static Properties readPropertiesFile(String file) {
		Properties config;
		try {
			config = new Properties();
			config.load(new FileReader(file));
		} catch (IOException e) {
			System.err.println("ERROR: Could not read config file: " + file);
			e.printStackTrace();
			config = null;
		}
		return config;
	}





	/**
	 * Writes the data to a file.
	 *
	 * @param data		The data to be stored in the database.
	 * @param fileName	The name of the file.
	 */

	public static void writeFile ( String fileName, LinkedList<String> data )
	{
		try
		{
			FileOutputStream strm = new FileOutputStream(fileName);
			PrintWriter writer = new PrintWriter(strm);

			ListIterator<String> it = data.listIterator();
			while ( it.hasNext() )
			{
				writer.println(it.next());
			}
			writer.close();
			strm.close();
		}
		catch ( IOException e )
		{
			e.printStackTrace();
			throw new IllegalArgumentException("ERROR: could not write to file!");
		}
	}



}
