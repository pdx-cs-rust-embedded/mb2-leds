#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // take the peripherals
    let board = Board::take().unwrap();
    let pins = board.display_pins;

    let image = [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 0, 0, 0, 1],
    ];

    let mut delay = timer::Timer::new(board.TIMER0);

    let mut display = Display::new(pins);
    loop {
        rprintln!("showing");
        display.show(&mut delay, image, 2000);
        rprintln!("sleeping");
        delay.delay_ms(1000u16);
    }

}
