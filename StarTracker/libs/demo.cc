/*
 *	File: 		demo.cc
 *	Author:		Tom Creusot
 *  Purpose:
 *				To visualy show the workings of the software.
 */

#include <iostream>
#include <chrono> // Get Time

#include "libs/get_image/get_image.h"
#include "libs/image_processing/image.h"
#include "libs/star_tracker/star_tracker.h"
#include "libs/database/find_elements.h"
#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"

using namespace util;
using namespace get_image;
using namespace image_processing;
using namespace star_tracker;
using namespace database;
using namespace std;




int main ( int argc, char** argv )
{
	if ( argc == 7 )
	{
		// Reading command line.
		char* file = argv[1];
		float threshold_agression = atof(argv[2]);
		float threshold_area = atof(argv[3]);
		int threshold_blobs = atoi(argv[4]);
		uint numStars = atoi(argv[5]);
		float fov = atof(argv[6]);

		// Reads in image.
		image_processing::Image img;
		get_image::GetImage get(file);		// Reads in image to bitmap.
		get.getImage(img);					// Copies bitmap to img.

		// Delete Background
		img.adaptiveThreshold(threshold_area, threshold_agression);


		// Shows the effects of increaseContrast and adaptiveThreshold.
		get.copyToBMP(img);

		// Finds blobs.
		util::ArrayList<image_processing::Blob> blobs;
		image_processing::Blob::findBlobs(img, threshold_blobs, blobs);

		// Gets the brightest stars from the list
		uint size = (blobs.size() > numStars ? numStars : blobs.size());
		cout << "Found blobs: " << blobs.size() << ", Using: " << size << endl;
		util::ArrayList<util::Point<util::decimal>> reduced;
		star_tracker::deriveBrightest(blobs, reduced, size);


		// Writes to file.
		get.drawPoints(reduced, 255, 0, 0);
		get.writeImage((char*)"out.bmp");


		// Creates a set of every combination of angles to calculate.
		util::ArrayList<star_tracker::Combo> combos;
		star_tracker::combinationsPilots(3, reduced.size(), combos);

		cout << "Number of combinations: " << combos.size() << endl;

		// Calculates the angles.
		util::ArrayList<database::AngleStat> angles;
		findAngles ( reduced, combos, angles );

		util::ArrayList<database::AngleStat> database_angles;

		for ( uint i = 0; i < angles.size(); i++ )
		{
			// angles[i].personalProbability();
			find_elements(angles[i], 0.1, database_angles);
		}
		// Reduces the database_angles odds depending on how clustered they are.
		AngleStat::clusterProbability(database_angles, 0.001, fov);


		// Prints in order of odds.
		uint curr = 0;

		for ( uint i = 0; i < database_angles.size(); i++)
		{
			if ( database_angles[i].odds > database_angles[curr].odds ) curr = i;
		}

		cout << database_angles[curr].pilot.x << "\t" << database_angles[curr].pilot.y << "\t\t" << database_angles[curr].odds << endl;


		for ( uint i = 0; i < database_angles.size(); i++ )
		{
			cout << database_angles[i].pilot.x << "\t" << database_angles[i].pilot.y << "\t\t" << database_angles[i].odds << endl;
		}
	}
	else
	{
		cout << "ERROR, Please Enter:"
			 << "\n\tThe image to read"
			 << "\n\tThe tolerance of the threshold"
			 << "\n\tThe sample area of the threshold"
			 << "\n\tThe min brightness of a blob"
			 << "\n\tThe max number of stars to display"
			 << "\n\tThe field of view of the camera"
			 << endl;
	}


}
