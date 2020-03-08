/**
 *	Runs the libraries Preprocessor.java and FillTemplate.java to create a compilable database.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		8/3/2020
 */

import java.util.*;
import java.io.*;

public class Main
{
	public static void main ( String [] args )
	{
		Properties config = null;
		if ( args.length == 1 )
		{
			config = FileIO.readPropertiesFile(args[0]);
		}

		if ( config != null )
		{
			// Files
			String database		= config.getProperty("database");
			String template		= config.getProperty("template");
			String output		= config.getProperty("output");

			// Other Strings
			String arrayName	= config.getProperty("array-name");

			// Preprocessor
			double fov			= Double.parseDouble(config.getProperty("fov"));
			double mag	= Double.parseDouble(config.getProperty("cutoff-mag"));
			int pilotSets		= Integer.parseInt(
											config.getProperty("pilot-sets"));

			LinkedList<String> databaseFile = FileIO.readFile(database);
			LinkedList<String> angleDatabase = Preprocessor.run(databaseFile,
														fov, mag, pilotSets);


			FillTemplate fill = new FillTemplate();

			fill.addKey("$(file)", output);
			fill.addKey("$(array_name)", arrayName);
			fill.addKey("$(num_elements)",
										Integer.toString(angleDatabase.size()));

			String array = FillTemplate.listToArrayFormat(angleDatabase);
			fill.addKey("$(array_elements)", array);

			LinkedList<String> templateContence = FileIO.readFile(template);
			FileIO.writeFile(output, fill.replaceVariables(templateContence));
		}
		else
		{
			System.out.println(
				"There was an error opening the config file.\n"+
				"\tSpecify the location of the config file as an argument.\n"+
				"\tRefere to the readme on how to setup the config."
				);
		}

	}








}
