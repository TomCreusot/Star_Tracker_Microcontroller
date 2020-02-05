#include "gtest/gtest.h"
#include "get_image.hpp"

using namespace gi;
using namespace ip;


TEST ( GetImage, ValidFileName )
{
	gi::GetImage getI("test_img.bmp");
	ip::ImageInterface img = (gi::GetImageInterface) getI.getImage();

}


TEST ( GetImage, InvalidFileName )
{



}
