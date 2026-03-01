use std::{
    io,
    process::{Child, Command},
};

pub type PID = u32;

pub trait Process {
    fn pid(&self) -> PID;
}

pub struct ChildProcess {
    ps: Child,
}

impl ChildProcess {
    pub fn new(mut cmd: Command) -> io::Result<Self> {
        cmd.spawn().map(|ps| ChildProcess { ps })
    }
}

impl Drop for ChildProcess {
    fn drop(&mut self) {
        let w = self.ps.try_wait();
        match w {
            Ok(_) => (),
            Err(err) if err.raw_os_error().unwrap() == 10 => (),
            Err(_) => self.ps.kill().unwrap(),
        }
    }
}

impl Process for ChildProcess {
    fn pid(&self) -> PID {
        self.ps.id()
    }
}
