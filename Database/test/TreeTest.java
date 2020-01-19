import java.util.*;

/*
 *	This file is designed to test /Database/src/Tree.java
 *
 *	It tests the methods:
 *		- inOrderTraversal
 *		- insert
 *		- preOrderTraversal
 *		- Alternate Constructor
 *		- balance
 *		- height
 *		- createBalancedTree
 *
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */


public class TreeTest extends UnitTest
{

	/**
	 * Call to test Tree.java
	 * @return True if all tests pass.
	 */

	public static boolean run ( )
	{
		printHeader("Tree.java", "TreeTest.java");
		boolean test = true;
		test &= printPass("inOrderTraversal", testInOrderTraversal(generateRandom(100)));
		test &= printPass("preOrderTraversal", testPreOrderTraversal());
		test &= printPass("insert", testInsert());
		test &= printPass("alternate constructor", testAlternateConstructor());
		test &= printPass("height", testHeight());
		test &= printPass("balance", testBalance());
		test &= printPass("createBalancedTree", testCreateBalancedTree(generateRandom(256)));
		return test;
	}




	/**
	 * Checks the specified tree.inOrderTraversal is in order min to max.
	 *
	 * @param tree The tree to test.
	 * @return True if all cases passed.
	 */

	public static boolean testInOrderTraversal ( Tree tree )
	{
		LinkedList<Star> stars = tree.inOrderTraversal();
		ListIterator<Star> it = stars.listIterator();
		double prev = stars.getFirst().attribute;
		double cur;
		boolean valid = true;

		while ( it.hasNext() && valid )
		{
			cur = it.next().attribute;
			valid &= ( cur >= prev );
			prev = cur;
		}
		return valid;
	}




	/**
	 * Tests preOrderTraversal
	 *
	 * @return True if all cases passed.
	 */

	public static boolean testPreOrderTraversal ( )
	{

		return false;
	}



	/**
	 * Runs test testing the Tree.insert(Star) method:
	 *		- Inserting the root.
	 *		- Inserting left and right.
	 *		- Appending to a branch.
	 *		- Duplications.
	 *		- Adding a leaf after a dupe on the left.
	 *		- Adding a leaf to the right of a dupe.
	 *		- Adding a dupe after the origional had 2 leaves.
	 *
	 * REQUIRES inOrderTraversal TO BE CORRECT.
	 * @return true if all cases passed.
	 */

	public static boolean testInsert ( )
	{
		boolean valid = true;
		Tree tree = new Tree();
		// -99, -43, -43, -2, -1, 2, 5, 5, 12, 22
		tree.insert(new Star(-1));		// Tests insert left
		tree.insert(new Star(5));		// Tests insert right
		tree.insert(new Star(22));		// Tests appending to branch
		tree.insert(new Star(12));
		tree.insert(new Star(-43));
		tree.insert(new Star(-2));	 	// Tests adding right of dupe
		tree.insert(new Star(-43)); 	// Tests inserting on self
		tree.insert(new Star(2));
		tree.insert(new Star(5));		// Tests inserting on self with full branch
		tree.insert(new Star(-99)); 	// Tests adding after duplicate

		tree.insert(new Star(-43)); 	// Tests inserting on self when leaf.

		LinkedList<Star> ordered = tree.inOrderTraversal();

		valid &= equal(ordered.removeFirst(), -99d);
		valid &= equal(ordered.removeFirst(), -43d);
		valid &= equal(ordered.removeFirst(), -43d);
		valid &= equal(ordered.removeFirst(), -43d);
		valid &= equal(ordered.removeFirst(), -2d);
		valid &= equal(ordered.removeFirst(), -1d);
		valid &= equal(ordered.removeFirst(), 2d);
		valid &= equal(ordered.removeFirst(), 5d);
		valid &= equal(ordered.removeFirst(), 5d);
		valid &= equal(ordered.removeFirst(), 12d);
		valid &= equal(ordered.removeFirst(), 22d);
		return valid;
	}





   /**
	* The alternate constructor creates the object with a linked list.
	* If every element is in order and inserted, it works.
	*
	* REQUIRES inOrderTraversal TO WORK.
	* @return True if all tests pass.
	*/

	public static boolean testAlternateConstructor ( )
	{
		boolean valid = true;
		LinkedList<Star> stars = new LinkedList<Star>();
		// -5, -3, -3, -2, 10, 53
		stars.add(new Star(10));
		stars.add(new Star(-3));
		stars.add(new Star(-2));
		stars.add(new Star(-5));
		stars.add(new Star(-3));
		stars.add(new Star(53));

		Tree tree = new Tree(stars);
		stars = tree.inOrderTraversal();

		valid &= equal(stars.removeFirst(), -5);
		valid &= equal(stars.removeFirst(), -3);
		valid &= equal(stars.removeFirst(), -3);
		valid &= equal(stars.removeFirst(), -2);
		valid &= equal(stars.removeFirst(), 10);
		valid &= equal(stars.removeFirst(), 53);
		return valid;
	}


	/**
	 * Tests balanceRec so that testing of createBalancedTree can be tested.
	 * Adds several elements to make a degenerate and balanced tree.
	 *
	 * @return True if all tests pass.
	 */

	 public static boolean testBalance ( )
	 {
		 /*boolean valid = true;
		 Tree tree = new Tree();
		 // Degenerate (0%)
		 tree.insert(new Star(0));
		 tree.insert(new Star(2));
		 tree.insert(new Star(1));

		 valid &= tree.balance() < 50;

		 // Slightly Degenerate (50%)
		 tree.insert(new Star(-2));
		 tree.insert(new Star(-1));
		 valid &= tree.balance() == 50;

		 //  Mostly Balanced (75%)
		 tree.insert(new Star(3));
		 valid &= tree.balance() == 75;

		 // Balanced (100 %)
		 tree.insert(new Star(-3));
		 valid &= tree.balance() == 100;*/

		 return false;//valid;
	 }


	 /**
	  * Tests the height function.
	  *
	  * @return True if all tests pass.
	  */

	 public static boolean testHeight ( )
	 {
		 boolean valid = true;
		 Tree tree = new Tree();
		 // degenerate
		 tree.insert(new Star(0));
		 tree.insert(new Star(1));
		 tree.insert(new Star(2));

		 valid &= tree.height(tree.root) == 3;

		 // Mostly degenerate
		 tree.insert(new Star(-1));
		 valid &= tree.height(tree.root) == 3;

		 // Even degenerate
		 tree.insert(new Star(-2));
		 valid &= tree.height(tree.root) == 3;

		 // Mostly degenerate
		 tree.insert(new Star(-3));
		 valid &= tree.height(tree.root) == 4;

		 return valid;
	 }






	 /**
	  * Tests if the tree can be staticaly balanced.
	  *
	  * @param tree The tree to test.
	  * @return True if all tests passed.
	  */

	 public static boolean testCreateBalancedTree ( Tree tree ) // This works, balance() does not (pass 33, 65, 256...).
	 {
		 boolean valid = true;
		 System.out.println("Created Balance: " + Tree.createBalancedTree(tree).balance() + "  " + tree.balance());
		 return false;// valid;
	 }




	/**
	 * Generates a randomised tree given the specified size.
	 *
	 * REQURIES insert() TO BE CORRECT.
	 *
	 * @param size The number of elements to add.
	 * @return The randomly generated tree.
	 */

	public static Tree generateRandom ( int size )
	{
		Tree tree = new Tree();
		for ( int i = 0; i < size; i++ )
		{
			tree.insert(new Star(Math.random() * 100));
		}
		return tree;
	}



}
