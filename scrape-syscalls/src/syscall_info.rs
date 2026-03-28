/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree.
 */

use core::fmt;

/// Contains information about a syscall, including its parameters.
pub struct SyscallInfo {
    pub num: usize,
    pub name: String,
    pub params: Vec<(String, String)>,
}

impl fmt::Display for SyscallInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>3} => {}({})",
            self.num,
            self.name,
            self.params
                .iter()
                .map(|(t, a)| format!("{} {}", t, a))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl SyscallInfo {
    pub fn display_as_rust(&self) -> RustSyscall<'_> {
        RustSyscall(self)
    }

    pub fn display_as_struct(&self) -> RustSyscallStruct<'_> {
        RustSyscallStruct(self)
    }
}

pub struct RustSyscall<'a>(&'a SyscallInfo);

impl<'a> fmt::Display for RustSyscall<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params = self
            .0
            .params
            .iter()
            .map(|(t, a)| RustParam::new(t, a))
            .collect::<Vec<_>>();

        let name = translate_syscall(&self.0.name);

        writeln!(
            f,
            "/// See [{name}(2)](http://man7.org/linux/man-pages/man2/{name}.2.html)\n\
            /// for more info on this syscall.",
            name = name
        )?;
        writeln!(f, "#[inline(always)]")?;

        writeln!(
            f,
            "pub unsafe fn sys_{}({}) -> Result<usize, Errno> {{",
            name,
            params
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        let idents = params
            .iter()
            .map(|p| p.ident.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        if params.is_empty() {
            writeln!(f, "    syscall!(Sysno::{})", name)?;
        } else {
            writeln!(f, "    syscall!(Sysno::{}, {})", name, idents)?;
        }

        writeln!(f, "}}")
    }
}

pub struct RustSyscallStruct<'a>(&'a SyscallInfo);

impl<'a> fmt::Display for RustSyscallStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = translate_syscall(&self.0.name);
        // PascalCase: split on _ and capitalize each segment
        let struct_name: String = name
            .split('_')
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(c).collect(),
                }
            })
            .collect();

        let params: Vec<_> = self
            .0
            .params
            .iter()
            .map(|(t, a)| RustParam::new(t, a))
            .collect();

        writeln!(
            f,
            "/// Arguments for [{name}(2)](http://man7.org/linux/man-pages/man2/{name}.2.html).",
            name = name
        )?;
        writeln!(f, "#[derive(Debug, Clone, Copy)]")?;
        writeln!(f, "pub struct {} {{", struct_name)?;
        for p in &params {
            writeln!(f, "    pub {}: {},", p.ident, to_rust_type(p.ident, p.ty))?;
        }
        writeln!(f, "}}")?;

        // Impl with sysno() and call()
        writeln!(f, "impl {} {{", struct_name)?;
        writeln!(f, "    pub const SYSNO: Sysno = Sysno::{};", name)?;

        // call() method
        let idents: Vec<_> = params.iter().map(|p| format!("self.{}", p.ident)).collect();
        if params.is_empty() {
            writeln!(f, "    pub unsafe fn call(&self) -> Result<usize, Errno> {{")?;
            writeln!(f, "        syscall!(Sysno::{})", name)?;
        } else {
            writeln!(f, "    pub unsafe fn call(&self) -> Result<usize, Errno> {{")?;
            writeln!(f, "        syscall!(Sysno::{}, {})", name, idents.join(", "))?;
        }
        writeln!(f, "    }}")?;
        writeln!(f, "}}")
    }
}

/// Format a parameter as a Rust parameter.
struct RustParam<'a> {
    /// The type of the parameter.
    ty: &'a str,
    /// The identifier of the parameter.
    ident: &'a str,
}

impl<'a> RustParam<'a> {
    pub fn new(ty: &'a str, ident: &'a str) -> Self {
        let ident = translate_ident(ident);
        Self { ty, ident }
    }
}

impl<'a> fmt::Display for RustParam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.ident, to_rust_type(self.ident, self.ty))
    }
}

pub fn translate_syscall(name: &str) -> &str {
    let name = name.strip_prefix("sys_").unwrap_or(name);

    match name {
        "newstat" => "stat",
        "newfstat" => "fstat",
        "newlstat" => "lstat",
        "sendfile64" => "sendfile",
        "sysctl" => "_sysctl",
        "umount" => "umount2",
        "newuname" => "uname",
        _ => name,
    }
}

