#![no_std]
#![no_main]

use arduino_hal::{default_serial, delay_ms};
use avr_hal_generic::void::Void;
use panic_halt as _;
use ufmt::{uWrite, uwriteln};

//
//
//

#[arduino_hal::entry]
fn main() -> ! {
    //

    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);

    let mut serial = default_serial!(dp, pins, 115200);

    let _ = uwriteln!(&mut serial, "Hello, world!");

    //

    if true {
        debug_check1(&mut serial);
    }

    loop {
        if true {
            delay_ms(1000);
        }
    }
}

fn debug_check1(serial: &mut dyn uWrite<Error = Void>) {
    use byte_slice_cast::AsByteSlice;
    let pixels = [0xf800u16, 0x07e0, 0x001f];
    // let be0 = pixels[0].swap_bytes();

    let mut buf = [0u16; 3];

    {
        for (i, pixel) in pixels.iter().enumerate() {
            buf[i] = pixel.to_be();
        }
        let _ = uwriteln!(serial, "debug buf {:?} enumerate", buf);
    }

    let boo = buf.as_byte_slice();

    let _ = uwriteln!(serial, "debug as_byte_slice() {:?}", boo);
}

//

/*
cargo build --release
elf=$(echo target/avr-atmega328p/release/*.elf) &&
avrdude -C /etc/avrdude.conf -v -p atmega328p -c arduino -P /dev/ttyACM0  -D -Uflash:w:$elf:e
 */

*/
