#include <stdio.h>
#include <stdlib.h>
#include "pico/stdlib.h"
// #include "star_tracker_rp"


void run ();

uint32_t clock (  )
{
	return time_us_32();
}


bool serial_available ( ) 
{
	return uart_is_readable(uart0);
}

unsigned char read_byte ( )
{
	// Read bytes from the serial port.
	char buffer[1];
    uart_read_blocking(uart0, buffer, sizeof(buffer));
	return buffer[0];
}

void print_string ( char* text ) { printf(text); }
    // Create a buffer to store the incoming bytes.


int main ( )
{    
	const uint led_pin = 25;

	uart_init(uart0, 9600);
	stdio_init_all();                // Serial
	gpio_init(led_pin);              // Setup LED
	gpio_set_dir(led_pin, GPIO_OUT); // LED OUTPUT


	// WrappedImage img;
	// img.rows = 30;
	// img.cols = 30;
	// img.img = malloc(sizeof(unsigned char) * img.rows * img.cols);

	// WrappedList stars_2d;
	// stars_2d.end = 0;
	// stars_2d.size = 100;
	// stars_2d.list = malloc(sizeof())

	while ( true )
	{
		run();
		// printf("%d %d\n", img.rows, img.cols);
		// for ( int x = 0; x < img.cols; x++ )
		// 	for ( int y = 0; y < img.rows; y++ )
		// 		img.img[y + x * img.rows] = x + y;


		// testing(&img);

		// for ( int y = 0; y < img.rows; y++ )
		// {
		// 	for ( int x = 0; x < img.cols; x++ )
		// 		printf("%d\t", img.img[y + x * img.rows]);
		// 	printf("\n\n");
		// }
		// printf("\n\n\n\n\n\n\n\n\n\n");


		// gpio_put(led_pin, true);
		// sleep_ms(100); // Puts processor into low power.
		// gpio_put(led_pin, false);
		// sleep_ms(1000);
		
		// find_stars(img, 50, 1, &stars_2d);

	}
}