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
    // TODO store the status of the doors
    // - 2 doors
    // - true for closed
    // - false for open

    // TODO store the tenmperature value
    // - i32 in hunderds of centigrades
}

impl SmartHome {
    pub fn new() -> SmartHome {
        // TODO implement the `new` function
        SmartHome {  }
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
            // TODO implement the commands
            // - DRIVER_EXISTS -> CommandReturn::success()

            // - SET_DOORS -> CommandReturn::success_u32_u32(door1, door2)
            // - SET_TEMPERATURE -> Command_return::success_u32(temperature)
            // Hint: make sure you use interior mutability (Cell) for the values
            
            // - GET_DOORS
            // - GET_TEMPERATURE
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: kernel::ProcessId) -> Result<(), kernel::process::Error> {
        Ok(())
    }
}
