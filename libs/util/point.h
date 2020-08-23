/**
 *  A class template that stores an x and y coordinate.
 *	@file		point.h
 *	@author		Tom Creusot
 */

#pragma once
#include <math.h>
#include <assert.h>

namespace util
{
// Forward Declaration
template <class T> class Point;

/// @var Point describing a Cartesian Coordinate.
template <class T>	using Cartesian = Point<T>;
template <class T>	using Equatorial = Point<T>;




/**
 *  A class template that stores an x and y coordinates.
 *	Consider x as Right Ascension and y as Declination.
 *	There are several typedefs for the class depending on what system the object is using.
 *
 *	@tparam T	The datatype to store.
 *
 *	@example
 *		Point<int> point(1, 0);
 *		point.x = 10;
 *		Point<int> point2(10, 10);
 *		cout << point2.Distance(point) << endl;	// 10
 *		cout << point2.Equal(point) << endl;	// false
 *
 *
 *	@author		Tom Creusot
 */

template <class T> class Point
{
public:
	/// The x variable.
	T x;
	/// The y variable.
	T y;


	/**
	 * @brief 	Default Constructor.
	 * @details	Sets all values to 0.
	 */

	Point ( )
	{
		x = 0;
		y = 0;
	}


	/**
	 * @brief		Alternate Constructor.
	 * @param val	The value to set x and y.
	 * @details		This is for testing code of users of Point.
	 */

	Point ( T val )
	{
		x = val;
		y = val;
	}


	/**
	 * @brief 	Alternate Constructor
	 * @param x	The x position.
	 * @param y	The y position.
	 */

	Point ( T x, T y )
	{
		this->x = x;
		this->y = y;
	}

	/**
	 * @brief			Copy Constructor
	 * @param p [in]	The value to copy.
	 */

	Point ( const Point& p )
	{
		x = p.x;
		y = p.y;
	}



	/// @brief		Alias for x.
	/// @return		x.
	T Ra 		( )			{	return this->x;		}

	/// @brief		Alias for x (COPY).
	/// @param x	The value to assign to x.
	void Ra 	( T x )		{	this->x = x;	}



	/// @brief		Alias for right ascention with 24 hour time.
	/// @return		The 0-24h Range of 0-2PIdeg.
	T RaHour	( )			{	return this->x * 12.0 / M_PI;	}

	/// @brief		Alias for right ascention from a Range of 0 to 24h instead of 0 to 2PIdeg.
	/// @param x	The value to assign to x.
	void RaHour	( T x )		{	this->x = x * M_PI / 12.0;	}

	/// @brief		Returns the angle in degrees.
	/// @return		The angle in degrees.
	T DecDeg	( )			{	return this->y * 180 / M_PI;	}
	/// @brief		Alias for declination from a Range of 0 to 180h instead of 0 to 2PIdeg.
	/// @param y	The value of Declination in degrees.
	void DecDeg	( T y )		{	this->y = y * M_PI / 180;	}

	/// @brief		Alias for y.
	/// @return		y.
	T Dec 		( )			{	return this->y;		}

	/// @brief		Alias for y (COPY).
	/// @param y	The value to assign to y.
	void Dec	( T y )		{	this->y = y;	}


	/**
	 * @brief		Sets the position of the x and y (for malloced pointers).
	 * @param x		The x position.
	 * @param y		The y position.
	 */

	void Set ( T x, T y )
	{
		this->x = x;
		this->y = y;
	}









	/**
	 * @brief		Finds the hypotenues between this and the other point in cartesian coordinates.
	 * @param [in]	other The other point to measure with.
	 * @return		The distance between the points.
	 */

	T Distance ( Cartesian<T>& other ) const
	{
		return (T) hypot(x - other.x, y - other.y);
	}



	/**
	 * @brief		The angle between 2 points, considering the plane is curved to a field of view.
	 * @param rad_per_pixel	The angle between each pixel.
	 * @param other [in]	The other point to find the distance of.
	 * @return		The vector difference.
	 */

	T RadialDistance ( T rad_per_pixel, Cartesian<T>& other )
	{
		return Distance(other) * rad_per_pixel;
	}


	/**
	 * @brief		Finds the Radial distance on a unit sphere between the equitorial points.
	 * @param other [in]	The other point to find the distance of.
	 * @return		The vector difference.
	 */

	T RadialDistance ( Equatorial<T>& other )
	{
		T ascention = Ra() - other.Ra();
		return acos(	sin(Dec()) * sin(other.Dec()) +
						cos(Dec()) * cos(other.Dec()) * cos(ascention)	);
	}










	/**
	 * @brief	The magnitude of the vector.
	 * @return	The magnitude of the vector.
	 */

	T Magnitude ( )
	{
		return std::hypot(x, y);
	}


	/**
	 * @brief		Finds if other point is the same position.
	 * @param [in]	other The other point to test against.
	 * @return 		True if x and y are equal to this x and y.
	 */

	bool Equal ( Point<T>& other ) const
	{
		return Equal(other.x, other.y);
	}


	/**
	 * @brief			Finds if the other point is the same position.
	 * @param xx 		The x position to compare to.
	 * @param yy 		The y position to compare to.
	 * @return 			True if x and y are equal to xx and yy.
	 */

	bool Equal ( T xx, T yy ) const
	{
		return fabs(x - xx) < 0.0001 && fabs(y - yy) < 0.0001;
	}

};
}
