import java.util.*;

import junit.framework.TestCase;
import org.junit.*;


public class TestFillTemplate extends TestCase
{
	@Test
	public static void testConstructor ( )
	{
		FillTemplate fill = new FillTemplate();
		assertTrue(fill.keys != null);
		assertTrue(fill.values != null);
	}

	@Test
	public static void testAddKey ( )
	{
		FillTemplate fill = new FillTemplate();
		fill.addKey("", "");
		fill.addKey("a", "b");
		fill.addKey("1", "2");
		fill.addKey("\n\n", "\n\n");
		fill.addKey("\tabc", "\\");
		assertEquals("",		fill.keys.removeFirst());
		assertEquals("a",		fill.keys.removeFirst());
		assertEquals("1",		fill.keys.removeFirst());
		assertEquals("\n\n",	fill.keys.removeFirst());
		assertEquals("\tabc",	fill.keys.removeFirst());

		assertEquals("", 		fill.values.removeFirst());
		assertEquals("b",		fill.values.removeFirst());
		assertEquals("2",		fill.values.removeFirst());
		assertEquals("\n\n",	fill.values.removeFirst());
		assertEquals("\\",		fill.values.removeFirst());
	}



	@Test
	public static void testReplaceVariables ( )
	{
		FillTemplate fill = new FillTemplate();

		fill.addKey("key", "value");
		fill.addKey("a ", "b ");
		fill.addKey("1 ", "2 ");
		fill.addKey("c c", "d d");
		fill.addKey("\n\n", "\t\t");

		LinkedList<String> template = new LinkedList<String>();
		template.add(	"this should not be altered"	);
		template.add(	"key should be value"			);
		template.add(	"a should be b"					);
		template.add(	"1 should be 2"					);
		template.add(	"c c should be d d"				);
		template.add(	"\n\n should be \t\t(tab x2)"	);

		LinkedList<String> replaced = fill.replaceVariables(template);

		assertEquals("this should not be altered", 		replaced.removeFirst());
		assertEquals("value should be value", 			replaced.removeFirst());
		assertEquals("b should be b", 					replaced.removeFirst());
		assertEquals("2 should be 2", 					replaced.removeFirst());
		assertEquals("d d should be d d", 				replaced.removeFirst());
		assertEquals("\t\t should be \t\t(tab x2)", 	replaced.removeFirst());
	}


	@Test
	public static void testListToArrayFormat ( )
	{
		LinkedList<String> list = new LinkedList<String>();
		String str = FillTemplate.listToArrayFormat(list);
		String [] split = str.split("\n");


		assertEquals(split[0], "{");
		assertEquals(split[1], "};");

		list.add(	"1, 2, 3"	);
		str = FillTemplate.listToArrayFormat(list);
		split = str.split("\n");
		assertEquals(split[0], "{");
		assertEquals(split[1], "{1, 2, 3}");
		assertEquals(split[2], "};");


		list.add(	"-1, 2, 3"	);
		list.add(	"-, -, -"	);
		list.add(	"a, b, c"	);
		list.add(	"a, b"	);
		list.add(	""	);

		str = FillTemplate.listToArrayFormat(list);
		split = str.split("\n");
		assertEquals("0", "{", 				split[0]);
		assertEquals("e1", "{1, 2, 3},",	split[1]);
		assertEquals("e2", "{-1, 2, 3},",	split[2]);
		assertEquals("e3", "{-, -, -},",	split[3]);
		assertEquals("e4", "{a, b, c},",	split[4]);
		assertEquals("e5", "{a, b},",		split[5]);
		assertEquals("e6", "{}",			split[6]);
		assertEquals("7", "};",				split[7]);
	}
}
