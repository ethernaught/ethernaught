use std::fs::File;
use std::io;
use std::io::Read;
use std::os::fd::{FromRawFd, RawFd};
use std::process::{Command, Stdio};

const TIOCGPTN: u32 = 0x80045430;
const TIOCSPTLCK: u32 = 0x40045431;

pub const SYS_OPEN: i64 = 2;
pub const SYS_CLOSE: i64 = 3;
pub const SYS_IOCTL: i64 = 16;
pub const O_RDWR: i32 = 2;

unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
    ret
}

fn open_pty() -> io::Result<(RawFd, RawFd)> {
    let master_fd = unsafe { syscall(SYS_OPEN, b"/dev/ptmx\0".as_ptr() as i64, O_RDWR as i64, 0, 0, 0) } as RawFd;

    let mut pty_num: i32 = 0;
    unsafe { syscall(SYS_IOCTL, master_fd as i64, TIOCGPTN as i64, &mut pty_num as *mut _ as i64, 0, 0) };
    unsafe { syscall(SYS_IOCTL, master_fd as i64, TIOCSPTLCK as i64, &0 as *const _ as i64, 0, 0) };

    let slave_name = format!("/dev/pts/{}\0", pty_num);
    let slave_fd = unsafe { syscall(SYS_OPEN, slave_name.as_ptr() as i64, O_RDWR as i64, 0, 0, 0) } as RawFd;

    Ok((master_fd, slave_fd))
}

pub fn pty() {
    let (master_fd, slave_fd) = open_pty().expect("Failed to open PTY");

    let _child = Command::new("/bin/bash")
        .stdin(unsafe { Stdio::from_raw_fd(slave_fd) })
        .stdout(unsafe { Stdio::from_raw_fd(slave_fd) })
        .stderr(unsafe { Stdio::from_raw_fd(slave_fd) })
        .spawn()
        .expect("Failed to start shell");

    unsafe { syscall(SYS_CLOSE, slave_fd as i64, 0, 0, 0, 0) };

    let mut reader = unsafe { File::from_raw_fd(master_fd) };
    let mut buf = [0; 1024];

    loop {
        match reader.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(bytes_read) => {
                let output = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                println!("{}", output);
            }
            Err(_) => break,
        }
    }
}
