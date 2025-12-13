use std::{
    io,
    process::{Child, Command},
};

use libc::{c_int, waitpid};

pub trait Process {
    fn pid(&self) -> u32;
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
            Err(err) if err.raw_os_error().unwrap() == 10 => (), // this is the case where all
            // child processes exited and now
            // it throws error for some
            // reason?
            Err(_) => self.ps.kill().unwrap(),
        }
    }
}

impl Process for ChildProcess {
    fn pid(&self) -> u32 {
        self.ps.id()
    }
}

pub struct ProcessStopper<'a, Ps: Process> {
    ps: &'a Ps,
}

fn report_scall_panic(call: &str) {
    let error = io::Error::last_os_error().raw_os_error();
    panic!("{} failed. errno: {}", call, error.unwrap_or(0),);
}

fn report_kill_panic<Ps: Process>(ps: &Ps, sig: c_int) {
    report_scall_panic(&format!("Kill with signal {} to {}", sig, ps.pid()));
}

impl<'a, Ps: Process> ProcessStopper<'a, Ps> {
    pub fn new(ps: &'a Ps) -> ProcessStopper<'a, Ps> {
        unsafe {
            let res = libc::kill(ps.pid() as i32, libc::SIGSTOP);
            if res != 0 {
                report_kill_panic(ps, libc::SIGSTOP);
            }
            let mut status: c_int = 0;
            let res = waitpid(ps.pid() as i32, &mut status, libc::WUNTRACED);
            if res != ps.pid() as i32 {
                if res == -1 {
                    report_scall_panic("waitpid");
                } else {
                    panic!("unexpected pid {} returned", res);
                }
            }
            if !libc::WIFSTOPPED(status) {
                panic!("Process did {} instead of stopping", status); // im really unsure about
                // amount of panics there.
                // perhaps i should write it
                // all into Results.
            }
        }
        ProcessStopper { ps }
    }
}

impl<'a, Ps: Process> Drop for ProcessStopper<'a, Ps> {
    fn drop(&mut self) {
        unsafe {
            let res = libc::kill(self.ps.pid() as i32, libc::SIGCONT);
            if res != 0 {
                report_kill_panic(self.ps, libc::SIGCONT);
            }
            let mut status: c_int = 0;
            let res = waitpid(self.ps.pid() as i32, &mut status, libc::WUNTRACED);
            if res != self.ps.pid() as i32 {
                if res == -1 {
                    report_scall_panic("waitpid");
                } else {
                    panic!("unexpected pid {} returned", res);
                }
            }
        }
    }
}
