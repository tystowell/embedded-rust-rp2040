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

    p.IO_BANK0.gpio[26].gpio_ctrl.write_with_zero(|x| x.funcsel().null()); // funcsel for ADC is NULL (generic type for peripherals only defined on a few pins)

    adc.cs.write(|w| unsafe { w.rrobin().bits(0x00) }); // Disable round robin sampling of pins

    adc.cs.write(|w| unsafe { w.ainsel().bits(0) }); // Set the current pin select to ADC0 (all the adc inputs are mux'ed to a single ADC)

    adc.cs.write(|w| w.en().set_bit()); // Enable the ADC
    
    loop {
        adc.cs.write(|w| w.start_once().set_bit());

        while adc.cs.read().ready().bit_is_clear() {}

        adc.result.read().result(); // Figure out how to unpack this type
    }
}
