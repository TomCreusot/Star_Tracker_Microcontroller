#ifndef IMAGE_PROCESS_H
#define IMAGE_PROCESS_H

#include <opencv2/opencv.hpp>
using namespace cv;
using namespace std;

namespace ip
{

std::vector<cv::KeyPoint>& getPoints(const String& fileName);

cv::KeyPoint* getMainPoints(const int num, const std::vector<cv::KeyPoint>& points);

//For debugging
vector<cv::KeyPoint>& convertArrayToVector(const int num, const cv::KeyPoint* set);
void drawImage(const String& file, const vector<cv::KeyPoint>& set);


}


#endif