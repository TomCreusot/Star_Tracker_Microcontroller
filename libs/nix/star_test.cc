#include "gtest/gtest.h"
#include "star.h"
#include "libs/util/array_list.h"
#include <iostream>

using namespace std;
using namespace util;
using namespace star_tracker;

TEST ( DefaultConstructor, Test )
{
	star_tracker::Star s;
	EXPECT_FLOAT_EQ(s.position.x, 0);
	EXPECT_FLOAT_EQ(s.position.y, 0);
	EXPECT_FLOAT_EQ(s.magnitude, 0);
}

TEST ( AlternateConstructor, Test )
{
	star_tracker::Star s(10, -10, 12);
	EXPECT_FLOAT_EQ(s.position.x, 10);
	EXPECT_FLOAT_EQ(s.position.y, -10);
	EXPECT_FLOAT_EQ(s.magnitude, 12);
}


TEST ( AlternateConstructor, Point )
{
	Point<decimal> pos(1, -2);
	star_tracker::Star s(pos, 12);
	EXPECT_FLOAT_EQ(s.position.x, 1);
	EXPECT_FLOAT_EQ(s.position.y, -2);
	EXPECT_FLOAT_EQ(s.magnitude, 12);
}


TEST ( Split, Valid )
{
	char token = ',';
	string str = "a,b,c,,,d,asdf,1234,bvcx,";
	std::vector<string> v;
	star_tracker::Star::Split(token, str, &v);

	EXPECT_EQ(v[0], "a");
	EXPECT_EQ(v[1], "b");
	EXPECT_EQ(v[2], "c");
	EXPECT_EQ(v[3], "");
	EXPECT_EQ(v[4], "");
	EXPECT_EQ(v[5], "d");
	EXPECT_EQ(v[6], "asdf");
	EXPECT_EQ(v[7], "1234");
	EXPECT_EQ(v[8], "bvcx");
	EXPECT_EQ(v[9], "");
	EXPECT_EQ(v.size(), 10);

	str = "z, x, y";
	v = std::vector<string>();
	star_tracker::Star::Split(token, str, &v);
	EXPECT_EQ(v[0], "z");
	EXPECT_EQ(v[1], " x");
	EXPECT_EQ(v[2], " y");
	EXPECT_EQ(v.size(), 3);

}





TEST ( SortByMagnitude, Basic )
{
	Star s1(1, 1, 2);
	Star s2(2, 2, 1);

	EXPECT_TRUE(star_tracker::Star::SortByMagnitude(s2, s1));
	EXPECT_FALSE(star_tracker::Star::SortByMagnitude(s1, s2));
}


TEST ( SortByMagnitude, ArrayList )
{
	util::ArrayList<star_tracker::Star, 10> star_list;
	star_tracker::Star s1(1, 1, 3);
	star_tracker::Star s2(2, 2, 2);
	star_tracker::Star s3(3, 3, 1);
	star_tracker::Star s4(4, 4, 0);

	star_list.PushBack(s1);
	star_list.PushBack(s3);
	star_list.PushBack(s4);
	star_list.PushBack(s2);

	star_list.Sort(star_tracker::Star::SortByMagnitude);
	EXPECT_FLOAT_EQ(s1.magnitude, 3);
	EXPECT_FLOAT_EQ(s2.magnitude, 2);
	EXPECT_FLOAT_EQ(s3.magnitude, 1);
	EXPECT_FLOAT_EQ(s4.magnitude, 0);
}






TEST ( FindCloseStars, Valid )
{
	const util::uint size = 10;
	util::ArrayList<Star, size> stars;
	util::ArrayList<Point<decimal>, size> points;

	Star s1(0, 0, 1);
	Star s2(0.1, 0, 2);
	Star s3(2.01, 0, 3);	// Futhest distance
	Star s4(-2.01, -1, 4);
	Star s5(1.999, 0, 5);
	Star s6(-1.999, 0, 5);
	Star s7(4.001, 0, 5);
	Star s8(-4.001, 0, 5);

	stars.PushBack(s1);
	stars.PushBack(s2);
	stars.PushBack(s3);
	stars.PushBack(s4);
	stars.PushBack(s5);
	stars.PushBack(s6);
	stars.PushBack(s7);
	stars.PushBack(s8);

	star_tracker::Star::FindCloseStars<size, size>(0, 4, 2, stars, &points);

	EXPECT_EQ(points.Get(0).x, 0);
	EXPECT_EQ(points.Get(1).x, 0.1);
	EXPECT_EQ(points.Get(3).x, 1.999);
	// EXPECT_EQ(points.Get(4).x, -1.999); // This is the 5th element
	EXPECT_EQ(points.Size(), 4);
}


TEST ( FindCloseStars, NotEnoughElements )
{
	const util::uint size = 10;
	util::ArrayList<Star, size> stars;
	util::ArrayList<Point<decimal>, size> points;

	Star s1(0, 1, 1);
	Star s2(0, 1.5, 2);
	Star s3(0, -2, 3);
	Star s4(0, -0.4, 4);

	stars.PushBack(s1);
	stars.PushBack(s2);
	stars.PushBack(s3);
	stars.PushBack(s4);

	star_tracker::Star::FindCloseStars<size, size>(0, 4, 1.5, stars, &points);

	EXPECT_EQ(points.Get(0).y, 1);
	EXPECT_EQ(points.Get(1).y, 1.5);
	EXPECT_EQ(points.Get(2).y, -0.4);
	EXPECT_EQ(points.Size(), 3);
}









TEST ( GreaterOperator, Basic )
{
	Star s1(1, 1, 2);
	Star s2(2, 2, 1);

	EXPECT_TRUE(s1 > s2);
	EXPECT_FALSE(s2 > s1);
}



TEST ( GreaterOperator, LinkedList )
{
	std::list<star_tracker::Star> star_list;
	star_tracker::Star s1(1, 1, 3);
	star_tracker::Star s2(2, 2, 2);
	star_tracker::Star s3(3, 3, 1);
	star_tracker::Star s4(4, 4, 0);

	star_list.push_back(s1);
	star_list.push_back(s3);
	star_list.push_back(s4);
	star_list.push_back(s2);

	star_list.sort(greater<Star>());
	EXPECT_FLOAT_EQ(s1.magnitude, 3);
	EXPECT_FLOAT_EQ(s2.magnitude, 2);
	EXPECT_FLOAT_EQ(s3.magnitude, 1);
	EXPECT_FLOAT_EQ(s4.magnitude, 0);
}
