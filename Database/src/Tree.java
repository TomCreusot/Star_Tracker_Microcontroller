/*
 *	A very basic tree, this is only to sort the data into a preorder traversal.
 *
 * Note:
 *		This is very basic.
 *		This will not handle exceptions, if there is a problem, the user will still find out though.
 *		This is also designed to handle duplicates and will add the duplicate to the left of the other.
 *
 *						__ 20 __
 *					  /			\
 *				 __ 10 __	 	15
 *				/		 \
 *		__ 10 (dupe)	 11
 *	  /
 *	9
 *
 *
 *	@author		Tom Creusot
 *	@version 	1.0
 *	@since		12/1/2020
 */


import java.util.*;


/**
* A binary search tree which stores identical keys to the left.
*/

public class Tree
{
	// The top most node
	public TreeNode root;
	// It is nice to track.
	public int count;


	/**
	 * Default Constructor.
	 */

	public Tree ( ) { root = null; }



	/**
	 * Alturnate Constructor.
	 * @param stars	The list to insert into the tree.
	 */

	public Tree ( LinkedList<Star> stars )
	{
		ListIterator<Star> node = stars.listIterator(0);
		while ( node.hasNext() )	insert(node.next());
	}



	/**
	 * Inserts the element into the corresponding location.
 	 * 		NOTE:	In the case of a duplicate, it will add it immediatly to the left of the current element.
	 *
	 * @param val	The value to insert.
	 */

	public void insert ( Star val )
	{
		TreeNode node = new TreeNode(val);
		if ( root != null )
		{
			boolean found = false;
			TreeNode curNode = root;

			while ( !found )
			{
				if ( node.value.attribute < curNode.value.attribute )
				{
					if ( curNode.left != null )
					{
						curNode = curNode.left;
					}
					else
					{
						curNode.left = node;
						found = true;
					}
				}
				else if ( node.value.attribute > curNode.value.attribute )
				{
					if ( curNode.right != null )
					{
						curNode = curNode.right;
					}
					else
					{
						curNode.right = node;
						found = true;
					}
				}
				else
				{
					node.left = curNode.left;
					curNode.left = node;
					found = true;
				}
			}
		}
		else root = node;
		count++;
	}






	/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
	|																  	|
	-	--------------------- Balance Stuff --------------------		-
	|																	|
	\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


	/**
	* Input a tree, output a balanced tree.
	*
	* @param 	tree The tree to create a balanced duplicate of.
	* @return	The balanced duplicate.
	*/

	public static Tree createBalancedTree ( Tree tree )
	{
		LinkedList<Star> sorted = tree.inOrderTraversal();
		Star[] array = sorted.toArray(new Star[sorted.size()]);
		Tree newTree = new Tree();
		createBalancedTreeRec(newTree, array, 0, array.length - 1);
		return newTree;
	}



	/**
	 * Recursivly balances the input tree into this tree.
	 *
	 * @param traversal	The array of values.
	 * @param max		The maximum bounds to look at.
	 * @param min		The minimum bounds to look at.
	 */

	private static void createBalancedTreeRec ( Tree tree, Star[] traversal, int min, int max )
	{
		int mid = (max - min) / 2 + min;
        if ( mid != max && mid != min )
        {
            tree.insert(traversal[mid]);
            createBalancedTreeRec(tree, traversal, min, mid);
            createBalancedTreeRec(tree, traversal, mid, max);
      	}

	}




	/**
	* Finds a percentage of how balanced the tree is.
	*
	* @return a number from 0 to 100 representing the balance.
	*/

	public int balance ( )
	{
		return balanceRec(root);
	}


	/**
	 * Recursively finds a percentage of how balanced the tree is.
	 *
	 * @return a number from 0 to 100 representing the balance.
	 */

	 public int balanceRec ( TreeNode node )
	 {
		 if ( node != null )
		{
			if ( node.left == null && node.right == null )
			{
				return 100;
			}
			else
			{
				return (balanceRec(node.left) + balanceRec(node.right)) / 2;
			}
		}
		return 0;
	}

	/**
	 * Finds the maximum height of the node.
	 */

	public int height ( TreeNode node )
    {
        /* base case tree is empty */
        if (node == null) return 0;

        /* If tree is not empty then height = 1 + max of left
         height and right heights */
        return 1 + Math.max(height(node.left), height(node.right));
    }



	/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
	|																  	|
	-	--------------------- Traversal Stuff -------------------		-
	|																	|
	\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



	/**
	* The wrapper that peforms a traversal in (Root, Left, Right) order.
	* This is useful for reconstructing the tree.
	*
	* @return 	The traversal.
	*/

	public LinkedList<Star> preOrderTraversal ( )
	{
		LinkedList<Star> list = new LinkedList<Star>();
		preOrderTraversalRec(list, root);
		return list;
	}



	/**
	* The wrapper that peforms a traversal in (Left, Root, Right) order.
	* This is useful for ordering.
	*
	* @return 	The traversal.
	*/

	public LinkedList<Star> inOrderTraversal ( )
	{
		LinkedList<Star> list = new LinkedList<Star>();
		inOrderTraversalRec(list, root);
		return list;
	}



	/**
	 * Recursivly performs a traversal (Root, Left, Right).
	 *
	 * @param traversal	The traversal.
	 * @param curNode	The node to examine.
	 */

	private void preOrderTraversalRec ( LinkedList<Star> traversal, TreeNode curNode )
	{
		if ( curNode != null )
		{
			traversal.add(curNode.value);
			preOrderTraversalRec(traversal, curNode.left);
			preOrderTraversalRec(traversal, curNode.right);
		}
	}


	/**
	 * Recursivly performs a traversal (Left, Root, Right).
	 *
	 * @param traversal	The traversal.
	 * @param curNode	The node to examine.
	 */

	private void inOrderTraversalRec ( LinkedList<Star> traversal, TreeNode curNode )
	{
		if ( curNode != null )
		{
			inOrderTraversalRec(traversal, curNode.left);
			traversal.add(curNode.value);
			inOrderTraversalRec(traversal, curNode.right);
		}
	}





	/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
	|																  	|
	-	---------------------- Display Stuff --------------------		-
	|																	|
	\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


	/**
	 * Displays the tree.
	 */

	public void displayTree ( )
	{
		displayTreeRec(root, 10);
	}

	/**
	 * Displays a list of the keys in order of left to right.
	 * @param cNode The current node to display childeren of.
	 * @param indend The indent from the side.
	 */

	private void displayTreeRec ( TreeNode cNode, int indent )
	{
		if ( cNode != null )
		{
			for ( int i = 0; i < indent; i++ )
			{
				System.out.print("\t");
			}
			System.out.println(cNode.value.attribute);
			displayTreeRec(cNode.left, indent - 1);
			displayTreeRec(cNode.right, indent + 1);
		}
	}




}







/**
 * The tree node to the Tree class.
 */

class TreeNode
{
	public Star value;
	public TreeNode left;
	public TreeNode right;

	/**
	 * Alturnate Constructor.
	 * @param value_	The value of the tree node.
	 */

	public TreeNode ( Star value_ )
	{
		value = value_;
	}
}
