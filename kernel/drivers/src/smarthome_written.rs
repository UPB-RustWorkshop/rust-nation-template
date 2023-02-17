use core::cell::Cell;

use kernel::{
    syscall::{CommandReturn, SyscallDriver},
    ErrorCode,
};

pub const DRIVER_NUM: usize = 0xa0000;

mod command {
    pub const DRIVER_EXISTS: usize = 0;

    pub const SET_DOORS: usize = 101;
    pub const SET_TEMPERATURE: usize = 102;

    pub const GET_DOORS: usize = 201;
    pub const GET_TEMPERATURE: usize = 202;
}

pub struct SmartHome {
    door_a: Cell<bool>,
    door_b: Cell<bool>,
    temperature: Cell<i32>,
}

impl SmartHome {
    pub fn new() -> SmartHome {
        SmartHome {
            door_a: Cell::new(false),
            door_b: Cell::new(false),
            temperature: Cell::new(0),
        }
    }
}

impl SyscallDriver for SmartHome {
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        data2: usize,
        _process_id: kernel::ProcessId,
    ) -> CommandReturn {
        match command_num {
            command::DRIVER_EXISTS => CommandReturn::success(),
            command::SET_DOORS => {
                self.door_a.set(data1 != 0);
                self.door_b.set(data2 != 0);
                CommandReturn::success()
            }
            command::SET_TEMPERATURE => {
                self.temperature.set(data1 as i32);
                CommandReturn::success()
            }

            command::GET_DOORS => {
                CommandReturn::success_u32_u32(self.door_a.get() as u32, self.door_b.get() as u32)
            }

            command::GET_TEMPERATURE => CommandReturn::success_u32(self.temperature.get() as u32),

            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: kernel::ProcessId) -> Result<(), kernel::process::Error> {
        Ok(())
    }
}
