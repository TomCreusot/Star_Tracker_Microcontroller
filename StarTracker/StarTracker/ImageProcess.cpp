#include "ImageProcess.h"

namespace ip
{

	/**
	 * @brief Gets all the points from an image specified.
	 * @param fileName The name of the file to test.
	 * @return The points/blobs of the image.
	 */

	std::vector<cv::KeyPoint>& ip::getPoints(const String& fileName)
	{
		cv::Mat img = imread(fileName, IMREAD_GRAYSCALE);

		cv::SimpleBlobDetector::Params params;

		// Change thresholds
		params.filterByColor = true;
		params.blobColor = 255;
		params.minThreshold = 100;
		params.maxThreshold = 255;

		// Filter by Area.
		params.filterByArea = false;

		// Filter by Circularity
		params.filterByCircularity = true;
		params.minCircularity = 0.1f;

		// Filter by Convexity
		params.filterByConvexity = true;
		params.minConvexity = 0.87f;

		// Filter by Inertia
		params.filterByInertia = true;
		params.minInertiaRatio = 0.01f;

		// Detect blobs.
		std::vector<KeyPoint> keypoints;
		cv::Ptr<cv::SimpleBlobDetector> detector = cv::SimpleBlobDetector::create(params);
		detector->detect(img, keypoints);

		return keypoints;
	}


	/**
	 * @brief			Finds the brightest points from the list.
	 * @param num		The number of stars to append to the list.
	 * @param points	The points to examine.
	 * @return			The brightest points (0 index is brightest).
	 *
	 * THE RETURNED POINTER MUST BE DISPOSED!
	 */
	
	cv::KeyPoint* ip::getMainPoints(const int num, const vector<KeyPoint>& points)
	{
		cv::KeyPoint* set = new cv::KeyPoint[num]();
		//Assigning values to smallest so they will be overwritten.
		KeyPoint smallest = points[0];
		for (int i = 0; i < points.size(); i++)
			if (smallest.size > points[i].size) smallest = points[i];
		for (int i = 0; i < num; i++) set[i] = smallest;

		//Gets the largest elements in the vector and swaps the values in the array.
		for (int ii = 0; ii < points.size(); ii++)
		{
			int jj = num - 1;

			//The first element in the array must be replaced not shifted.
			if (points[ii].size > set[jj].size) set[jj] = points[ii];
			jj--;

			//For all other elements in the array, swap every time greater.
			while (points[ii].size > set[jj].size && jj >= 0)
			{
				set[jj + 1] = set[jj];
				set[jj] = points[ii];
				jj--;
			}
		}
		return set;
	}















	/**
	 * @brief		For displaying the image, this is for debugging.
	 * @param set	The list of KeyPoints in an array to convert.
	 * @param num	The number of elements in the array.
	 * @return		The vector equivalent of set.
	 */

	vector<cv::KeyPoint>& ip::convertArrayToVector(const int num, const cv::KeyPoint* set)
	{
		vector<cv::KeyPoint> vector;
		for (int i = 0; i < num; i++)
		{
			vector.push_back(set[i]);
		}
		return vector;
	}




	/**
	 * @brief		Draws the image with the keypoints selected, this is for debugging.
	 * @param file	The file location and name.
	 * @param set	The vector containing all the values.
	 */

	void ip::drawImage(const String& file, const vector<cv::KeyPoint>& set)
	{
		cv::Mat img = imread(file);
		cv::Mat img_with_keypoints;
		drawKeypoints(img, set, img_with_keypoints, Scalar(0, 0, 255), DrawMatchesFlags::DRAW_RICH_KEYPOINTS);

		cv::imshow(file, img_with_keypoints);
		cv::waitKey();
	}

}