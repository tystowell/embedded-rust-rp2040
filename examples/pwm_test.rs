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

    pad.gpio[0].reset();

    //pwm.ch0_csr.write(|w| w.ph_correct().set_bit());

    pwm.ch0_top.write(|w| unsafe { w.ch0_top().bits(0xfffe) });

    //pwm.ch0_cc.write(|w| unsafe { w.b().bits(0x7fff) });

    pwm.ch0_csr.write(|w| w.en().set_bit());

    io.gpio[25].gpio_ctrl.write_with_zero(|w| w.funcsel().pwm_a_0());

    loop {
        pwm.ch0_cc.write(|w| unsafe { w.a().bits(0x0000) });

        cortex_m::asm::delay(500_000);

        pwm.ch0_cc.write(|w| unsafe { w.a().bits(0xffff) });

        cortex_m::asm::delay(500_000);
    }
}
