#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use rp2040_pac;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[entry]
fn main() -> ! {
    let p = rp2040_pac::Peripherals::take().unwrap();

    let pwm = p.PWM;
    let io = p.IO_BANK0;
    let pads = p.PADS_BANK0;



    p.RESETS.reset.modify(|_, w| w.pwm().clear_bit());

    while p.RESETS.reset_done.read().pwm().bit_is_clear() {}

    p.RESETS.reset.modify(|_, w| w.pads_bank0().clear_bit());

    while p.RESETS.reset_done.read().pads_bank0().bit_is_clear() {}

    pads.gpio[0].reset();



    pwm.ch0_csr.write(|w| w.ph_correct().clear_bit()); //Disable ph_correct

    pwm.ch0_div.write(|w| unsafe { w.int().bits(0x01u8) }); //Set clock divider to 1

    pwm.ch0_csr.write(|w| w.divmode().div()); //Sets counter to go at rate dictated by fractional divider (not gated)

    pwm.ch0_csr.write(|w| w.a_inv().clear_bit()); //Don't invert channel A

    pwm.ch0_csr.write(|w| w.b_inv().clear_bit()); //Don't invert channel B

    pwm.ch0_top.write(|w| unsafe { w.ch0_top().bits(0xffffu16) }); //Set TOP register to max value


    pwm.ch0_cc.write(|w| unsafe { w.a().bits(0x7fffu16) }); //Default duty cycle of 50%

    pwm.ch0_ctr.write(|w| unsafe { w.ch0_ctr().bits(0x0000u16) }); //Reset the counter

    pwm.ch0_csr.write(|w| w.en().set_bit()); // Enable the PWM channel

    // Pad input enable and output disable take preference over any peripherals using it.
    // According to pico-sdk, enabling inputs and outputs through the pad lets the peripheral control it's own behavior.
    pads.gpio[0].write(|w| w.ie().set_bit());
    pads.gpio[0].write(|w| w.od().clear_bit());

    io.gpio[0].gpio_ctrl.write_with_zero(|w| w.funcsel().pwm_a_0()); // Connect the gpio pin with the pwm

    loop {
        pwm.ch0_cc.write(|w| unsafe { w.a().bits(0x4000u16) });

        cortex_m::asm::delay(900_000);

        pwm.ch0_cc.write(|w| unsafe { w.a().bits(0xa000u16) });

        cortex_m::asm::delay(900_000);
    }
}
