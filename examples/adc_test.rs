#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::OutputPin;
use rp2040_pac;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[entry]
fn main() -> ! {
    let p = rp2040_pac::Peripherals::take().unwrap();

    let adc = p.ADC;

    p.RESETS.reset.modify(|_, w| w.adc().clear_bit());

    while p.RESETS.reset_done.read().adc().bit_is_clear() {}

    //funcsel doesn't appear to have an option for adc?
    //p.IO_BANK0.gpio[26].gpio_ctrl.write_with_zero(|x| x.funcsel().sio_0());

    adc.cs.write(|w| unsafe { w.rrobin().bits(0x00) });

    adc.cs.write(|w| unsafe { w.ainsel().bits(0) });

    adc.cs.write(|w| w.en().set_bit());
    
    loop {
        adc.cs.write(|w| w.start_once().set_bit());

        while adc.cs.read().ready().bit_is_clear() {}

        adc.result.read().result() // Figure out how to unpack this type
    }
}
