//! Basic example that produces a 1Hz square-wave on Pin PE1

#![no_main]
#![no_std]
#![allow(unused_imports)]

// extern crate star_tracker_lib;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32h7xx_hal;
extern crate usb_device;
extern crate usbd_serial;
extern crate star_tracker_lib;

use stm32h7xx_hal::rcc::rec::UsbClkSel;
use stm32h7xx_hal::usb_hs::{UsbBus, USB1};
// use usb_device::prelude::*;
// use usbd_serial::{DefaultBufferStore, SerialPort};

#[macro_use]
mod utilities;

use stm32h7xx_hal::pac;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::stm32;
use stm32h7xx_hal::nb::block;

use core::fmt::Write;
// use nb::block;

use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::list::ArrayList;
use star_tracker_lib::util::list::List;
use star_tracker_lib::util::err::Errors;
use star_tracker_lib::util::err::Error;

use star_tracker_lib::image_processing::Blob;
use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::BasicImage;
use star_tracker_lib::image_processing::Threshold;
use star_tracker_lib::image_processing::ThresholdGrid;
use star_tracker_lib::image_processing::RefImage;


mod flash;


#[entry]
fn main() -> ! {
	utilities::logger::init();
	let cp = cortex_m::Peripherals::take().unwrap();
	let dp = pac::Peripherals::take().unwrap();

	// Constrain and Freeze power
	let pwr = dp.PWR.constrain();
	let pwrcfg = example_power!(pwr).freeze();

	// Constrain and Freeze clock
	let rcc = dp.RCC.constrain();
	let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);


	let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
	let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

	// Configure PE1 as output.
	let mut led_y = gpioe.pe1.into_push_pull_output();
	let mut led_r = gpiob.pb0.into_push_pull_output();
	let mut led_g = gpiob.pb14.into_push_pull_output();

	// Get the delay provider.
	let mut delay = cp.SYST.delay(ccdr.clocks);



	// serial
    // let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    // let tx = gpiod.pd9.into_alternate();
    // let rx = gpiod.pd8.into_alternate();

    // let serial = dp
    //     .USART3
    //     .serial((tx, rx), 9600.bps(), ccdr.peripheral.USART3, &ccdr.clocks)
    //     .unwrap();

    // let (mut tx, mut rx) = serial.split();

    // // core::fmt::Write is implemented for tx.
    // writeln!(tx, "Hello, world!").unwrap();

	




	loop {
		loop {
			led_r.set_high();
			led_y.set_low();
			led_g.set_low();
			delay.delay_ms(500_u16);
			
			led_r.set_low();
			led_y.set_high();
			led_g.set_low();
			delay.delay_ms(500_u16);
			
			led_r.set_low();
			led_y.set_low();
			led_g.set_high();
			delay.delay_ms(500_u16);
			
			// let received = block!(rx.read()).unwrap();
			// block!(tx.write(received)).ok();

			// let mut img = RefImage{img: &mut flash::img::img};
			// for y in 0..img.height() {
			// 	for x in 0..img.width() { 
			// 		let byte = flash::img::get_px(Pixel{x: x, y: y});
			// 		img.set(Pixel{x: x, y: y}, byte);
			// 	}
			// }
		}
	}
}
