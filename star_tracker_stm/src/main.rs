//! Basic example that produces a 1Hz square-wave on Pin PE1

#![no_main]
#![no_std]

extern crate star_tracker_lib;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate log;
extern crate panic_halt;
extern crate stm32h7xx_hal;
extern crate usb_device;
extern crate usbd_serial;

use star_tracker_lib::util::*;
use star_tracker_lib::util::units::*;
use star_tracker_lib::projection::*;
use star_tracker_lib::image_processing::*;
use star_tracker_lib::tracking_mode::*;
use star_tracker_lib::tracking_mode::database::*;
use star_tracker_lib::attitude_determination::*;

use stm32h7xx_hal::rcc::rec::UsbClkSel;
use stm32h7xx_hal::usb_hs::{UsbBus, USB1};
use usb_device::prelude::*;
use usbd_serial::{DefaultBufferStore, SerialPort};

#[macro_use]
mod utilities;
use log::info;
#[macro_use(info)]
use utilities::logger;

use stm32h7xx_hal::pac;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::stm32;
use stm32h7xx_hal::nb::block;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    utilities::logger::init();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    info!("Setup PWR...                  ");
    let pwr = dp.PWR.constrain();
    let pwrcfg = example_power!(pwr).freeze();

    // Constrain and Freeze clock
    info!("Setup RCC...                  ");
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(160.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOC peripheral. This also enables the clock for
    // GPIOC in the RCC register.
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    let tx = gpioc.pc10.into_alternate();
    let rx = gpioc.pc11.into_alternate();

    info!("");
    info!("stm32h7xx-hal example - USART");
    info!("");

    // Configure the serial peripheral.
    let serial = dp
        .USART3
        .serial((tx, rx), 19_200.bps(), ccdr.peripheral.USART3, &ccdr.clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello, world!").unwrap();
	
    loop {
		// Echo what is received on the serial link.
		writeln!(tx, "Hello, world!").unwrap();
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
/*
#[entry]

// `main` is not allowed to return
fn main() -> ! {
	// utilities::logger::init();
	let cp = cortex_m::Peripherals::take().unwrap();
	let dp = pac::Peripherals::take().unwrap();

	// Constrain and Freeze power
	info!("Setup PWR...                  ");
	let pwr = dp.PWR.constrain();
	let pwrcfg = example_power!(pwr).freeze();

	// Constrain and Freeze clock
	info!("Setup RCC...                  ");
	let rcc = dp.RCC.constrain();
	let mut ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

	info!("Setup serial clock?");
	let _ = ccdr.clocks.hsi48_ck().expect("HSI48 must run");
	ccdr.peripheral.kernel_usb_clk_mux(UsbClkSel::Hsi48);
	
	// #[cfg(any(feature = "rm0433", feature = "rm0399"))]
	// let (pin_dm, pin_dp) = {
	// 	let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
	// 	(gpiob.pb14.into_alternate(), gpiob.pb15.into_alternate())
	// };
	
	
	// #[cfg(any(feature = "rm0455", feature = "rm0468"))]
    let (pin_dm, pin_dp) = {
        let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
        (gpioa.pa11, gpioa.pa12)
        // (gpioa.pa11.into_alternate(), gpioa.pa12.into_alternate())
    };
	
	let usb = USB1::new_unchecked(
		dp.OTG1_HS_GLOBAL,
		dp.OTG1_HS_DEVICE,
		dp.OTG1_HS_PWRCLK,
		// pin_dm,
		// pin_dp,
		ccdr.peripheral.USB1OTG,
		&ccdr.clocks,
	);
	
	let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

	let mut serial = usbd_serial::SerialPort::new(&usb_bus);

	let usb_dev =
		UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
			.manufacturer("Fake company")
			.product("Serial port")
			.serial_number("TEST")
			.device_class(usbd_serial::USB_CLASS_CDC)
			.build();

    info!("");
    info!("stm32h7xx-hal example - Blinky");
    info!("");

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    // let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure PE1 as output.
	// let mut led_ld1 = gpiob.pb0.into_push_pull_output();
    let mut led_ld2 = gpioe.pe1.into_push_pull_output();
    // let mut led_ld3 = gpiob.pb14.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

	// led_e.set_high();
	// led_b.set_high();
	// delay.delay_ms(500_u16);
	// 
	// led_e.set_low();
	// led_b.set_low();
	// delay.delay_ms(500_u16);

    loop 
	{
        loop
		{
			// led_ld1.toggle();
			led_ld2.toggle();
			// led_ld3.toggle();
			// led_ld1.set_low();
			// led_ld2.set_low();
			// led_ld3.set_high();
			serial.write(&[b'a', b' ', b'b']);
			delay.delay_ms(1000_u16);
			// 
            // delay.delay_ms(500_u16);
        }
    }
}

// // define the hard fault handler
// exception!(HardFault, hard_fault);
// 
// fn hard_fault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// define the default exception handler
// exception!(*, default_handler);
// 
// fn default_handler(irqn: i16) {
//     panic!("unhandled exception (IRQn={})", irqn);
// }
*/