fn translate_ident(ident: &str) -> &str {
    match ident {
        "type" => "r#type",
        "usize" => "size",
        _ => ident,
    }
}

/// Converts this type to a Rust type if possible.
fn to_rust_type(ident: &str, ty: &str) -> &'static str {
    match ty {
        "char *" => match ident {
            "buf" => "*mut u8",
            _ => "*mut c_char",
        },
        "const char *" => match ident {
            "buf" => "*const u8",
            _ => "*const c_char",
        },
        "unsigned char *" => "*mut u8",
        "const unsigned char *" => "*const u8",
        "int" => "i32",
        "int *" => "*mut i32",
        "const int *" => "*const i32",
        "u32" => "u32",
        "u32 *" => "*mut u32",
        "__u64" => "u64",
        "__s32" => "i32",
        "long" => "i64",
        "unsigned" => "u32",
        "unsigned *" => "*mut u32",
        "unsigned int" => "u32",
        "unsigned int *" => "*mut u32",
        "size_t" => "usize",
        "size_t *" => "*mut usize",
        "unsigned long" => "u64",
        "unsigned long *" => "*mut u64",
        "const unsigned long *" => "*const u64",
        "umode_t" => "u32",
        "struct stat *" => "*mut c_void",
        "struct pollfd *" => "*mut c_void",
        "off_t" => "i64",
        "const struct sigaction *" => "*const c_void",
        "struct sigaction *" => "*mut c_void",
        "sigset_t *" => "*mut c_void",
        "const sigset_t *" => "*const c_void",
        "siginfo_t *" => "*mut c_void",
        "struct siginfo *" => "*mut c_void",
        "loff_t" => "i64",
        "loff_t *" => "*mut i64",
        "const struct iovec *" => "*const c_void",
        "fd_set *" => "*mut c_void",
        "struct __kernel_old_timeval *" => "*mut c_void",
        "key_t" => "i32",
        "struct shmid_ds *" => "*mut c_void",
        "struct __kernel_timespec *" => "*mut c_void",
        "const struct __kernel_timespec *" => "*const c_void",
        "struct __kernel_old_itimerval *" => "*mut c_void",
        "struct sockaddr *" => "*mut c_void",
        "void *" => "*mut c_void",
        "const void *" => "*const c_void",
        "const void * *" => "*mut *const c_void",
        "struct user_msghdr *" => "*mut c_void",
        "const char *const *" => "*const *const c_char",
        "pid_t" => "i32",
        "struct rusage *" => "*mut c_void",
        "struct new_utsname *" => "*mut c_void",
        "struct sembuf *" => "*mut c_void",
        "struct msgbuf *" => "*mut c_void",
        "struct msqid_ds *" => "*mut c_void",
        "struct linux_dirent *" => "*mut c_void",
        "struct linux_dirent64 *" => "*mut c_void",
        "uid_t" => "u32",
        "uid_t *" => "*mut u32",
        "gid_t" => "u32",
        "gid_t *" => "*mut u32",
        "struct timezone *" => "*mut c_void",
        "struct rlimit *" => "*mut c_void",
        "struct rlimit64 *" => "*mut c_void",
        "const struct rlimit64 *" => "*const c_void",
        "struct sysinfo *" => "*mut c_void",
        "struct tms *" => "*mut c_void",
        // FIXME: See https://man7.org/linux/man-pages/man2/capget.2.html for
        // the definition of cap_user_header_t and cap_user_data_t.
        "cap_user_header_t" => "*mut c_void",
        "cap_user_data_t" => "*mut c_void",
        "const cap_user_data_t" => "*const c_void",
        "stack_t *" => "*mut c_void",
        "const stack_t *" => "*const c_void",
        "struct utimbuf *" => "*mut c_void",
        "struct ustat *" => "*mut c_void",
        "struct statfs *" => "*mut c_void",
        "struct sched_param *" => "*mut c_void",
        // FIXME: No equivalent exists. See
        // https://man7.org/linux/man-pages/man2/sysctl.2.html for definition.
        "struct __sysctl_args *" => "*mut c_void",
        "struct __kernel_timex *" => "*mut c_void",
        "qid_t" => "i32",
        "__kernel_old_time_t *" => "*mut i64",
        // aio_context_t is defined as a simple `unsigned long`.
        "aio_context_t *" => "*mut u64",
        "aio_context_t" => "u64",
        // FIXME: io_event is defined at
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/aio_abi.h#L60.
        "struct io_event *" => "*mut c_void",
        // FIXME: See https://man7.org/linux/man-pages/man2/io_submit.2.html for
        // definition of iocb.
        "struct iocb * *" => "*mut *mut c_void",
        "struct iocb *" => "*mut c_void",
        "clockid_t" => "i32",
        "const clockid_t" => "i32",
        "struct sigevent *" => "*mut c_void",
        "const struct sigevent *" => "*const c_void",
        "timer_t *" => "*mut i32",
        "timer_t" => "i32",
        "const struct __kernel_itimerspec *" => "*const c_void",
        "struct __kernel_itimerspec *" => "*mut c_void",
        "struct epoll_event *" => "*mut c_void",
        "struct mq_attr *" => "*mut c_void",
        "const struct mq_attr *" => "*const c_void",
        "mqd_t" => "i32",
        // FIXME: See https://man7.org/linux/man-pages/man2/kexec_load.2.html
        // for definition of kexec_segment.
        "struct kexec_segment *" => "*mut c_void",
        "key_serial_t" => "i32",
        // FIXME: robust_list_head is defined at
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/futex.h#L97
        "struct robust_list_head *" => "*mut c_void",
        "struct robust_list_head * *" => "*mut *mut c_void",
        // FIXME: perf_event_attr is a big struct and no definition in libc
        // exists. For real definiton, see
        // https://man7.org/linux/man-pages/man2/perf_event_open.2.html.
        "struct perf_event_attr *" => "*mut c_void",
        "struct mmsghdr *" => "*mut c_void",
        // FIXME: See
        // https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html for
        // definition of file_handle.
        "struct file_handle *" => "*mut c_void",
        // NOTE: getcpu_cache is an opaque type and should never be accessed by
        // user code.
        "struct getcpu_cache *" => "*mut c_void",
        // FIXME: For definition of sched_attr, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/sched/types.h#L102
        "struct sched_attr *" => "*mut c_void",
        // FIXME: For definition of bpf_attr, see:
        // https://man7.org/linux/man-pages/man2/bpf.2.html
        "union bpf_attr *" => "*mut c_void",
        "rwf_t" => "i32",
        "struct statx *" => "*mut c_void",
        // mount_attr is used by mount_setattr(2). See:
        // https://elixir.bootlin.com/linux/v6.6/source/include/uapi/linux/mount.h#L124
        "struct mount_attr *" => "*mut c_void",
        // FIXME: For definition of __aio_sigset, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/fs/aio.c#L2216
        "const struct __aio_sigset *" => "*mut c_void",
        // FIXME: For definition of rseq, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/rseq.h#L62
        "struct rseq *" => "*mut c_void",
        // FIXME: For definitino of io_uring_params, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/io_uring.h#L265
        "struct io_uring_params *" => "*mut c_void",
        // FIXME: This is used by the clone3 syscall and libc doesn't have this
        // yet. For the definition of clone_args, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/sched.h#L92
        "struct clone_args *" => "*mut c_void",
        // FIXME: This is used by the openat2 syscall and libc doesn't have this
        // yet. For the definition of open_how, see:
        // https://elixir.bootlin.com/linux/v5.16.11/source/include/uapi/linux/openat2.h#L19
        "struct open_how *" => "*mut c_void",
        // Landlock LSM types (kernel 5.13+)
        "const struct landlock_ruleset_attr *const" => "*const c_void",
        "const struct landlock_ruleset_attr *" => "*const c_void",
        "const void *const" => "*const c_void",
        // cachestat (kernel 6.5+)
        "struct cachestat_range *" => "*mut c_void",
        "struct cachestat *" => "*mut c_void",
        // const-qualified value types (kernel 5.13+ landlock, etc.)
        "const size_t" => "usize",
        "const unsigned int" => "u32",
        "const __u32" => "u32",
        "const int" => "i32",
        "const unsigned long" => "u64",
        "const long" => "i64",
        // Landlock LSM (kernel 5.13+)
        "const enum landlock_rule_type" => "u32",
        // futex_waitv (kernel 5.16+)
        "struct futex_waitv *" => "*mut c_void",
        _ => panic!(
            "Don't know how to convert this syscall parameter to Rust: {} {}",
            ident, ty
        ),
    }
}
