#include "libs/util/util.h"
#include "libs/nix/config.h"
#include "libs/nix/fill_template.h"
using namespace util;
using namespace nix;

int main ( int argc, char** argv )
{


	if ( argc == 2 )
	{
	/**********************************************************************
	 *			Gets properties input file.
	 */
		cout << "reading properties file:..." << endl;
		Config c;
		c.ReadFile(argv[1]);
		string read_file			= c.GetString("properties_template_file");
		string out_file				= c.GetString("properties_out_file");

	/**********************************************************************
	 *			Fills a template file with the required variables.
	 */
		nix::FillTemplate file_template;			// Setup Template
		file_template.AddKey("$(histogram_bars)",		c.GetString("histogram_bars"));
		file_template.AddKey("$(threshold_tolerance)",	c.GetString("threshold_tolerance"));
		file_template.AddKey("$(max_points)",			c.GetString("max_points"));
		file_template.AddKey("$(max_sets)",				c.GetString("max_sets"));
		file_template.AddKey("$(distance_tolerance)",	c.GetString("distance_tolerance"));
		file_template.AddKey("$(image_width)",			c.GetString("image_width"));
		file_template.AddKey("$(image_height)",			c.GetString("image_height"));
		file_template.AddKey("$(max_matches)",			c.GetString("max_matches"));
		file_template.AddKey("$(max_matches_per_star)",	c.GetString("max_matches_per_star"));

		file_template.AddKey("$(tolerance_area)",	c.GetString("tolerance_area"));
		file_template.AddKey("$(tolerance_moment)",	c.GetString("tolerance_moment"));

		file_template.ReplaceFile(	read_file, out_file	);
	}
	else
	{
		cout << "ERROR, Please Enter the name of the properties file" << endl;
	}
}
