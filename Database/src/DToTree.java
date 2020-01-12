/*
 *	This file is to convert some preprocessed data into a tree format.
 *	It requires a csv file with 3 columns and a header row.
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */

import java.util.*;

public class DToTree
{
	/**
	 * This function runs all the code of the program.
	 */

	public static void main ( String [] args )
	{
		if (args.length > 0)
		{
			// Unbalanced Tree
			Tree tree = new Tree(FileIO.readFile(args[0], 1000)); //angles only get to 2pi
			// Balanced Tree
			Tree balanced = Tree.createBalancedTree(tree);
			System.out.println("Balance of tree: " + balanced.balanceRec(balanced.root) + "%" );

			String[] file = args[0].split("/");
			FileIO.writeToFile("angle,ra,dec", balanced.preOrderTraversal(), "balanced" + file[file.length - 1]);
		}
		else
		{
			System.out.println("Input the file name as a parameter.");
		}
	}
}
