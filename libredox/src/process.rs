use string::{String, ToString};
use vec::Vec;

use syscall::{sys_clone, sys_execve, sys_spawnve, sys_exit, sys_waitpid};
use syscall::common::{CLONE_VM, CLONE_VFORK};

pub struct ExitStatus {
    status: usize
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.status == 0
    }

    pub fn code(&self) -> Option<i32> {
        Some(self.status as i32)
    }
}

pub struct Child {
    pid: isize
}

impl Child {
    pub fn id(&self) -> u32 {
        self.pid as u32
    }

    pub fn wait(&mut self) -> Option<ExitStatus> {
        let mut status: usize = 0;
        if unsafe { sys_waitpid(self.pid, &mut status, 0) } as isize >= 0 {
            Some(ExitStatus {
                status: status
            })
        } else {
            None
        }
    }
}

pub struct Command {
    pub path: String,
    pub args: Vec<String>
}

impl Command {
    pub fn new(path: &str) -> Command {
        Command {
            path: path.to_string(),
            args: Vec::new()
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Command {
        self.args.push(arg.to_string());
        self
    }

    pub fn spawn(&mut self) -> Option<Child> {
        let path_c = self.path.to_string() + "\0";

        let mut args_vec: Vec<String> = Vec::new();
        for arg in self.args.iter() {
            args_vec.push(arg.to_string() + "\0");
        }

        let mut args_c: Vec<*const u8> = Vec::new();
        for arg_vec in args_vec.iter() {
            args_c.push(arg_vec.as_ptr());
        }
        args_c.push(0 as *const u8);

        let pid = unsafe { sys_clone(CLONE_VM | CLONE_VFORK) } as isize;
        if pid == 0 {
            unsafe {
                sys_execve(path_c.as_ptr(), args_c.as_ptr());
                sys_exit(127);
            }
            None
        } else if pid > 0 {
            Some(Child {
                pid: pid
            })
        } else {
            None
        }
    }

    pub fn spawn_scheme(&mut self) -> Option<Child> {
        let path_c = self.path.to_string() + "\0";

        let mut args_vec: Vec<String> = Vec::new();
        for arg in self.args.iter() {
            args_vec.push(arg.to_string() + "\0");
        }

        let mut args_c: Vec<*const u8> = Vec::new();
        for arg_vec in args_vec.iter() {
            args_c.push(arg_vec.as_ptr());
        }
        args_c.push(0 as *const u8);

        let pid = unsafe { sys_spawnve(path_c.as_ptr(), args_c.as_ptr()) } as isize;
        if pid > 0 {
            Some(Child {
                pid: pid
            })
        } else {
            None
        }
    }
}
