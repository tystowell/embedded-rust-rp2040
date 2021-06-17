#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use rp2040_pac;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[entry]
fn main() -> ! {
    let p = rp2040_pac::Peripherals::take().unwrap();

    let io = p.IO_BANK0;
    let sio = p.SIO;
    let pad = p.PADS_BANK0;

    let pin = 25;

    p.RESETS.reset.modify(|_, w| w.io_bank0().clear_bit());

    while p.RESETS.reset_done.read().io_bank0().bit_is_clear() {}

    p.RESETS.reset.modify(|_, w| w.pads_bank0().clear_bit());

    while p.RESETS.reset_done.read().pads_bank0().bit_is_clear() {}

    pad.gpio[pin].reset();

    io.gpio[pin].gpio_ctrl.write_with_zero(|x| x.funcsel().sio_0());

    sio.gpio_oe_set.write(|x| unsafe { x.bits(1 << pin) });

    loop {
        sio.gpio_out_set.write(|x| unsafe { x.bits(1 << pin) });

        cortex_m::asm::delay(500_000);

        sio.gpio_out_clr.write(|x| unsafe { x.bits(1 << pin) });

        cortex_m::asm::delay(500_000);
    }
}
