#pragma once
#include <string>

#include "libs/util/util.h"


namespace config
{
/// The image file to read in and examine.
// const string	image_file 			= "test_images/Archernar_1h37m_57deg14m_45fov.bmp";

// const string	image_file			= "test_images/uCar_9h47m_-65deg04m_10fov.bmp";
// const string	image_file			= "test_images/Alnilam_5h36m_1deg12m_10fov.bmp";

// const string	image_file			= "test_images/Sirius_6h45m_-16deg43m_60fov.bmp";
// const string	image_file			= "test_images/exp200k_g8.bmp";
// const string	image_file			= "test_images/Archernar_1h37m_57deg14m_45fov.bmp";

const string	image_file			= "test_images/Adhara_6h58m_-28deg_51m_10fov.bmp";


/// The image file to write to show the outcome from BLOB detection.
const string	image_out_file 	= "out.bmp";

/// The maximum elements to display to the console.
const uint		max_display		= 5;
}
