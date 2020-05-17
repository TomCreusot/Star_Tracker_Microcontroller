#include <string>
#include "gtest/gtest.h"
#include "fill_template.h"

using namespace nix;


TEST ( AddKey, Valid )
{
	nix::FillTemplate temp;
	temp.AddKey("asdf", "fdsa");
	temp.AddKey("list", "qwerty");

	EXPECT_EQ(temp.keys.front(), "asdf");
	EXPECT_EQ(temp.values.front(), "fdsa");

	EXPECT_EQ(temp.keys.back(), "list");
	EXPECT_EQ(temp.values.back(), "qwerty");
}


TEST ( ReplaceVariables, Valid )
{
	nix::FillTemplate temp;
	temp.AddKey("    ", "\t");
	temp.AddKey("a", "b");
	temp.AddKey("q", ".");
	string value = "    tabs    are\tbetter   than\nspacesqqq";

	temp.ReplaceVariables(&value);

	EXPECT_EQ(value, "\ttbbs\tbre\tbetter   thbn\nspbces...");
}



TEST ( Replace, Valid )
{
	string val = "12345678asdf123asdf123";
	nix::FillTemplate::Replace("asdf", "1234567", &val);
	EXPECT_EQ(val, "1234567812345671231234567123");
}

TEST ( Replace, Invalid )
{
	string val = "12345678asdf123asdf123";
	nix::FillTemplate::Replace("abcd", "1234567", &val);
	EXPECT_EQ(val, "12345678asdf123asdf123");
}
