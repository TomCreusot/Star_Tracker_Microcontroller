#ifndef STAR_TRACKER_H
#define STAR_TRACKER_H

#include <opencv2/opencv.hpp>
#include <math.h>
using namespace cv;
using namespace std;

namespace st
{
	vector<float>& findAngles(const int num, const KeyPoint* set);

	float findAngle(const KeyPoint& pilot, const KeyPoint& node1, const KeyPoint& node2, const KeyPoint& node3);
}

#endif
