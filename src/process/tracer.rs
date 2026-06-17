use std::{ffi::{CString, c_void}, io};

use nix::{errno::Errno, sys::{ptrace::{self, kill}, signal::Signal, wait::{WaitStatus, waitpid}}, unistd::{ForkResult, Pid, execv, fork}};

use crate::process::process::Process;


pub struct ProcessTracer {
    child: Pid
}

fn report_scall_panic(call: &str) {
    let error = io::Error::last_os_error().raw_os_error();
    panic!("{} failed. errno: {}", call, error.unwrap_or(0),);
}

impl Process for ProcessTracer {
    fn pid(&self) -> super::process::PID {
        self.child.as_raw() as u32
    }
}


impl ProcessTracer {
    /// Forks and returns ptraced child process
    pub fn new(path: &CString, args: &[CString]) -> ProcessTracer {
        // a lot of unreachables because compiler can't prove that report_scall_panic panics the
        // program anyway.
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                if let Err(_) = ptrace::traceme() {
                    report_scall_panic("traceme");
                }

                let Err(_) = execv(path, args);
                report_scall_panic("execv");
                unreachable!()
            }
            Ok(ForkResult::Parent { child }) => {
                match waitpid(child, None) {
                    Ok(WaitStatus::Stopped(child, _)) => { 
                        ProcessTracer { child }
                    }
                    _ => {
                        report_scall_panic("waitpid");
                        unreachable!();
                    }
               }
            }
            Err(_) => {
                report_scall_panic("fork");
                unreachable!();
            }
        }
    }

    pub fn set_break(&self, address: usize) -> i64 {
        let pid = self.child;
        let old = ptrace::read(pid, address as *mut c_void).expect("not correct address given");

        let breakpointed = (old & !0xFF) | 0xCC;

        ptrace::write(pid, address as *mut c_void, breakpointed).expect("breakpoint not set");

        old
    }

    pub fn clean_break(&self, address: usize, old: i64) {
        ptrace::write(self.child, address as *mut c_void, old).unwrap();
    }
    
    /// set a (software) breakpoint and continue execution until it stops at it.
    pub fn breakpoint(&self, address: usize) {
        let pid = self.child;

        let old = self.set_break(address);

        ptrace::cont(pid, None).unwrap();

        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, sig)) => assert_eq!(sig,Signal::SIGTRAP),
            _ => 
                report_scall_panic("waitpid"),
        }

        // set rip one byte earlier(before the trapp)
        let mut regs = ptrace::getregs(pid).unwrap();
        regs.rip = regs.rip - 1;
        ptrace::setregs(pid, regs).unwrap();

        self.clean_break(address, old);
    }
}

impl Drop for ProcessTracer {
    fn drop(&mut self) {
        kill(self.child).expect("couldn't kill traced child");
    }
}
