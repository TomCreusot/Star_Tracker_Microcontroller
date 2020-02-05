

import java.util.*;
import java.io.*;

class ToCArray
{

	public static final String [] keys =
	{"$(database_file)", "$(file)", "$(header_guard)", "$(array_name)"};

	// Yep global.
	public static String [] values;

	public static final String arrayKey = "$(loop_array_elements)";
	public static final String numElementsKey = "$(num_elements)";


	public static void main ( String [] args )
	{
		if ( args.length == keys.length )
		{
			values = new String[keys.length];

			for ( int i = 0; i < values.length; i++) values[i] = args[i];

			LinkedList<String> lines = readFile(args[1]);
			replaceVariables(lines);
			LinkedList<String> database = readFile(args[0]);
			replaceDynamicVariables(lines, database);



			ListIterator<String> it = lines.listIterator(0);

			while ( it.hasNext() ) System.out.println(it.next());
		}
		else
		{
			System.out.println("\nError, Please enter the value of:");
			for ( int i = 0; i < keys.length; i++ )
			{
				System.out.println("\t- " + keys[i]);
			}
			System.out.println("\n");
		}

	}


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


	public static void replaceVariables ( LinkedList<String> lines )
	{
		ListIterator<String> it = lines.listIterator(0);

		while ( it.hasNext() )
		{
			String cur = it.next();
			for ( int i = 0; i < keys.length; i++ )
			{
				cur = cur.replace(keys[i], values[i]);
			}
			it.set(cur);
		}
	}


	public static void replaceDynamicVariables ( LinkedList<String> template, LinkedList<String> database )
	{
		ListIterator<String> itT = template.listIterator(0);

		while ( itT.hasNext() )
		{
			String cur = itT.next();
			if ( cur.contains(arrayKey) )
			{
				itT.remove();

				while ( database.size() > 1 )
						itT.add("{" + database.removeFirst() + "},");

				itT.add("{" + database.removeFirst() + "}");
			}
			else if ( cur.contains(numElementsKey) )
			{
				cur = cur.replace(numElementsKey, Integer.toString(database.size()));
				System.out.println(cur);
				itT.set(cur);
			}
		}
	}

}
