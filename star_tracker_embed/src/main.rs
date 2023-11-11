// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]
// #![macro_use]

// use defmt::*;
// use embassy_executor::Spawner;
// use embassy_net::tcp::TcpSocket;
// use embassy_net::{Ipv4Address, Stack, StackResources};
// use embassy_stm32::eth::generic_smi::GenericSMI;
// use embassy_stm32::eth::{Ethernet, PacketQueue};
// use embassy_stm32::peripherals::ETH;
// use embassy_stm32::rng::Rng;
// use embassy_stm32::{bind_interrupts, eth, peripherals, rng, Config};
// use embassy_time::Timer;
// // use embedded_io_async::Write;
// use rand_core::RngCore;
// use static_cell::make_static;
// use {defmt_rtt as _, panic_probe as _};

// use embassy_stm32::peripherals::RNG;
// // use crate::peripherals::RNG;
// // use embassy_stm32::interrupt::typelevel::*;


// bind_interrupts!(struct Irqs {
//     ETH => eth::InterruptHandler;
//     // RNG => rng::InterruptHandler<peripherals::RNG>;
//     HASH_RNG => rng::InterruptHandler<peripherals::RNG>;//embassy_stm32::interrupt::typelevel::HASH_RNG;
// });

// type Device = Ethernet<'static, ETH, GenericSMI>;

// #[embassy_executor::task]
// async fn net_task(stack: &'static Stack<Device>) -> ! {
//     stack.run().await
// }

// #[embassy_executor::main]
// async fn main(spawner: Spawner) -> ! {

//     info!("HI");

//     let mut config = Config::default();
//     {
//         use embassy_stm32::rcc::*;
//         config.rcc.hsi = Some(HSIPrescaler::DIV1);
//         config.rcc.csi = true;
//         config.rcc.hsi48 = true; // needed for RNG
//         config.rcc.pll1 = Some(Pll {
//             source: PllSource::HSI,
//             prediv: PllPreDiv::DIV4,
//             mul: PllMul::MUL50,
//             divp: Some(PllDiv::DIV2),
//             divq: None,
//             divr: None,
//         });
//         config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
//         config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
//         config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
//         config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
//         config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
//         config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
//         config.rcc.voltage_scale = VoltageScale::Scale1;
//     }

//     let p = embassy_stm32::init(config);

//     info!("Hello World!");
    
//     // Generate random seed.
//     let mut rng = Rng::new(p.RNG, Irqs);
//     let mut seed = [0; 8];
//     rng.fill_bytes(&mut seed);
//     let seed = u64::from_le_bytes(seed);
    
//     let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    
    
//     let device = Ethernet::new(
//             make_static!(PacketQueue::<16, 16>::new()),
//             p.ETH,
//             Irqs,
//             p.PA1,
//             p.PA2,
//             p.PC1,
//             p.PA7,
//             p.PC4,
//             p.PC5,
//             p.PG13,
//             p.PB13,
//             p.PG11,
//         GenericSMI::new(0),
//         mac_addr,
//     );
    
    // let config = embassy_net::Config::dhcpv4(Default::default());
    //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
        //    dns_servers: Vec::new(),
        //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
        //});
        

        // // Init network stack
        // let stack = &*make_static!(Stack::new(
            //     device,
            //     config,
            //     make_static!(StackResources::<2>::new()),
            //     seed
            // ));
            
    // // Launch network task
    // unwrap!(spawner.spawn(net_task(&stack)));

    // // Ensure DHCP configuration is up before trying connect
    // stack.wait_config_up().await;

    // info!("Network task initialized");

    // // Then we can use it!
    // let mut rx_buffer = [0; 1024];
    // let mut tx_buffer = [0; 1024];

    // loop {
    //     let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);

    //     socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    //     let remote_endpoint = (Ipv4Address::new(10, 42, 0, 1), 8000);
    //     info!("connecting...");
    //     let r = socket.connect(remote_endpoint).await;
    //     if let Err(e) = r {
    //         info!("connect error: {:?}", e);
    //         Timer::after_secs(1).await;
    //         continue;
    //     }
    //     info!("connected!");
    //     loop {
    //         // let r = socket.write_all(b"Hello\n").await;
    //         let r = socket.write(b"Hello\n").await;
    //         if let Err(e) = r {
    //             info!("write error: {:?}", e);
    //             break;
    //         }
    //         Timer::after_secs(1).await;
    //     }
    // }
// }

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{panic, *};
use embassy_executor::Spawner;
use embassy_stm32::usb_otg::{Driver, Instance};
use embassy_stm32::{bind_interrupts, peripherals, usb_otg, Config};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use futures::future::join;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.hsi48 = true; // needed for USB
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    let p = embassy_stm32::init(config);

    // Create the driver, from the HAL.
    let mut ep_out_buffer = [0u8; 256];
    let mut config = embassy_stm32::usb_otg::Config::default();
    config.vbus_detection = true;
    let driver = Driver::new_fs(p.USB_OTG_FS, Irqs, p.PA12, p.PA11, &mut ep_out_buffer, config);

    // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut control_buf,
    );

    // Create classes on the builder.
    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

	info!("READY");
    // Do stuff with the class!
    let echo_fut = async {
        loop {
            // class.wait_connection().await;
            info!("Connected");
            class.write_packet(&[b'O', b'\n', b'\r']);
            // let _ = echo(&mut class).await;
            // info!("Disconnected");
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, echo_fut).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn echo<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}

// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]

// use defmt::*;
// use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Input, Speed, Pull};
// use embassy_time::Timer;
// use {defmt_rtt as _, panic_probe as _};

// pub mod access_ram;
// use crate::access_ram::AccessRam;

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
// 	// defmt::trace!("trace");
//     info!("Hello!");
//     let p = embassy_stm32::init(Default::default());
//     info!("Hello World!");
// 	// defmt::trace!("trace2");

//     let mut led_r = Output::new(p.PB0, Level::High, Speed::Low);
//     let mut led_y = Output::new(p.PE1, Level::High, Speed::Low);
//     let mut led_g = Output::new(p.PB14, Level::High, Speed::Low);


// 	let trigger = Input::new(p.PG12, Pull::Up);

// 	defmt::info!("INFO");
// 	defmt::println!("PRINTLN");
// 	defmt::debug!("DEBUG");
// 	defmt::error!("ERROR");
// 	defmt::trace!("TRACE");
// 	println!("???");

// 	let ram;
// 	unsafe
// 	{
// 		ram = AccessRam::new(0x24000000 as *mut u32, 512/4);
// 	}
// 	let ram_arr = ram.as_bytes();
// 	for i in 0..ram_arr.len()
// 	{
// 		defmt::println!("{} {}", i, ram_arr[i]);
// 	}



//     loop {
//         led_r.set_high();
//         led_y.set_high();
//         led_g.set_high();
//         Timer::after_millis(500).await;

// 		println!("{}", trigger.is_low());
		
//         led_r.set_low();
//         led_y.set_low();
//         led_g.set_low();
//         Timer::after_millis(500).await;

//         // led_r.set_low();
//         // led_y.set_low();
//         // led_g.set_low();
//         // Timer::after_millis(500).await;
//     }
// }