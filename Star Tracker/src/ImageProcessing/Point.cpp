/*
 *	File: 		Point.cpp
 *	Author:		Tom Creusot
 *  Purpose:	A class template that an x and y coordinate.
 */

#ifndef POINT_CPP
#define POINT_CPP

#ifdef COMPUTER
	#include <iostream>
	using namespace std;
#endif

namespace ip
{
template <class T> class Point
{
public:
 	T x, y;


	/**
	 * @brief Default Constructor
	 * Sets all values to 0.
	 */

	Point ( )
	{
		x = 0;
		y = 0;
	}

	/**
	* @brief 	Alternate Constructor
	* @param x The x position.
	* @param y The y position.
	*/

	Point ( T x_, T y_ )
	{
		x = x_;
		y = y_;
	}


	/**
	 * @brief	Alternate Constructor.
	 *			Creates a Point object with the ascention/declination coordinate
	 * @param degreesX	The whole decimal.
	 * @param minutesX	The whole minute.
	 * @param secondsX	Seconds and anything bellow.
	 */

	Point  ( 	T degreesX, T minutesX, T secondsX,
				T degreesY, T minutesY, T secondsY	)
	{
		set ( degreesX, minutesX, secondsX, degreesY, minutesY, secondsY );
	}


	/**
	 * @brief		Sets the position of the x and y.
	 * @param x_	The x position.
	 * @param y_	The y position.
	 */

	void set ( T x_, T y_ )
	{
		x = x_;
		y = y_;
	}


	/**
	 * @brief Creates a Point object with the ascention/declination coordinate.
	 * @param degreesX	The whole decimal.
	 * @param minutesX	The whole minute.
	 * @param secondsX	Seconds and anything bellow.
	 */
	void set (	T degreesX, T minutesX, T secondsX,
				T degreesY, T minutesY, T secondsY	)
	{
		x = degreesX + minutesX / 60 + secondsX / 3600;
		y = degreesY + minutesY / 60 + secondsY / 3600;
	}


	/**
	* @brief		Finds the hypotenues between this and the other point.
	* @param other The other point to measure with.
	* @return		The distance between the points.
	*/

	T distance ( Point<T> other )
	{
		return (T) hypot(x - other.x, y - other.y);
	}



	/**
	* @brief		Finds if othe point is the same position.
	* @param other The other point to test against.
	* @return 		True if x and y are equal to this x and y.
	*/

	bool equal ( Point<T> other )
	{
		return abs(x - other.x) < 0.0001 && abs(y - other.y) < 0.0001;
	}



#ifdef COMPUTER
	void print ( )
	{
		std::cout << "x: " << x << ",\ty: " << y;
	}
#endif

};

}
#endif
