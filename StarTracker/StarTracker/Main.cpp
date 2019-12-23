#include <opencv2/opencv.hpp>
#include <IOStream>
#include "ImageProcess.h"
using namespace cv;
using namespace std;
using namespace ip;

int main()
{
	String file = "C:\\Users\\soyuz\\Desktop\\orion.jpg";
	vector<KeyPoint> points = ip::getPoints(file);

	KeyPoint* main = ip::getMainPoints(30, points);
	vector<KeyPoint> mainPoints = ip::convertArrayToVector(30, main);

	float prev = main[0].size;
	for (int i = 0; i < 30; i++)
	{
		cout << main[i].size << "\t" << main[i].pt.x << endl;
	}

	ip::drawImage(file, mainPoints);
}