#include "gtest/gtest.h"
#include "properties.h"


using namespace std;


//////////////////////////////////////////////////////////////////////
//																	//
//							---- get ----							//
//																	//
//////////////////////////////////////////////////////////////////////


TEST 		( GetInteger, WhenInvalidKey )
{
	nix::Properties p;
	bool valid = false;
	try
	{
		p.GetInteger("Does not exist");
	}
	catch ( invalid_argument e )
	{
		valid = true;
	}
	EXPECT_TRUE(valid);
}


TEST 		( GetDecimal, WhenInvalidKey )
{
	nix::Properties p;
	bool valid = false;
	try
	{
		p.GetDecimal("Does not exist");
	}
	catch ( invalid_argument e )
	{
		valid = true;
	}
	EXPECT_TRUE(valid);
}


TEST 		( GetInteger, WhenInvalidValue )
{
	nix::Properties p;
	bool valid = false;

	p.Add("Does not exist", "DNE");
	try
	{
		p.GetInteger("Does not exist");
	}
	catch ( invalid_argument e )
	{
		valid = true;
	}
	EXPECT_TRUE(valid);
}


TEST 		( GetDecimal, WhenInvalidValue )
{
	nix::Properties p;
	bool valid = false;

	p.Add("Does not exist", "DNE");
	try
	{
		p.GetDecimal("Does not exist");
	}
	catch ( invalid_argument e )
	{
		valid = true;
	}
	EXPECT_TRUE(valid);
}


TEST 		( GetInteger, WhenValid )
{
	nix::Properties p;

	p.Add("exists", "321");
	EXPECT_EQ(p.GetInteger("exists"), 321);
}


TEST 		( GetDecimal, WhenValid )
{
	nix::Properties p;

	p.Add("exists", "123");
	EXPECT_FLOAT_EQ(p.GetDecimal("exists"), 123);
}


TEST 		( GetString, WhenValid )
{
	nix::Properties p;

	p.Add("exists", "123");
	EXPECT_EQ(p.GetString("exists"), "123");
}


TEST 		( ConvertString, WhenValid )
{
	string str = "abcd\tasdf\n.";
	char array[100];
	nix::Properties::ConvertString(str, array);
	EXPECT_EQ(array[0], 'a');
	EXPECT_EQ(array[1], 'b');
	EXPECT_EQ(array[2], 'c');
	EXPECT_EQ(array[3], 'd');
	EXPECT_EQ(array[4], '\t');
	EXPECT_EQ(array[5], 'a');
	EXPECT_EQ(array[6], 's');
	EXPECT_EQ(array[7], 'd');
	EXPECT_EQ(array[8], 'f');
	EXPECT_EQ(array[9], '\n');
	EXPECT_EQ(array[10], '.');
	EXPECT_EQ(array[11], '\0');
}



TEST 		( GetInteger, WhenValidNegativeDecimal )
{
	nix::Properties p;

	p.Add("exists", "-123.5");
	EXPECT_EQ(p.GetInteger("exists"), -123);
}


TEST 		( GetDecimal, WhenValidNegativeDecimal )
{
	nix::Properties p;

	p.Add("exists", "-123.01234");
	EXPECT_FLOAT_EQ(p.GetDecimal("exists"), -123.01234);
}






//////////////////////////////////////////////////////////////////////
//																	//
//					---- RemoveAfterComment ----					//
//																	//
//////////////////////////////////////////////////////////////////////



TEST		( RemoveAfterComment,  WhenEmpty )
{
	std::string line;
	nix::Properties::RemoveAfterComment(&line);
	EXPECT_EQ(line.length(), 0);
}



TEST		( RemoveAfterComment,  WhenNoComment )
{
	std::string line("HELLO THIS IS A SENTANCE, PLEASE DONT IGNORE");
	std::string expected = line;
	nix::Properties::RemoveAfterComment(&line);
	EXPECT_EQ( line, expected );
}

TEST		( RemoveAfterComment,  WhenOnlyComment )
{
	std::string line("#HELLO THIS IS A SENTANCE, PLEASE DONT IGNORE");
	std::string expected = "";
	nix::Properties::RemoveAfterComment(&line);
	EXPECT_EQ( line.compare(expected), 0 );
}


TEST		( RemoveAfterComment,  WhenCommentHalfLine )
{
	std::string line("HELLO# THIS IS A SENTANCE, PLEASE DONT IGNORE");
	std::string expected = "HELLO";
	nix::Properties::RemoveAfterComment(&line);
	EXPECT_EQ( line, expected );
}


TEST		( RemoveAfterComment,  WhenMultipleComments )
{
	std::string line("HELLO# THIS IS A SENTANCE# PLEASE DONT IGNORE");
	std::string expected = "HELLO";
	nix::Properties::RemoveAfterComment(&line);
	EXPECT_EQ( line, expected );
}



//////////////////////////////////////////////////////////////////////
//																	//
//					---- removeTabsSpaces ----						//
//																	//
//////////////////////////////////////////////////////////////////////

TEST ( RemoveTabsSpaces, WhenEmpty )
{
	std:: string line;

	nix::Properties::RemoveTabsSpaces(&line);

	EXPECT_EQ(line, "");
}


TEST ( RemoveTabsSpaces, WhenNone )
{
	std:: string line = "\nasdf1234";

	nix::Properties::RemoveTabsSpaces(&line);

	EXPECT_EQ(line, "\nasdf1234");
}


TEST ( RemoveTabsSpaces, WhenTab )
{
	std:: string line = "\tasdf1234\t";

	nix::Properties::RemoveTabsSpaces(&line);

	EXPECT_EQ(line, "asdf1234");
}

TEST ( RemoveTabsSpaces, WhenSpace )
{
	std:: string line = " asdf1234 ";

	nix::Properties::RemoveTabsSpaces(&line);

	EXPECT_EQ(line, "asdf1234");
}

TEST ( RemoveTabsSpaces, WhenBoth )
{
	std:: string line = " asdf1234\t";

	nix::Properties::RemoveTabsSpaces(&line);

	EXPECT_EQ(line, "asdf1234");
}









//////////////////////////////////////////////////////////////////////
//																	//
//					---- separateNameValue ----						//
//																	//
//////////////////////////////////////////////////////////////////////

TEST ( SeparateNameValue, WhenEmpty )
{
	std::string line;
	std::string name;
	std::string value;

	bool valid = nix::Properties::SeparateNameValue(line, &name, &value);

	EXPECT_FALSE(valid);
	EXPECT_EQ(line, "");
	EXPECT_EQ(name, "");
	EXPECT_EQ(value, "");
}

TEST ( SeparateNameValue, WhenInvalid )
{
	std::string line = "Thisisnotacorrectline";
	std::string name;
	std::string value;

	bool valid = nix::Properties::SeparateNameValue(line, &name, &value);

	EXPECT_FALSE(valid);
	EXPECT_EQ(name, line);
	EXPECT_EQ(value, "");
}

TEST ( SeparateNameValue, WhenSingleEquals )
{
	std::string line = "hello=10";
	std::string name;
	std::string value;

	bool valid = nix::Properties::SeparateNameValue(line, &name, &value);

	EXPECT_TRUE(valid);
	EXPECT_EQ(name, "hello");
	EXPECT_EQ(value, "10");
}

TEST ( SeparateNameValue, MultipleEquals )
{
	std::string line = "hello=10=";
	std::string name;
	std::string value;

	bool valid = nix::Properties::SeparateNameValue(line, &name, &value);

	EXPECT_TRUE(valid);
	EXPECT_EQ(name, "hello");
	EXPECT_EQ(value, "10");
}
