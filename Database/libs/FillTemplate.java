/*
 *	Reads a file (template) and replaces specified keys with specified values.
 *
 *	Use:
 *		This is useful to create an array c++ header file with a template.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		5/2/2020
 */

import java.util.*;

public class FillTemplate
{
	// fileR: read template, fileW: write template
	String fileR, fileW;

	/// keys are replaced by values
	private ArrayList<String> keys;
 	private ArrayList<String> values;



	/**
	 * Sample code.
	 * @param args The command line arguments:
	 *		- args[0]:	Template file name.
	 *		- args[1]:	csv file name.
	 *		- args[2]:	output file name.
	 *		- args[3]:	array name.
	 */

	public static void main ( String [] args )
	{
		if ( args.length == 4 )
		{
			String templateFile	= args[0];
			String csvFile 		= args[1];
			String outputFile 	= args[2];
			String arrayName 	= args[3];

			FillTemplate fill = new FillTemplate(templateFile, outputFile);

			LinkedList<String> csv = FileIO.readFile(csvFile);

			fill.addKey("$(file)", outputFile);
			fill.addKey("$(array_name)", arrayName);
			fill.addKey("$(num_elements)", Integer.toString(csv.size()));

			StringBuilder arrayElements = new StringBuilder();

			while ( csv.size() > 1 )
			{
				arrayElements.append("{" + csv.removeFirst() + "},\n");
			}
			arrayElements.append("{" + csv.removeFirst() + "}");
			fill.addKey("$(array_elements)", arrayElements.toString());
			fill.run();
		}
		else
		{
			System.out.println("ERROR, Please Enter: ");
			System.out.println("\t- The template file name.");
			System.out.println("\t- The csv file name.");
			System.out.println("\t- The output file name.");
			System.out.println("\t- The name of the array.");
		}


	}






	/**
	 *	Alternate Constructor
	 *	@param fR The file to read from.
	 *	@param fW The file to write to.
	 */

	 public FillTemplate( String fR, String fW )
	 {
		 fileR = fR;
		 fileW = fW;
		 keys = new ArrayList<String>();
		 values = new ArrayList<String>();
	 }



	/**
	 *	Adds a key and replacement value to the checks.
	 *	@param key		The key to search.
	 *	@param value	The value to replace.
	 */

	public void addKey ( String key, String value )
	{
		keys.add(key);
		values.add(value);
	}



	/**
	 *	Reads the template and writes to the file with the keys replaced.
	 */

	public void run ( )
	{
		LinkedList<String> lines = FileIO.readFile(fileR);
		replaceVariables(lines);
		FileIO.writeFile(fileW, lines);
	}



	/**
	 *	Reads through the file and replaces the keys with the values.
	 *	@param lines	The lines of the file to read from.
	 */

	public void replaceVariables ( LinkedList<String> lines )
	{
		ListIterator<String> it = lines.listIterator(0);

		while ( it.hasNext() )
		{
			String cur = it.next();

			for ( int i = 0; i < keys.size(); i++ )
			{
				cur = cur.replace(keys.get(i), values.get(i));
			}
			it.set(cur);
		}
	}
}
