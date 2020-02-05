#include "get_image.hpp"

namespace gi
{



GetImage ( std::string& f )
{
	file = f;
}


ImageInterface getImage ( )
{
	BMP image = ReadFromFile(file);
	ip::Image im(image.TellWidth(), image.TellHeight());

	for ( int xx = 0; xx < im.getWidth(); xx++ )
	{
		for (int yy = 0; yy < im.getHeight(); yy++ )
		{
			im.setPixel(xx, yy,
							(	image(xx, yy) -> Red / 3 +
								image(xx, yy) -> Green / 3 +
								image(xx, yy) -> Blue / 3
							));
		}
	}
	return im;
}



}
