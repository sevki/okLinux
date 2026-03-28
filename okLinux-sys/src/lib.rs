#![no_std]

use core::arch::asm;
pub use core::ffi::c_char;
pub use core::ffi::c_void;

macro_rules! syscall {
    ($nr:expr) => {
        unsafe { $crate::syscall0($nr as usize) }
    };
    ($nr:expr, $a0:expr) => {
        unsafe { $crate::syscall1($nr as usize, $a0 as usize) }
    };
    ($nr:expr, $a0:expr, $a1:expr) => {
        unsafe { $crate::syscall2($nr as usize, $a0 as usize, $a1 as usize) }
    };
    ($nr:expr, $a0:expr, $a1:expr, $a2:expr) => {
        unsafe { $crate::syscall3($nr as usize, $a0 as usize, $a1 as usize, $a2 as usize) }
    };
    ($nr:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { $crate::syscall4($nr as usize, $a0 as usize, $a1 as usize, $a2 as usize, $a3 as usize) }
    };
    ($nr:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        unsafe { $crate::syscall5($nr as usize, $a0 as usize, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize) }
    };
    ($nr:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe { $crate::syscall6($nr as usize, $a0 as usize, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize, $a5 as usize) }
    };
}

mod syscalls;
pub use syscalls::*;

/// Errno returned by failed syscalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Errno(pub i32);

#[inline(always)]
fn syscall_ret(raw: usize) -> Result<usize, Errno> {
    let signed = raw as isize;
    if signed >= -4095 && signed < 0 {
        Err(Errno(-signed as i32))
    } else {
        Ok(raw)
    }
}

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall1(nr: usize, a0: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall2(nr: usize, a0: usize, a1: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, in("rsi") a1, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, in("rsi") a1, in("rdx") a2, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall4(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, in("rsi") a1, in("rdx") a2, in("r10") a3, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall5(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, in("rsi") a1, in("rdx") a2, in("r10") a3, in("r8") a4, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall6(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<usize, Errno> {
    let ret: usize;
    unsafe {
        asm!("syscall", inlateout("rax") nr => ret, in("rdi") a0, in("rsi") a1, in("rdx") a2, in("r10") a3, in("r8") a4, in("r9") a5, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    syscall_ret(ret)
}
