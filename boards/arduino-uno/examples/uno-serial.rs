#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;
use arduino_uno::prelude::*;

// This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
// OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 57600

#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
    }
}