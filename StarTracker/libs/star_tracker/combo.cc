#include "star_tracker.h"

namespace star_tracker
{
Combo::Combo ( )
{
	pilot = 0;
	s1 =	0;
	s2 = 	0;
	s3 = 	0;
}



Combo::Combo ( uint p, uint sA, uint sB, uint sC )
{
	pilot = p;
	s1 = sA;
	s2 = sB;
	s3 = sC;
}



bool Combo::equal ( uint p, uint sA, uint sB, uint sC )
{
	return p == pilot && s1 == sA && s2 == sB && s3 == sC;
}


}
