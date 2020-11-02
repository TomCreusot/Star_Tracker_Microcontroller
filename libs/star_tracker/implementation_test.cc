#include <ctime>
#include <stdlib.h>

#include <iostream>
#include "gtest/gtest.h"

#include "libs/star_tracker/star_set.h"
#include "libs/star_tracker/database.h"

#include "libs/nix/linked_list.h"

using namespace star_tracker;

Database GenerateEmptyDatabase ( )
{
	decimal fov = 0;
	vector<array<decimal, Database::kNumElements>>* array = {{}};
	Database database(fov, array);
	return database;
}





Database Generate1TriangleDatabase ( )
{
	decimal fov = 360;
	vector<array<decimal, Database::kNumElements>>* array = {{}};
	Database database(fov, array);
	return database;
}



/// @brief Creates a set of random points and adds specified points to start.
/// It then constructs the database.
///
/// @param num	The number of points.
/// @param fov	The field of view of the camera in radians.
/// @param pt1	The brightest point.
/// @param pt2	The other point.
/// @param pt3	The dullest point.
Database GenerateRandom (uint num, decimal fov, LinkedList<Point<decimal>>& pts)
{
	LinkedList<Point<decimal>> points;
	for ( uint i = 0; i < num; i++ )
	{
		decimal ra = fmod((decimal)rand(), 24);
		decimal dec = fmod((decimal)rand(), 180) - 90;
		Point<decimal> pt;
		pt.RaHour(ra);
		pt.DecDeg(dec);
		points.PushBack(pt);
	}
	for ( uint i = 0; i < pts.Size(); i++ )
		points.PushBack(pts.Get(i));

	LinkedList<StarSet> sets;
	StarSet::GenerateSets<0, 0>(points, &sets);
	static vector<array<decimal, Database::kNumElements>> d_array(sets.Size());

	for ( uint i = 0; i < d_array.size(); i++ )
		Database::ToArray(sets.Get(i), &d_array[i]);

	Database database(fov, &d_array);
	return database;
}




void PrintSet ( StarSet& set )
{
		cout <<
		"Ra: " << set.position.Ra() << ", Dec: " << set.position.Dec() <<
		",\t\t area: " << set.area << ", moment: " << set.moment <<
		",\t\tvote:" << set.vote << endl;
}





TEST ( Single, Accurate )
{

	srand (time(NULL));	// Starts random generation.
	LinkedList<Point<decimal>> pts;
	pts.PushBack(Point<decimal>(1, 2));
	pts.PushBack(Point<decimal>(1.1, 2.1));
	pts.PushBack(Point<decimal>(1.2, 2.3));

	Database database = GenerateRandom(10, M_PI, pts);


	StarSet set(pts.Get(0), pts.Get(1), pts.Get(2));

	const int kNumElementsIn = 1;
	const int kNumElementsOut = 500;
	ArrayList<StarSet, kNumElementsIn> sets_in;
	ArrayList<StarSet, kNumElementsOut> sets_out;

	// pt1.Ra(2.01);
	sets_in.PushBack(set);

	const decimal tolerance_area = 0.0001;
	const decimal tolerance_moment = 0.0001;

	database.FindElements
						<kNumElementsIn, kNumElementsOut>
						(sets_in, tolerance_area, tolerance_moment, &sets_out);

	StarSet::Vote<kNumElementsOut>(&sets_out);
	sets_out.Sort(StarSet::SortByVoteDecending);
	cout << "Database Size: " << database.database->size() << endl;

	cout << "Actual:" << endl;
	PrintSet(set);

	cout << endl << endl << "Found: " << sets_out.Size() << endl;

	for ( uint i = 0; i < sets_out.Size(); i++ )
	{
		PrintSet(sets_out.Get(i));
	}
	cout << endl << endl;

	EXPECT_FLOAT_EQ(set.position.x, sets_out.Get(0).position.x);
	EXPECT_FLOAT_EQ(set.position.y, sets_out.Get(0).position.y);
}




TEST ( Multiple, Accurate )
{
	LinkedList<Point<decimal>> pts;
	pts.PushBack(Point<decimal>(1, 2));
	pts.PushBack(Point<decimal>(1.1, 2.1));
	pts.PushBack(Point<decimal>(1.2, 2.3));
	pts.PushBack(Point<decimal>(1.5, 2.2));
	pts.PushBack(Point<decimal>(1.1, 2.1));
	pts.PushBack(Point<decimal>(2.1, 3.1));
	Database database = GenerateRandom(10, M_PI, pts);

	const uint kNumElementsIn = 50;
	const uint kNumElementsOut = 500;
	ArrayList<StarSet, kNumElementsIn> sets_in;
	ArrayList<StarSet, kNumElementsOut> sets_out;

	StarSet::GenerateSets<0, kNumElementsIn>(pts, &sets_in);
	const decimal tolerance_area = 0.003;
	const decimal tolerance_moment = 0.003;

	database.FindElements
						<kNumElementsIn, kNumElementsOut>
						(sets_in, tolerance_area, tolerance_moment, &sets_out);

	StarSet::Vote<kNumElementsOut>(&sets_out);
	sets_out.Sort(StarSet::SortByVoteDecending);
	cout << "Database Size: " << database.database->size() << endl;


	cout << endl << endl << "Found: " << sets_out.Size() << endl;


	for ( uint i = 0; i < sets_out.Size(); i++ )
	{
		PrintSet(sets_out.Get(i));
	}
	cout << endl << endl;
}
