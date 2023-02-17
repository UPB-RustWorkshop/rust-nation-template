//! An extremely simple libtock-rs example. Register button events.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::platform::{ErrorCode, Syscalls};
use libtock::runtime::TockSyscalls;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {1024}

const DRIVER_NUM_SMARTHOME: u32 = 0xa0000;

mod command {
    pub const DRIVER_EXISTS: u32 = 0;

    pub const SET_DOORS: u32 = 101;
    pub const SET_TEMPERATURE: u32 = 102;

    // pub const GET_DOORS: u32 = 201;
    // pub const GET_TEMPERATURE: u32 = 202;
}

struct SmartHome;

impl SmartHome {
    pub fn exists() -> bool {
        TockSyscalls::command(DRIVER_NUM_SMARTHOME, command::DRIVER_EXISTS, 0, 0).is_success()
    }

    // TODO implement set_doors 
    // Hint: use `to_result`

    // TODO implement set_temperature 
    // Hint: use `to_result`
}

fn main() {
    writeln!(Console::writer(), "Service started\n").unwrap();
    if SmartHome::exists() {
        loop {
            // TODO read the temperature
            // Hint: look at `libtock-rs/examples/temperature.rs`
            
            // TODO set the temperature using `set_temperature`

            // TODO read the doors status
            // Hint: look at `libtock-rs/examples/buttons.rs`
            
            // TODO set the doors status using `set_doors`

            // TODO sleep for 500 ms
            // Hint: look at `libtock-rs/examples/alarm.rs`
        }
    } else {
        writeln!(Console::writer(), "Smarthome driver is not available").unwrap();
    }
}
