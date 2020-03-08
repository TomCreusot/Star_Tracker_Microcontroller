/**
 *	Reads a file (template) and replaces specified keys with specified values.
 *
 *	Use:
 *		This is useful to create an array c++ header file with a template.
 *
 *		DO NOT USE A KEY WHICH IS AN OUTPUT VALUE.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		5/2/2020
 */

import java.util.*;

public class FillTemplate
{
	/// keys are replaced by values
	public LinkedList<String> keys;
 	public LinkedList<String> values;


	/**
	 *	Alternate Constructor
	 *	@param fR The file to read from.
	 *	@param fW The file to write to.
	 */

	 public FillTemplate( )
	 {
		 keys = new LinkedList<String>();
		 values = new LinkedList<String>();
	 }



	/**
	 *	Adds a key and replacement value to the checks.
	 *	DO NOT USE A KEY WHICH IS AN OUTPUT VALUE.
	 *	@param key		The key to search.
	 *	@param value	The value to replace.
	 */

	public void addKey ( String key, String value )
	{
		keys.add(key);
		values.add(value);
	}



	/**
	 *	Reads through the file and replaces the keys with the values.
	 *	@param template	The lines of the file to read from.
	 */

	public LinkedList<String> replaceVariables ( LinkedList<String> template )
	{
		LinkedList<String> replaced = new LinkedList<String>();
		ListIterator<String> it = template.listIterator(0);

		while ( it.hasNext() )
		{
			String cur = it.next();
			for ( int i = 0; i < keys.size(); i++ )
			{
				cur = cur.replace(keys.get(i), values.get(i));
			}
			replaced.add(cur);
		}
		return replaced;
	}






	/**
	 * Converts the input list into a format which represents an array.
	 *	{
	 *		{element1},
	 *		{element2},
 	 *	};
	 *
	 *
	 * @param list	The list of elements to add.
	 * @return		The formatted array.
	 */

	public static String listToArrayFormat ( LinkedList<String> list )
	{
		StringBuilder arrayElements = new StringBuilder();
		ListIterator<String> it = list.listIterator();
		arrayElements.append("{\n");

		while ( it.hasNext() )
		{
			arrayElements.append("{" + it.next() + "}");
			if ( it.hasNext() ) arrayElements.append(",\n");
			else 				arrayElements.append("\n");
		}

		arrayElements.append("};");
		return arrayElements.toString();
	}
}
