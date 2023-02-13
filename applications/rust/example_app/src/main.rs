#![no_main]
#![no_std]

use libtock::buttons::Buttons;
use libtock::runtime::{set_main, stack_size};
use libtock::buttons::ButtonListener;
use libtock::platform::{share, Syscalls};
use libtock::runtime::TockSyscalls;

set_main! {main}
stack_size! {0x100}

fn main() {
    let listener = ButtonListener(|_button, _state| {
        // TODO when a console driver will be available
        // println!("button {}: {}", button, state);
    });
    if let Ok(_buttons_count) = Buttons::count() {
        if let Ok(()) = share::scope(|subscribe| Buttons::register_listener(&listener, subscribe)) {
            loop {
                TockSyscalls::yield_wait();
            }
        }
    }
}
