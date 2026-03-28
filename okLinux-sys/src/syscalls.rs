use super::*;

/// Syscall numbers scraped from vmlinux.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum Sysno {
    read = 0,
    write = 1,
    open = 2,
    close = 3,
    stat = 4,
    fstat = 5,
    lstat = 6,
    poll = 7,
    lseek = 8,
    mmap = 9,
    mprotect = 10,
    munmap = 11,
    brk = 12,
    rt_sigaction = 13,
    rt_sigprocmask = 14,
    rt_sigreturn = 15,
    ioctl = 16,
    pread64 = 17,
    pwrite64 = 18,
    readv = 19,
    writev = 20,
    access = 21,
    pipe = 22,
    select = 23,
    sched_yield = 24,
    mremap = 25,
    msync = 26,
    mincore = 27,
    madvise = 28,
    shmget = 29,
    shmat = 30,
    shmctl = 31,
    dup = 32,
    dup2 = 33,
    pause = 34,
    nanosleep = 35,
    getitimer = 36,
    alarm = 37,
    setitimer = 38,
    getpid = 39,
    sendfile = 40,
    socket = 41,
    connect = 42,
    accept = 43,
    sendto = 44,
    recvfrom = 45,
    sendmsg = 46,
    recvmsg = 47,
    shutdown = 48,
    bind = 49,
    listen = 50,
    getsockname = 51,
    getpeername = 52,
    socketpair = 53,
    setsockopt = 54,
    getsockopt = 55,
    clone = 56,
    fork = 57,
    vfork = 58,
    execve = 59,
    exit = 60,
    wait4 = 61,
    kill = 62,
    uname = 63,
    semget = 64,
    semop = 65,
    semctl = 66,
    shmdt = 67,
    msgget = 68,
    msgsnd = 69,
    msgrcv = 70,
    msgctl = 71,
    fcntl = 72,
    flock = 73,
    fsync = 74,
    fdatasync = 75,
    truncate = 76,
    ftruncate = 77,
    getdents = 78,
    getcwd = 79,
    chdir = 80,
    fchdir = 81,
    rename = 82,
    mkdir = 83,
    rmdir = 84,
    creat = 85,
    link = 86,
    unlink = 87,
    symlink = 88,
    readlink = 89,
    chmod = 90,
    fchmod = 91,
    chown = 92,
    fchown = 93,
    lchown = 94,
    umask = 95,
    gettimeofday = 96,
    getrlimit = 97,
    getrusage = 98,
    sysinfo = 99,
    times = 100,
    ptrace = 101,
    getuid = 102,
    syslog = 103,
    getgid = 104,
    setuid = 105,
    setgid = 106,
    geteuid = 107,
    getegid = 108,
    setpgid = 109,
    getppid = 110,
    getpgrp = 111,
    setsid = 112,
    setreuid = 113,
    setregid = 114,
    getgroups = 115,
    setgroups = 116,
    setresuid = 117,
    getresuid = 118,
    setresgid = 119,
    getresgid = 120,
    getpgid = 121,
    setfsuid = 122,
    setfsgid = 123,
    getsid = 124,
    capget = 125,
    capset = 126,
    rt_sigpending = 127,
    rt_sigtimedwait = 128,
    rt_sigqueueinfo = 129,
    rt_sigsuspend = 130,
    sigaltstack = 131,
    utime = 132,
    mknod = 133,
    personality = 135,
    ustat = 136,
    statfs = 137,
    fstatfs = 138,
    sysfs = 139,
    getpriority = 140,
    setpriority = 141,
    sched_setparam = 142,
    sched_getparam = 143,
    sched_setscheduler = 144,
    sched_getscheduler = 145,
    sched_get_priority_max = 146,
    sched_get_priority_min = 147,
    sched_rr_get_interval = 148,
    mlock = 149,
    munlock = 150,
    mlockall = 151,
    munlockall = 152,
    vhangup = 153,
    modify_ldt = 154,
    pivot_root = 155,
    prctl = 157,
    arch_prctl = 158,
    adjtimex = 159,
    setrlimit = 160,
    chroot = 161,
    sync = 162,
    settimeofday = 164,
    mount = 165,
    umount2 = 166,
    swapon = 167,
    swapoff = 168,
    reboot = 169,
    sethostname = 170,
    setdomainname = 171,
    iopl = 172,
    ioperm = 173,
    gettid = 186,
    readahead = 187,
    setxattr = 188,
    lsetxattr = 189,
    fsetxattr = 190,
    getxattr = 191,
    lgetxattr = 192,
    fgetxattr = 193,
    listxattr = 194,
    llistxattr = 195,
    flistxattr = 196,
    removexattr = 197,
    lremovexattr = 198,
    fremovexattr = 199,
    tkill = 200,
    time = 201,
    futex = 202,
    sched_setaffinity = 203,
    sched_getaffinity = 204,
    io_setup = 206,
    io_destroy = 207,
    io_getevents = 208,
    io_submit = 209,
    io_cancel = 210,
    epoll_create = 213,
    remap_file_pages = 216,
    getdents64 = 217,
    set_tid_address = 218,
    restart_syscall = 219,
    semtimedop = 220,
    fadvise64 = 221,
    timer_create = 222,
    timer_settime = 223,
    timer_gettime = 224,
    timer_getoverrun = 225,
    timer_delete = 226,
    clock_settime = 227,
    clock_gettime = 228,
    clock_getres = 229,
    clock_nanosleep = 230,
    exit_group = 231,
    epoll_wait = 232,
    epoll_ctl = 233,
    tgkill = 234,
    utimes = 235,
    mq_open = 240,
    mq_unlink = 241,
    mq_timedsend = 242,
    mq_timedreceive = 243,
    mq_notify = 244,
    mq_getsetattr = 245,
    waitid = 247,
    add_key = 248,
    request_key = 249,
    keyctl = 250,
    ioprio_set = 251,
    ioprio_get = 252,
    inotify_init = 253,
    inotify_add_watch = 254,
    inotify_rm_watch = 255,
    openat = 257,
    mkdirat = 258,
    mknodat = 259,
    fchownat = 260,
    futimesat = 261,
    newfstatat = 262,
    unlinkat = 263,
    renameat = 264,
    linkat = 265,
    symlinkat = 266,
    readlinkat = 267,
    fchmodat = 268,
    faccessat = 269,
    pselect6 = 270,
    ppoll = 271,
    unshare = 272,
    set_robust_list = 273,
    get_robust_list = 274,
    splice = 275,
    tee = 276,
    sync_file_range = 277,
    vmsplice = 278,
    utimensat = 280,
    epoll_pwait = 281,
    signalfd = 282,
    timerfd_create = 283,
    eventfd = 284,
    fallocate = 285,
    timerfd_settime = 286,
    timerfd_gettime = 287,
    accept4 = 288,
    signalfd4 = 289,
    eventfd2 = 290,
    epoll_create1 = 291,
    dup3 = 292,
    pipe2 = 293,
    inotify_init1 = 294,
    preadv = 295,
    pwritev = 296,
    rt_tgsigqueueinfo = 297,
    perf_event_open = 298,
    recvmmsg = 299,
    fanotify_init = 300,
    fanotify_mark = 301,
    prlimit64 = 302,
    name_to_handle_at = 303,
    open_by_handle_at = 304,
    clock_adjtime = 305,
    syncfs = 306,
    sendmmsg = 307,
    setns = 308,
    getcpu = 309,
    process_vm_readv = 310,
    process_vm_writev = 311,
    sched_setattr = 314,
    sched_getattr = 315,
    renameat2 = 316,
    seccomp = 317,
    getrandom = 318,
    memfd_create = 319,
    bpf = 321,
    execveat = 322,
    membarrier = 324,
    mlock2 = 325,
    copy_file_range = 326,
    preadv2 = 327,
    pwritev2 = 328,
    pkey_mprotect = 329,
    pkey_alloc = 330,
    pkey_free = 331,
    statx = 332,
    io_pgetevents = 333,
    rseq = 334,
    pidfd_send_signal = 424,
    io_uring_setup = 425,
    io_uring_enter = 426,
    io_uring_register = 427,
    open_tree = 428,
    move_mount = 429,
    fsopen = 430,
    fsconfig = 431,
    fsmount = 432,
    fspick = 433,
    pidfd_open = 434,
    clone3 = 435,
    close_range = 436,
    openat2 = 437,
    pidfd_getfd = 438,
    faccessat2 = 439,
    process_madvise = 440,
    epoll_pwait2 = 441,
    mount_setattr = 442,
    landlock_create_ruleset = 444,
    landlock_add_rule = 445,
    landlock_restrict_self = 446,
    memfd_secret = 447,
    process_mrelease = 448,
    futex_waitv = 449,
    cachestat = 451,
    fchmodat2 = 452,
    mseal = 462,
}

/// See [read(2)](http://man7.org/linux/man-pages/man2/read.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_read(fd: u32, buf: *mut u8, count: usize) -> Result<usize, Errno> {
    syscall!(Sysno::read, fd, buf, count)
}

/// See [write(2)](http://man7.org/linux/man-pages/man2/write.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_write(fd: u32, buf: *const u8, count: usize) -> Result<usize, Errno> {
    syscall!(Sysno::write, fd, buf, count)
}

/// See [open(2)](http://man7.org/linux/man-pages/man2/open.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_open(filename: *const c_char, flags: i32, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::open, filename, flags, mode)
}

/// See [close(2)](http://man7.org/linux/man-pages/man2/close.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_close(fd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::close, fd)
}

/// See [stat(2)](http://man7.org/linux/man-pages/man2/stat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_stat(filename: *const c_char, statbuf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::stat, filename, statbuf)
}

/// See [fstat(2)](http://man7.org/linux/man-pages/man2/fstat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fstat(fd: u32, statbuf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::fstat, fd, statbuf)
}

/// See [lstat(2)](http://man7.org/linux/man-pages/man2/lstat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lstat(filename: *const c_char, statbuf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::lstat, filename, statbuf)
}

/// See [poll(2)](http://man7.org/linux/man-pages/man2/poll.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_poll(ufds: *mut c_void, nfds: u32, timeout_msecs: i32) -> Result<usize, Errno> {
    syscall!(Sysno::poll, ufds, nfds, timeout_msecs)
}

/// See [lseek(2)](http://man7.org/linux/man-pages/man2/lseek.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lseek(fd: u32, offset: i64, whence: u32) -> Result<usize, Errno> {
    syscall!(Sysno::lseek, fd, offset, whence)
}

/// See [mmap(2)](http://man7.org/linux/man-pages/man2/mmap.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mmap(
    addr: u64,
    len: u64,
    prot: u64,
    flags: u64,
    fd: u64,
    off: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::mmap, addr, len, prot, flags, fd, off)
}

/// See [mprotect(2)](http://man7.org/linux/man-pages/man2/mprotect.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mprotect(start: u64, len: usize, prot: u64) -> Result<usize, Errno> {
    syscall!(Sysno::mprotect, start, len, prot)
}

/// See [munmap(2)](http://man7.org/linux/man-pages/man2/munmap.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_munmap(addr: u64, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::munmap, addr, len)
}

/// See [brk(2)](http://man7.org/linux/man-pages/man2/brk.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_brk(brk: u64) -> Result<usize, Errno> {
    syscall!(Sysno::brk, brk)
}

/// See [rt_sigaction(2)](http://man7.org/linux/man-pages/man2/rt_sigaction.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigaction(
    sig: i32,
    act: *const c_void,
    oact: *mut c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigaction, sig, act, oact, sigsetsize)
}

/// See [rt_sigprocmask(2)](http://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigprocmask(
    how: i32,
    nset: *mut c_void,
    oset: *mut c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigprocmask, how, nset, oset, sigsetsize)
}

/// See [rt_sigreturn(2)](http://man7.org/linux/man-pages/man2/rt_sigreturn.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigreturn() -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigreturn)
}

/// See [ioctl(2)](http://man7.org/linux/man-pages/man2/ioctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ioctl(fd: u32, cmd: u32, arg: u64) -> Result<usize, Errno> {
    syscall!(Sysno::ioctl, fd, cmd, arg)
}

/// See [pread64(2)](http://man7.org/linux/man-pages/man2/pread64.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pread64(fd: u32, buf: *mut u8, count: usize, pos: i64) -> Result<usize, Errno> {
    syscall!(Sysno::pread64, fd, buf, count, pos)
}

/// See [pwrite64(2)](http://man7.org/linux/man-pages/man2/pwrite64.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pwrite64(
    fd: u32,
    buf: *const u8,
    count: usize,
    pos: i64,
) -> Result<usize, Errno> {
    syscall!(Sysno::pwrite64, fd, buf, count, pos)
}

/// See [readv(2)](http://man7.org/linux/man-pages/man2/readv.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_readv(fd: u64, vec: *const c_void, vlen: u64) -> Result<usize, Errno> {
    syscall!(Sysno::readv, fd, vec, vlen)
}

/// See [writev(2)](http://man7.org/linux/man-pages/man2/writev.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_writev(fd: u64, vec: *const c_void, vlen: u64) -> Result<usize, Errno> {
    syscall!(Sysno::writev, fd, vec, vlen)
}

/// See [access(2)](http://man7.org/linux/man-pages/man2/access.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_access(filename: *const c_char, mode: i32) -> Result<usize, Errno> {
    syscall!(Sysno::access, filename, mode)
}

/// See [pipe(2)](http://man7.org/linux/man-pages/man2/pipe.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pipe(fildes: *mut i32) -> Result<usize, Errno> {
    syscall!(Sysno::pipe, fildes)
}

/// See [select(2)](http://man7.org/linux/man-pages/man2/select.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_select(
    n: i32,
    inp: *mut c_void,
    outp: *mut c_void,
    exp: *mut c_void,
    tvp: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::select, n, inp, outp, exp, tvp)
}

/// See [sched_yield(2)](http://man7.org/linux/man-pages/man2/sched_yield.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_yield() -> Result<usize, Errno> {
    syscall!(Sysno::sched_yield)
}

/// See [mremap(2)](http://man7.org/linux/man-pages/man2/mremap.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mremap(
    addr: u64,
    old_len: u64,
    new_len: u64,
    flags: u64,
    new_addr: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::mremap, addr, old_len, new_len, flags, new_addr)
}

/// See [msync(2)](http://man7.org/linux/man-pages/man2/msync.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_msync(start: u64, len: usize, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::msync, start, len, flags)
}

/// See [mincore(2)](http://man7.org/linux/man-pages/man2/mincore.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mincore(start: u64, len: usize, vec: *mut u8) -> Result<usize, Errno> {
    syscall!(Sysno::mincore, start, len, vec)
}

/// See [madvise(2)](http://man7.org/linux/man-pages/man2/madvise.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_madvise(start: u64, len_in: usize, behavior: i32) -> Result<usize, Errno> {
    syscall!(Sysno::madvise, start, len_in, behavior)
}

/// See [shmget(2)](http://man7.org/linux/man-pages/man2/shmget.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_shmget(key: i32, size: usize, shmflg: i32) -> Result<usize, Errno> {
    syscall!(Sysno::shmget, key, size, shmflg)
}

/// See [shmat(2)](http://man7.org/linux/man-pages/man2/shmat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_shmat(shmid: i32, shmaddr: *mut c_char, shmflg: i32) -> Result<usize, Errno> {
    syscall!(Sysno::shmat, shmid, shmaddr, shmflg)
}

/// See [shmctl(2)](http://man7.org/linux/man-pages/man2/shmctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_shmctl(shmid: i32, cmd: i32, buf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::shmctl, shmid, cmd, buf)
}

/// See [dup(2)](http://man7.org/linux/man-pages/man2/dup.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_dup(fildes: u32) -> Result<usize, Errno> {
    syscall!(Sysno::dup, fildes)
}

/// See [dup2(2)](http://man7.org/linux/man-pages/man2/dup2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_dup2(oldfd: u32, newfd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::dup2, oldfd, newfd)
}

/// See [pause(2)](http://man7.org/linux/man-pages/man2/pause.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pause() -> Result<usize, Errno> {
    syscall!(Sysno::pause)
}

/// See [nanosleep(2)](http://man7.org/linux/man-pages/man2/nanosleep.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_nanosleep(rqtp: *mut c_void, rmtp: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::nanosleep, rqtp, rmtp)
}

/// See [getitimer(2)](http://man7.org/linux/man-pages/man2/getitimer.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getitimer(which: i32, value: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::getitimer, which, value)
}

/// See [alarm(2)](http://man7.org/linux/man-pages/man2/alarm.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_alarm(seconds: u32) -> Result<usize, Errno> {
    syscall!(Sysno::alarm, seconds)
}

/// See [setitimer(2)](http://man7.org/linux/man-pages/man2/setitimer.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setitimer(
    which: i32,
    value: *mut c_void,
    ovalue: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::setitimer, which, value, ovalue)
}

/// See [getpid(2)](http://man7.org/linux/man-pages/man2/getpid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getpid() -> Result<usize, Errno> {
    syscall!(Sysno::getpid)
}

/// See [sendfile(2)](http://man7.org/linux/man-pages/man2/sendfile.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sendfile(
    out_fd: i32,
    in_fd: i32,
    offset: *mut i64,
    count: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::sendfile, out_fd, in_fd, offset, count)
}

/// See [socket(2)](http://man7.org/linux/man-pages/man2/socket.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_socket(family: i32, r#type: i32, protocol: i32) -> Result<usize, Errno> {
    syscall!(Sysno::socket, family, r#type, protocol)
}

/// See [connect(2)](http://man7.org/linux/man-pages/man2/connect.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_connect(fd: i32, uservaddr: *mut c_void, addrlen: i32) -> Result<usize, Errno> {
    syscall!(Sysno::connect, fd, uservaddr, addrlen)
}

/// See [accept(2)](http://man7.org/linux/man-pages/man2/accept.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_accept(
    fd: i32,
    upeer_sockaddr: *mut c_void,
    upeer_addrlen: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::accept, fd, upeer_sockaddr, upeer_addrlen)
}

/// See [sendto(2)](http://man7.org/linux/man-pages/man2/sendto.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sendto(
    fd: i32,
    buff: *mut c_void,
    len: usize,
    flags: u32,
    addr: *mut c_void,
    addr_len: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::sendto, fd, buff, len, flags, addr, addr_len)
}

/// See [recvfrom(2)](http://man7.org/linux/man-pages/man2/recvfrom.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_recvfrom(
    fd: i32,
    ubuf: *mut c_void,
    size: usize,
    flags: u32,
    addr: *mut c_void,
    addr_len: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::recvfrom, fd, ubuf, size, flags, addr, addr_len)
}

/// See [sendmsg(2)](http://man7.org/linux/man-pages/man2/sendmsg.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sendmsg(fd: i32, msg: *mut c_void, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::sendmsg, fd, msg, flags)
}

/// See [recvmsg(2)](http://man7.org/linux/man-pages/man2/recvmsg.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_recvmsg(fd: i32, msg: *mut c_void, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::recvmsg, fd, msg, flags)
}

/// See [shutdown(2)](http://man7.org/linux/man-pages/man2/shutdown.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_shutdown(fd: i32, how: i32) -> Result<usize, Errno> {
    syscall!(Sysno::shutdown, fd, how)
}

/// See [bind(2)](http://man7.org/linux/man-pages/man2/bind.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_bind(fd: i32, umyaddr: *mut c_void, addrlen: i32) -> Result<usize, Errno> {
    syscall!(Sysno::bind, fd, umyaddr, addrlen)
}

/// See [listen(2)](http://man7.org/linux/man-pages/man2/listen.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_listen(fd: i32, backlog: i32) -> Result<usize, Errno> {
    syscall!(Sysno::listen, fd, backlog)
}

/// See [getsockname(2)](http://man7.org/linux/man-pages/man2/getsockname.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getsockname(
    fd: i32,
    usockaddr: *mut c_void,
    usockaddr_len: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::getsockname, fd, usockaddr, usockaddr_len)
}

/// See [getpeername(2)](http://man7.org/linux/man-pages/man2/getpeername.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getpeername(
    fd: i32,
    usockaddr: *mut c_void,
    usockaddr_len: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::getpeername, fd, usockaddr, usockaddr_len)
}

/// See [socketpair(2)](http://man7.org/linux/man-pages/man2/socketpair.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_socketpair(
    family: i32,
    r#type: i32,
    protocol: i32,
    usockvec: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::socketpair, family, r#type, protocol, usockvec)
}

/// See [setsockopt(2)](http://man7.org/linux/man-pages/man2/setsockopt.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setsockopt(
    fd: i32,
    level: i32,
    optname: i32,
    optval: *mut c_char,
    optlen: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::setsockopt, fd, level, optname, optval, optlen)
}

/// See [getsockopt(2)](http://man7.org/linux/man-pages/man2/getsockopt.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getsockopt(
    fd: i32,
    level: i32,
    optname: i32,
    optval: *mut c_char,
    optlen: *mut i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::getsockopt, fd, level, optname, optval, optlen)
}

/// See [clone(2)](http://man7.org/linux/man-pages/man2/clone.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clone(
    clone_flags: u64,
    newsp: u64,
    parent_tidptr: *mut i32,
    child_tidptr: *mut i32,
    tls: u64,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::clone,
        clone_flags,
        newsp,
        parent_tidptr,
        child_tidptr,
        tls
    )
}

/// See [fork(2)](http://man7.org/linux/man-pages/man2/fork.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fork() -> Result<usize, Errno> {
    syscall!(Sysno::fork)
}

/// See [vfork(2)](http://man7.org/linux/man-pages/man2/vfork.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_vfork() -> Result<usize, Errno> {
    syscall!(Sysno::vfork)
}

/// See [execve(2)](http://man7.org/linux/man-pages/man2/execve.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_execve(
    filename: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::execve, filename, argv, envp)
}

/// See [exit(2)](http://man7.org/linux/man-pages/man2/exit.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_exit(error_code: i32) -> Result<usize, Errno> {
    syscall!(Sysno::exit, error_code)
}

/// See [wait4(2)](http://man7.org/linux/man-pages/man2/wait4.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_wait4(
    upid: i32,
    stat_addr: *mut i32,
    options: i32,
    ru: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::wait4, upid, stat_addr, options, ru)
}

/// See [kill(2)](http://man7.org/linux/man-pages/man2/kill.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_kill(pid: i32, sig: i32) -> Result<usize, Errno> {
    syscall!(Sysno::kill, pid, sig)
}

/// See [uname(2)](http://man7.org/linux/man-pages/man2/uname.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_uname(name: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::uname, name)
}

/// See [semget(2)](http://man7.org/linux/man-pages/man2/semget.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_semget(key: i32, nsems: i32, semflg: i32) -> Result<usize, Errno> {
    syscall!(Sysno::semget, key, nsems, semflg)
}

/// See [semop(2)](http://man7.org/linux/man-pages/man2/semop.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_semop(semid: i32, tsops: *mut c_void, nsops: u32) -> Result<usize, Errno> {
    syscall!(Sysno::semop, semid, tsops, nsops)
}

/// See [semctl(2)](http://man7.org/linux/man-pages/man2/semctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_semctl(semid: i32, semnum: i32, cmd: i32, arg: u64) -> Result<usize, Errno> {
    syscall!(Sysno::semctl, semid, semnum, cmd, arg)
}

/// See [shmdt(2)](http://man7.org/linux/man-pages/man2/shmdt.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_shmdt(shmaddr: *mut c_char) -> Result<usize, Errno> {
    syscall!(Sysno::shmdt, shmaddr)
}

/// See [msgget(2)](http://man7.org/linux/man-pages/man2/msgget.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_msgget(key: i32, msgflg: i32) -> Result<usize, Errno> {
    syscall!(Sysno::msgget, key, msgflg)
}

/// See [msgsnd(2)](http://man7.org/linux/man-pages/man2/msgsnd.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_msgsnd(
    msqid: i32,
    msgp: *mut c_void,
    msgsz: usize,
    msgflg: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::msgsnd, msqid, msgp, msgsz, msgflg)
}

/// See [msgrcv(2)](http://man7.org/linux/man-pages/man2/msgrcv.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_msgrcv(
    msqid: i32,
    msgp: *mut c_void,
    msgsz: usize,
    msgtyp: i64,
    msgflg: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::msgrcv, msqid, msgp, msgsz, msgtyp, msgflg)
}

/// See [msgctl(2)](http://man7.org/linux/man-pages/man2/msgctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_msgctl(msqid: i32, cmd: i32, buf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::msgctl, msqid, cmd, buf)
}

/// See [fcntl(2)](http://man7.org/linux/man-pages/man2/fcntl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fcntl(fd: u32, cmd: u32, arg: u64) -> Result<usize, Errno> {
    syscall!(Sysno::fcntl, fd, cmd, arg)
}

/// See [flock(2)](http://man7.org/linux/man-pages/man2/flock.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_flock(fd: u32, cmd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::flock, fd, cmd)
}

/// See [fsync(2)](http://man7.org/linux/man-pages/man2/fsync.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fsync(fd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fsync, fd)
}

/// See [fdatasync(2)](http://man7.org/linux/man-pages/man2/fdatasync.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fdatasync(fd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fdatasync, fd)
}

/// See [truncate(2)](http://man7.org/linux/man-pages/man2/truncate.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_truncate(path: *const c_char, length: i64) -> Result<usize, Errno> {
    syscall!(Sysno::truncate, path, length)
}

/// See [ftruncate(2)](http://man7.org/linux/man-pages/man2/ftruncate.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ftruncate(fd: u32, length: i64) -> Result<usize, Errno> {
    syscall!(Sysno::ftruncate, fd, length)
}

/// See [getdents(2)](http://man7.org/linux/man-pages/man2/getdents.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getdents(fd: u32, dirent: *mut c_void, count: u32) -> Result<usize, Errno> {
    syscall!(Sysno::getdents, fd, dirent, count)
}

/// See [getcwd(2)](http://man7.org/linux/man-pages/man2/getcwd.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getcwd(buf: *mut u8, size: u64) -> Result<usize, Errno> {
    syscall!(Sysno::getcwd, buf, size)
}

/// See [chdir(2)](http://man7.org/linux/man-pages/man2/chdir.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_chdir(filename: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::chdir, filename)
}

/// See [fchdir(2)](http://man7.org/linux/man-pages/man2/fchdir.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchdir(fd: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fchdir, fd)
}

/// See [rename(2)](http://man7.org/linux/man-pages/man2/rename.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rename(oldname: *const c_char, newname: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::rename, oldname, newname)
}

/// See [mkdir(2)](http://man7.org/linux/man-pages/man2/mkdir.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mkdir(pathname: *const c_char, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::mkdir, pathname, mode)
}

/// See [rmdir(2)](http://man7.org/linux/man-pages/man2/rmdir.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rmdir(pathname: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::rmdir, pathname)
}

/// See [creat(2)](http://man7.org/linux/man-pages/man2/creat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_creat(pathname: *const c_char, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::creat, pathname, mode)
}

/// See [link(2)](http://man7.org/linux/man-pages/man2/link.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_link(oldname: *const c_char, newname: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::link, oldname, newname)
}

/// See [unlink(2)](http://man7.org/linux/man-pages/man2/unlink.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_unlink(pathname: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::unlink, pathname)
}

/// See [symlink(2)](http://man7.org/linux/man-pages/man2/symlink.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_symlink(oldname: *const c_char, newname: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::symlink, oldname, newname)
}

/// See [readlink(2)](http://man7.org/linux/man-pages/man2/readlink.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_readlink(path: *const c_char, buf: *mut u8, bufsiz: i32) -> Result<usize, Errno> {
    syscall!(Sysno::readlink, path, buf, bufsiz)
}

/// See [chmod(2)](http://man7.org/linux/man-pages/man2/chmod.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_chmod(filename: *const c_char, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::chmod, filename, mode)
}

/// See [fchmod(2)](http://man7.org/linux/man-pages/man2/fchmod.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchmod(fd: u32, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fchmod, fd, mode)
}

/// See [chown(2)](http://man7.org/linux/man-pages/man2/chown.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_chown(filename: *const c_char, user: u32, group: u32) -> Result<usize, Errno> {
    syscall!(Sysno::chown, filename, user, group)
}

/// See [fchown(2)](http://man7.org/linux/man-pages/man2/fchown.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchown(fd: u32, user: u32, group: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fchown, fd, user, group)
}

/// See [lchown(2)](http://man7.org/linux/man-pages/man2/lchown.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lchown(filename: *const c_char, user: u32, group: u32) -> Result<usize, Errno> {
    syscall!(Sysno::lchown, filename, user, group)
}

/// See [umask(2)](http://man7.org/linux/man-pages/man2/umask.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_umask(mask: i32) -> Result<usize, Errno> {
    syscall!(Sysno::umask, mask)
}

/// See [gettimeofday(2)](http://man7.org/linux/man-pages/man2/gettimeofday.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_gettimeofday(tv: *mut c_void, tz: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::gettimeofday, tv, tz)
}

/// See [getrlimit(2)](http://man7.org/linux/man-pages/man2/getrlimit.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getrlimit(resource: u32, rlim: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::getrlimit, resource, rlim)
}

/// See [getrusage(2)](http://man7.org/linux/man-pages/man2/getrusage.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getrusage(who: i32, ru: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::getrusage, who, ru)
}

/// See [sysinfo(2)](http://man7.org/linux/man-pages/man2/sysinfo.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sysinfo(info: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::sysinfo, info)
}

/// See [times(2)](http://man7.org/linux/man-pages/man2/times.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_times(tbuf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::times, tbuf)
}

/// See [ptrace(2)](http://man7.org/linux/man-pages/man2/ptrace.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ptrace(request: i64, pid: i64, addr: u64, data: u64) -> Result<usize, Errno> {
    syscall!(Sysno::ptrace, request, pid, addr, data)
}

/// See [getuid(2)](http://man7.org/linux/man-pages/man2/getuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getuid() -> Result<usize, Errno> {
    syscall!(Sysno::getuid)
}

/// See [syslog(2)](http://man7.org/linux/man-pages/man2/syslog.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_syslog(r#type: i32, buf: *mut u8, len: i32) -> Result<usize, Errno> {
    syscall!(Sysno::syslog, r#type, buf, len)
}

/// See [getgid(2)](http://man7.org/linux/man-pages/man2/getgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getgid() -> Result<usize, Errno> {
    syscall!(Sysno::getgid)
}

/// See [setuid(2)](http://man7.org/linux/man-pages/man2/setuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setuid(uid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setuid, uid)
}

/// See [setgid(2)](http://man7.org/linux/man-pages/man2/setgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setgid(gid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setgid, gid)
}

/// See [geteuid(2)](http://man7.org/linux/man-pages/man2/geteuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_geteuid() -> Result<usize, Errno> {
    syscall!(Sysno::geteuid)
}

/// See [getegid(2)](http://man7.org/linux/man-pages/man2/getegid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getegid() -> Result<usize, Errno> {
    syscall!(Sysno::getegid)
}

/// See [setpgid(2)](http://man7.org/linux/man-pages/man2/setpgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setpgid(pid: i32, pgid: i32) -> Result<usize, Errno> {
    syscall!(Sysno::setpgid, pid, pgid)
}

/// See [getppid(2)](http://man7.org/linux/man-pages/man2/getppid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getppid() -> Result<usize, Errno> {
    syscall!(Sysno::getppid)
}

/// See [getpgrp(2)](http://man7.org/linux/man-pages/man2/getpgrp.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getpgrp() -> Result<usize, Errno> {
    syscall!(Sysno::getpgrp)
}

/// See [setsid(2)](http://man7.org/linux/man-pages/man2/setsid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setsid() -> Result<usize, Errno> {
    syscall!(Sysno::setsid)
}

/// See [setreuid(2)](http://man7.org/linux/man-pages/man2/setreuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setreuid(ruid: u32, euid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setreuid, ruid, euid)
}

/// See [setregid(2)](http://man7.org/linux/man-pages/man2/setregid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setregid(rgid: u32, egid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setregid, rgid, egid)
}

/// See [getgroups(2)](http://man7.org/linux/man-pages/man2/getgroups.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getgroups(gidsetsize: i32, grouplist: *mut u32) -> Result<usize, Errno> {
    syscall!(Sysno::getgroups, gidsetsize, grouplist)
}

/// See [setgroups(2)](http://man7.org/linux/man-pages/man2/setgroups.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setgroups(gidsetsize: i32, grouplist: *mut u32) -> Result<usize, Errno> {
    syscall!(Sysno::setgroups, gidsetsize, grouplist)
}

/// See [setresuid(2)](http://man7.org/linux/man-pages/man2/setresuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setresuid(ruid: u32, euid: u32, suid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setresuid, ruid, euid, suid)
}

/// See [getresuid(2)](http://man7.org/linux/man-pages/man2/getresuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getresuid(
    ruidp: *mut u32,
    euidp: *mut u32,
    suidp: *mut u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::getresuid, ruidp, euidp, suidp)
}

/// See [setresgid(2)](http://man7.org/linux/man-pages/man2/setresgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setresgid(rgid: u32, egid: u32, sgid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setresgid, rgid, egid, sgid)
}

/// See [getresgid(2)](http://man7.org/linux/man-pages/man2/getresgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getresgid(
    rgidp: *mut u32,
    egidp: *mut u32,
    sgidp: *mut u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::getresgid, rgidp, egidp, sgidp)
}

/// See [getpgid(2)](http://man7.org/linux/man-pages/man2/getpgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getpgid(pid: i32) -> Result<usize, Errno> {
    syscall!(Sysno::getpgid, pid)
}

/// See [setfsuid(2)](http://man7.org/linux/man-pages/man2/setfsuid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setfsuid(uid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setfsuid, uid)
}

/// See [setfsgid(2)](http://man7.org/linux/man-pages/man2/setfsgid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setfsgid(gid: u32) -> Result<usize, Errno> {
    syscall!(Sysno::setfsgid, gid)
}

/// See [getsid(2)](http://man7.org/linux/man-pages/man2/getsid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getsid(pid: i32) -> Result<usize, Errno> {
    syscall!(Sysno::getsid, pid)
}

/// See [capget(2)](http://man7.org/linux/man-pages/man2/capget.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_capget(header: *mut c_void, dataptr: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::capget, header, dataptr)
}

/// See [capset(2)](http://man7.org/linux/man-pages/man2/capset.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_capset(header: *mut c_void, data: *const c_void) -> Result<usize, Errno> {
    syscall!(Sysno::capset, header, data)
}

/// See [rt_sigpending(2)](http://man7.org/linux/man-pages/man2/rt_sigpending.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigpending(uset: *mut c_void, sigsetsize: usize) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigpending, uset, sigsetsize)
}

/// See [rt_sigtimedwait(2)](http://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigtimedwait(
    uthese: *const c_void,
    uinfo: *mut c_void,
    uts: *const c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigtimedwait, uthese, uinfo, uts, sigsetsize)
}

/// See [rt_sigqueueinfo(2)](http://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigqueueinfo(pid: i32, sig: i32, uinfo: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigqueueinfo, pid, sig, uinfo)
}

/// See [rt_sigsuspend(2)](http://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_sigsuspend(unewset: *mut c_void, sigsetsize: usize) -> Result<usize, Errno> {
    syscall!(Sysno::rt_sigsuspend, unewset, sigsetsize)
}

/// See [sigaltstack(2)](http://man7.org/linux/man-pages/man2/sigaltstack.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sigaltstack(uss: *const c_void, uoss: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::sigaltstack, uss, uoss)
}

/// See [utime(2)](http://man7.org/linux/man-pages/man2/utime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_utime(filename: *mut c_char, times: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::utime, filename, times)
}

/// See [mknod(2)](http://man7.org/linux/man-pages/man2/mknod.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mknod(filename: *const c_char, mode: u32, dev: u32) -> Result<usize, Errno> {
    syscall!(Sysno::mknod, filename, mode, dev)
}

/// See [personality(2)](http://man7.org/linux/man-pages/man2/personality.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_personality(personality: u32) -> Result<usize, Errno> {
    syscall!(Sysno::personality, personality)
}

/// See [ustat(2)](http://man7.org/linux/man-pages/man2/ustat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ustat(dev: u32, ubuf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::ustat, dev, ubuf)
}

/// See [statfs(2)](http://man7.org/linux/man-pages/man2/statfs.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_statfs(pathname: *const c_char, buf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::statfs, pathname, buf)
}

/// See [fstatfs(2)](http://man7.org/linux/man-pages/man2/fstatfs.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fstatfs(fd: u32, buf: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::fstatfs, fd, buf)
}

/// See [sysfs(2)](http://man7.org/linux/man-pages/man2/sysfs.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sysfs(option: i32, arg1: u64, arg2: u64) -> Result<usize, Errno> {
    syscall!(Sysno::sysfs, option, arg1, arg2)
}

/// See [getpriority(2)](http://man7.org/linux/man-pages/man2/getpriority.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getpriority(which: i32, who: i32) -> Result<usize, Errno> {
    syscall!(Sysno::getpriority, which, who)
}

/// See [setpriority(2)](http://man7.org/linux/man-pages/man2/setpriority.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setpriority(which: i32, who: i32, niceval: i32) -> Result<usize, Errno> {
    syscall!(Sysno::setpriority, which, who, niceval)
}

/// See [sched_setparam(2)](http://man7.org/linux/man-pages/man2/sched_setparam.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_setparam(pid: i32, param: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setparam, pid, param)
}

/// See [sched_getparam(2)](http://man7.org/linux/man-pages/man2/sched_getparam.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_getparam(pid: i32, param: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getparam, pid, param)
}

/// See [sched_setscheduler(2)](http://man7.org/linux/man-pages/man2/sched_setscheduler.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_setscheduler(
    pid: i32,
    policy: i32,
    param: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setscheduler, pid, policy, param)
}

/// See [sched_getscheduler(2)](http://man7.org/linux/man-pages/man2/sched_getscheduler.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_getscheduler(pid: i32) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getscheduler, pid)
}

/// See [sched_get_priority_max(2)](http://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_get_priority_max(policy: i32) -> Result<usize, Errno> {
    syscall!(Sysno::sched_get_priority_max, policy)
}

/// See [sched_get_priority_min(2)](http://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_get_priority_min(policy: i32) -> Result<usize, Errno> {
    syscall!(Sysno::sched_get_priority_min, policy)
}

/// See [sched_rr_get_interval(2)](http://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_rr_get_interval(pid: i32, interval: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::sched_rr_get_interval, pid, interval)
}

/// See [mlock(2)](http://man7.org/linux/man-pages/man2/mlock.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mlock(start: u64, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::mlock, start, len)
}

/// See [munlock(2)](http://man7.org/linux/man-pages/man2/munlock.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_munlock(start: u64, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::munlock, start, len)
}

/// See [mlockall(2)](http://man7.org/linux/man-pages/man2/mlockall.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mlockall(flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::mlockall, flags)
}

/// See [munlockall(2)](http://man7.org/linux/man-pages/man2/munlockall.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_munlockall() -> Result<usize, Errno> {
    syscall!(Sysno::munlockall)
}

/// See [vhangup(2)](http://man7.org/linux/man-pages/man2/vhangup.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_vhangup() -> Result<usize, Errno> {
    syscall!(Sysno::vhangup)
}

/// See [modify_ldt(2)](http://man7.org/linux/man-pages/man2/modify_ldt.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_modify_ldt(func: i32, ptr: *mut c_void, bytecount: u64) -> Result<usize, Errno> {
    syscall!(Sysno::modify_ldt, func, ptr, bytecount)
}

/// See [pivot_root(2)](http://man7.org/linux/man-pages/man2/pivot_root.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pivot_root(
    new_root: *const c_char,
    put_old: *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::pivot_root, new_root, put_old)
}

/// See [prctl(2)](http://man7.org/linux/man-pages/man2/prctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_prctl(
    option: i32,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::prctl, option, arg2, arg3, arg4, arg5)
}

/// See [arch_prctl(2)](http://man7.org/linux/man-pages/man2/arch_prctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_arch_prctl(option: i32, arg2: u64) -> Result<usize, Errno> {
    syscall!(Sysno::arch_prctl, option, arg2)
}

/// See [adjtimex(2)](http://man7.org/linux/man-pages/man2/adjtimex.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_adjtimex(txc_p: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::adjtimex, txc_p)
}

/// See [setrlimit(2)](http://man7.org/linux/man-pages/man2/setrlimit.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setrlimit(resource: u32, rlim: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::setrlimit, resource, rlim)
}

/// See [chroot(2)](http://man7.org/linux/man-pages/man2/chroot.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_chroot(filename: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::chroot, filename)
}

/// See [sync(2)](http://man7.org/linux/man-pages/man2/sync.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sync() -> Result<usize, Errno> {
    syscall!(Sysno::sync)
}

/// See [settimeofday(2)](http://man7.org/linux/man-pages/man2/settimeofday.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_settimeofday(tv: *mut c_void, tz: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::settimeofday, tv, tz)
}

/// See [mount(2)](http://man7.org/linux/man-pages/man2/mount.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mount(
    dev_name: *mut c_char,
    dir_name: *mut c_char,
    r#type: *mut c_char,
    flags: u64,
    data: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::mount, dev_name, dir_name, r#type, flags, data)
}

/// See [umount2(2)](http://man7.org/linux/man-pages/man2/umount2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_umount2(name: *mut c_char, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::umount2, name, flags)
}

/// See [swapon(2)](http://man7.org/linux/man-pages/man2/swapon.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_swapon(specialfile: *const c_char, swap_flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::swapon, specialfile, swap_flags)
}

/// See [swapoff(2)](http://man7.org/linux/man-pages/man2/swapoff.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_swapoff(specialfile: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::swapoff, specialfile)
}

/// See [reboot(2)](http://man7.org/linux/man-pages/man2/reboot.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_reboot(
    magic1: i32,
    magic2: i32,
    cmd: u32,
    arg: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::reboot, magic1, magic2, cmd, arg)
}

/// See [sethostname(2)](http://man7.org/linux/man-pages/man2/sethostname.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sethostname(name: *mut c_char, len: i32) -> Result<usize, Errno> {
    syscall!(Sysno::sethostname, name, len)
}

/// See [setdomainname(2)](http://man7.org/linux/man-pages/man2/setdomainname.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setdomainname(name: *mut c_char, len: i32) -> Result<usize, Errno> {
    syscall!(Sysno::setdomainname, name, len)
}

/// See [iopl(2)](http://man7.org/linux/man-pages/man2/iopl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_iopl(level: u32) -> Result<usize, Errno> {
    syscall!(Sysno::iopl, level)
}

/// See [ioperm(2)](http://man7.org/linux/man-pages/man2/ioperm.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ioperm(from: u64, num: u64, turn_on: i32) -> Result<usize, Errno> {
    syscall!(Sysno::ioperm, from, num, turn_on)
}

/// See [gettid(2)](http://man7.org/linux/man-pages/man2/gettid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_gettid() -> Result<usize, Errno> {
    syscall!(Sysno::gettid)
}

/// See [readahead(2)](http://man7.org/linux/man-pages/man2/readahead.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_readahead(fd: i32, offset: i64, count: usize) -> Result<usize, Errno> {
    syscall!(Sysno::readahead, fd, offset, count)
}

/// See [setxattr(2)](http://man7.org/linux/man-pages/man2/setxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setxattr(
    pathname: *const c_char,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::setxattr, pathname, name, value, size, flags)
}

/// See [lsetxattr(2)](http://man7.org/linux/man-pages/man2/lsetxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lsetxattr(
    pathname: *const c_char,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::lsetxattr, pathname, name, value, size, flags)
}

/// See [fsetxattr(2)](http://man7.org/linux/man-pages/man2/fsetxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fsetxattr(
    fd: i32,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::fsetxattr, fd, name, value, size, flags)
}

/// See [getxattr(2)](http://man7.org/linux/man-pages/man2/getxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getxattr(
    pathname: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::getxattr, pathname, name, value, size)
}

/// See [lgetxattr(2)](http://man7.org/linux/man-pages/man2/lgetxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lgetxattr(
    pathname: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::lgetxattr, pathname, name, value, size)
}

/// See [fgetxattr(2)](http://man7.org/linux/man-pages/man2/fgetxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fgetxattr(
    fd: i32,
    name: *const c_char,
    value: *mut c_void,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::fgetxattr, fd, name, value, size)
}

/// See [listxattr(2)](http://man7.org/linux/man-pages/man2/listxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_listxattr(
    pathname: *const c_char,
    list: *mut c_char,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::listxattr, pathname, list, size)
}

/// See [llistxattr(2)](http://man7.org/linux/man-pages/man2/llistxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_llistxattr(
    pathname: *const c_char,
    list: *mut c_char,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::llistxattr, pathname, list, size)
}

/// See [flistxattr(2)](http://man7.org/linux/man-pages/man2/flistxattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_flistxattr(fd: i32, list: *mut c_char, size: usize) -> Result<usize, Errno> {
    syscall!(Sysno::flistxattr, fd, list, size)
}

/// See [removexattr(2)](http://man7.org/linux/man-pages/man2/removexattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_removexattr(
    pathname: *const c_char,
    name: *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::removexattr, pathname, name)
}

/// See [lremovexattr(2)](http://man7.org/linux/man-pages/man2/lremovexattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_lremovexattr(
    pathname: *const c_char,
    name: *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::lremovexattr, pathname, name)
}

/// See [fremovexattr(2)](http://man7.org/linux/man-pages/man2/fremovexattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fremovexattr(fd: i32, name: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::fremovexattr, fd, name)
}

/// See [tkill(2)](http://man7.org/linux/man-pages/man2/tkill.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_tkill(pid: i32, sig: i32) -> Result<usize, Errno> {
    syscall!(Sysno::tkill, pid, sig)
}

/// See [time(2)](http://man7.org/linux/man-pages/man2/time.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_time(tloc: *mut i64) -> Result<usize, Errno> {
    syscall!(Sysno::time, tloc)
}

/// See [futex(2)](http://man7.org/linux/man-pages/man2/futex.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_futex(
    uaddr: *mut u32,
    op: i32,
    val: u32,
    utime: *const c_void,
    uaddr2: *mut u32,
    val3: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::futex, uaddr, op, val, utime, uaddr2, val3)
}

/// See [sched_setaffinity(2)](http://man7.org/linux/man-pages/man2/sched_setaffinity.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_setaffinity(
    pid: i32,
    len: u32,
    user_mask_ptr: *mut u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setaffinity, pid, len, user_mask_ptr)
}

/// See [sched_getaffinity(2)](http://man7.org/linux/man-pages/man2/sched_getaffinity.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_getaffinity(
    pid: i32,
    len: u32,
    user_mask_ptr: *mut u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getaffinity, pid, len, user_mask_ptr)
}

/// See [io_setup(2)](http://man7.org/linux/man-pages/man2/io_setup.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_setup(nr_events: u32, ctxp: *mut u64) -> Result<usize, Errno> {
    syscall!(Sysno::io_setup, nr_events, ctxp)
}

/// See [io_destroy(2)](http://man7.org/linux/man-pages/man2/io_destroy.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_destroy(ctx: u64) -> Result<usize, Errno> {
    syscall!(Sysno::io_destroy, ctx)
}

/// See [io_getevents(2)](http://man7.org/linux/man-pages/man2/io_getevents.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_getevents(
    ctx_id: u64,
    min_nr: i64,
    nr: i64,
    events: *mut c_void,
    timeout: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::io_getevents, ctx_id, min_nr, nr, events, timeout)
}

/// See [io_submit(2)](http://man7.org/linux/man-pages/man2/io_submit.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_submit(
    ctx_id: u64,
    nr: i64,
    iocbpp: *mut *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::io_submit, ctx_id, nr, iocbpp)
}

/// See [io_cancel(2)](http://man7.org/linux/man-pages/man2/io_cancel.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_cancel(
    ctx_id: u64,
    iocb: *mut c_void,
    result: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::io_cancel, ctx_id, iocb, result)
}

/// See [epoll_create(2)](http://man7.org/linux/man-pages/man2/epoll_create.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_create(size: i32) -> Result<usize, Errno> {
    syscall!(Sysno::epoll_create, size)
}

/// See [remap_file_pages(2)](http://man7.org/linux/man-pages/man2/remap_file_pages.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_remap_file_pages(
    start: u64,
    size: u64,
    prot: u64,
    pgoff: u64,
    flags: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::remap_file_pages, start, size, prot, pgoff, flags)
}

/// See [getdents64(2)](http://man7.org/linux/man-pages/man2/getdents64.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getdents64(fd: u32, dirent: *mut c_void, count: u32) -> Result<usize, Errno> {
    syscall!(Sysno::getdents64, fd, dirent, count)
}

/// See [set_tid_address(2)](http://man7.org/linux/man-pages/man2/set_tid_address.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_set_tid_address(tidptr: *mut i32) -> Result<usize, Errno> {
    syscall!(Sysno::set_tid_address, tidptr)
}

/// See [restart_syscall(2)](http://man7.org/linux/man-pages/man2/restart_syscall.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_restart_syscall() -> Result<usize, Errno> {
    syscall!(Sysno::restart_syscall)
}

/// See [semtimedop(2)](http://man7.org/linux/man-pages/man2/semtimedop.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_semtimedop(
    semid: i32,
    tsops: *mut c_void,
    nsops: u32,
    timeout: *const c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::semtimedop, semid, tsops, nsops, timeout)
}

/// See [fadvise64(2)](http://man7.org/linux/man-pages/man2/fadvise64.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fadvise64(fd: i32, offset: i64, len: usize, advice: i32) -> Result<usize, Errno> {
    syscall!(Sysno::fadvise64, fd, offset, len, advice)
}

/// See [timer_create(2)](http://man7.org/linux/man-pages/man2/timer_create.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timer_create(
    which_clock: i32,
    timer_event_spec: *mut c_void,
    created_timer_id: *mut i32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::timer_create,
        which_clock,
        timer_event_spec,
        created_timer_id
    )
}

/// See [timer_settime(2)](http://man7.org/linux/man-pages/man2/timer_settime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timer_settime(
    timer_id: i32,
    flags: i32,
    new_setting: *const c_void,
    old_setting: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::timer_settime,
        timer_id,
        flags,
        new_setting,
        old_setting
    )
}

/// See [timer_gettime(2)](http://man7.org/linux/man-pages/man2/timer_gettime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timer_gettime(timer_id: i32, setting: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::timer_gettime, timer_id, setting)
}

/// See [timer_getoverrun(2)](http://man7.org/linux/man-pages/man2/timer_getoverrun.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timer_getoverrun(timer_id: i32) -> Result<usize, Errno> {
    syscall!(Sysno::timer_getoverrun, timer_id)
}

/// See [timer_delete(2)](http://man7.org/linux/man-pages/man2/timer_delete.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timer_delete(timer_id: i32) -> Result<usize, Errno> {
    syscall!(Sysno::timer_delete, timer_id)
}

/// See [clock_settime(2)](http://man7.org/linux/man-pages/man2/clock_settime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clock_settime(which_clock: i32, tp: *const c_void) -> Result<usize, Errno> {
    syscall!(Sysno::clock_settime, which_clock, tp)
}

/// See [clock_gettime(2)](http://man7.org/linux/man-pages/man2/clock_gettime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clock_gettime(which_clock: i32, tp: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::clock_gettime, which_clock, tp)
}

/// See [clock_getres(2)](http://man7.org/linux/man-pages/man2/clock_getres.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clock_getres(which_clock: i32, tp: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::clock_getres, which_clock, tp)
}

/// See [clock_nanosleep(2)](http://man7.org/linux/man-pages/man2/clock_nanosleep.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clock_nanosleep(
    which_clock: i32,
    flags: i32,
    rqtp: *const c_void,
    rmtp: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::clock_nanosleep, which_clock, flags, rqtp, rmtp)
}

/// See [exit_group(2)](http://man7.org/linux/man-pages/man2/exit_group.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_exit_group(error_code: i32) -> Result<usize, Errno> {
    syscall!(Sysno::exit_group, error_code)
}

/// See [epoll_wait(2)](http://man7.org/linux/man-pages/man2/epoll_wait.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_wait(
    epfd: i32,
    events: *mut c_void,
    maxevents: i32,
    timeout: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::epoll_wait, epfd, events, maxevents, timeout)
}

/// See [epoll_ctl(2)](http://man7.org/linux/man-pages/man2/epoll_ctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_ctl(
    epfd: i32,
    op: i32,
    fd: i32,
    event: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::epoll_ctl, epfd, op, fd, event)
}

/// See [tgkill(2)](http://man7.org/linux/man-pages/man2/tgkill.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_tgkill(tgid: i32, pid: i32, sig: i32) -> Result<usize, Errno> {
    syscall!(Sysno::tgkill, tgid, pid, sig)
}

/// See [utimes(2)](http://man7.org/linux/man-pages/man2/utimes.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_utimes(filename: *mut c_char, utimes: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::utimes, filename, utimes)
}

/// See [mq_open(2)](http://man7.org/linux/man-pages/man2/mq_open.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_open(
    u_name: *const c_char,
    oflag: i32,
    mode: u32,
    u_attr: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::mq_open, u_name, oflag, mode, u_attr)
}

/// See [mq_unlink(2)](http://man7.org/linux/man-pages/man2/mq_unlink.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_unlink(u_name: *const c_char) -> Result<usize, Errno> {
    syscall!(Sysno::mq_unlink, u_name)
}

/// See [mq_timedsend(2)](http://man7.org/linux/man-pages/man2/mq_timedsend.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_timedsend(
    mqdes: i32,
    u_msg_ptr: *const c_char,
    msg_len: usize,
    msg_prio: u32,
    u_abs_timeout: *const c_void,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::mq_timedsend,
        mqdes,
        u_msg_ptr,
        msg_len,
        msg_prio,
        u_abs_timeout
    )
}

/// See [mq_timedreceive(2)](http://man7.org/linux/man-pages/man2/mq_timedreceive.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_timedreceive(
    mqdes: i32,
    u_msg_ptr: *mut c_char,
    msg_len: usize,
    u_msg_prio: *mut u32,
    u_abs_timeout: *const c_void,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::mq_timedreceive,
        mqdes,
        u_msg_ptr,
        msg_len,
        u_msg_prio,
        u_abs_timeout
    )
}

/// See [mq_notify(2)](http://man7.org/linux/man-pages/man2/mq_notify.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_notify(mqdes: i32, u_notification: *const c_void) -> Result<usize, Errno> {
    syscall!(Sysno::mq_notify, mqdes, u_notification)
}

/// See [mq_getsetattr(2)](http://man7.org/linux/man-pages/man2/mq_getsetattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mq_getsetattr(
    mqdes: i32,
    u_mqstat: *const c_void,
    u_omqstat: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::mq_getsetattr, mqdes, u_mqstat, u_omqstat)
}

/// See [waitid(2)](http://man7.org/linux/man-pages/man2/waitid.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_waitid(
    which: i32,
    upid: i32,
    infop: *mut c_void,
    options: i32,
    ru: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::waitid, which, upid, infop, options, ru)
}

/// See [add_key(2)](http://man7.org/linux/man-pages/man2/add_key.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_add_key(
    _type: *const c_char,
    _description: *const c_char,
    _payload: *const c_void,
    plen: usize,
    ringid: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::add_key, _type, _description, _payload, plen, ringid)
}

/// See [request_key(2)](http://man7.org/linux/man-pages/man2/request_key.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_request_key(
    _type: *const c_char,
    _description: *const c_char,
    _callout_info: *const c_char,
    destringid: i32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::request_key,
        _type,
        _description,
        _callout_info,
        destringid
    )
}

/// See [keyctl(2)](http://man7.org/linux/man-pages/man2/keyctl.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_keyctl(
    option: i32,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::keyctl, option, arg2, arg3, arg4, arg5)
}

/// See [ioprio_set(2)](http://man7.org/linux/man-pages/man2/ioprio_set.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ioprio_set(which: i32, who: i32, ioprio: i32) -> Result<usize, Errno> {
    syscall!(Sysno::ioprio_set, which, who, ioprio)
}

/// See [ioprio_get(2)](http://man7.org/linux/man-pages/man2/ioprio_get.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ioprio_get(which: i32, who: i32) -> Result<usize, Errno> {
    syscall!(Sysno::ioprio_get, which, who)
}

/// See [inotify_init(2)](http://man7.org/linux/man-pages/man2/inotify_init.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_inotify_init() -> Result<usize, Errno> {
    syscall!(Sysno::inotify_init)
}

/// See [inotify_add_watch(2)](http://man7.org/linux/man-pages/man2/inotify_add_watch.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_inotify_add_watch(
    fd: i32,
    pathname: *const c_char,
    mask: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::inotify_add_watch, fd, pathname, mask)
}

/// See [inotify_rm_watch(2)](http://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_inotify_rm_watch(fd: i32, wd: i32) -> Result<usize, Errno> {
    syscall!(Sysno::inotify_rm_watch, fd, wd)
}

/// See [openat(2)](http://man7.org/linux/man-pages/man2/openat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_openat(
    dfd: i32,
    filename: *const c_char,
    flags: i32,
    mode: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::openat, dfd, filename, flags, mode)
}

/// See [mkdirat(2)](http://man7.org/linux/man-pages/man2/mkdirat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mkdirat(dfd: i32, pathname: *const c_char, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::mkdirat, dfd, pathname, mode)
}

/// See [mknodat(2)](http://man7.org/linux/man-pages/man2/mknodat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mknodat(
    dfd: i32,
    filename: *const c_char,
    mode: u32,
    dev: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::mknodat, dfd, filename, mode, dev)
}

/// See [fchownat(2)](http://man7.org/linux/man-pages/man2/fchownat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchownat(
    dfd: i32,
    filename: *const c_char,
    user: u32,
    group: u32,
    flag: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::fchownat, dfd, filename, user, group, flag)
}

/// See [futimesat(2)](http://man7.org/linux/man-pages/man2/futimesat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_futimesat(
    dfd: i32,
    filename: *const c_char,
    utimes: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::futimesat, dfd, filename, utimes)
}

/// See [newfstatat(2)](http://man7.org/linux/man-pages/man2/newfstatat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_newfstatat(
    dfd: i32,
    filename: *const c_char,
    statbuf: *mut c_void,
    flag: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::newfstatat, dfd, filename, statbuf, flag)
}

/// See [unlinkat(2)](http://man7.org/linux/man-pages/man2/unlinkat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_unlinkat(dfd: i32, pathname: *const c_char, flag: i32) -> Result<usize, Errno> {
    syscall!(Sysno::unlinkat, dfd, pathname, flag)
}

/// See [renameat(2)](http://man7.org/linux/man-pages/man2/renameat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_renameat(
    olddfd: i32,
    oldname: *const c_char,
    newdfd: i32,
    newname: *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::renameat, olddfd, oldname, newdfd, newname)
}

/// See [linkat(2)](http://man7.org/linux/man-pages/man2/linkat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_linkat(
    olddfd: i32,
    oldname: *const c_char,
    newdfd: i32,
    newname: *const c_char,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::linkat, olddfd, oldname, newdfd, newname, flags)
}

/// See [symlinkat(2)](http://man7.org/linux/man-pages/man2/symlinkat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_symlinkat(
    oldname: *const c_char,
    newdfd: i32,
    newname: *const c_char,
) -> Result<usize, Errno> {
    syscall!(Sysno::symlinkat, oldname, newdfd, newname)
}

/// See [readlinkat(2)](http://man7.org/linux/man-pages/man2/readlinkat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_readlinkat(
    dfd: i32,
    pathname: *const c_char,
    buf: *mut u8,
    bufsiz: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::readlinkat, dfd, pathname, buf, bufsiz)
}

/// See [fchmodat(2)](http://man7.org/linux/man-pages/man2/fchmodat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchmodat(dfd: i32, filename: *const c_char, mode: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fchmodat, dfd, filename, mode)
}

/// See [faccessat(2)](http://man7.org/linux/man-pages/man2/faccessat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_faccessat(dfd: i32, filename: *const c_char, mode: i32) -> Result<usize, Errno> {
    syscall!(Sysno::faccessat, dfd, filename, mode)
}

/// See [pselect6(2)](http://man7.org/linux/man-pages/man2/pselect6.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pselect6(
    n: i32,
    inp: *mut c_void,
    outp: *mut c_void,
    exp: *mut c_void,
    tsp: *mut c_void,
    sig: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::pselect6, n, inp, outp, exp, tsp, sig)
}

/// See [ppoll(2)](http://man7.org/linux/man-pages/man2/ppoll.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_ppoll(
    ufds: *mut c_void,
    nfds: u32,
    tsp: *mut c_void,
    sigmask: *const c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::ppoll, ufds, nfds, tsp, sigmask, sigsetsize)
}

/// See [unshare(2)](http://man7.org/linux/man-pages/man2/unshare.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_unshare(unshare_flags: u64) -> Result<usize, Errno> {
    syscall!(Sysno::unshare, unshare_flags)
}

/// See [set_robust_list(2)](http://man7.org/linux/man-pages/man2/set_robust_list.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_set_robust_list(head: *mut c_void, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::set_robust_list, head, len)
}

/// See [get_robust_list(2)](http://man7.org/linux/man-pages/man2/get_robust_list.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_get_robust_list(
    pid: i32,
    head_ptr: *mut *mut c_void,
    len_ptr: *mut usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::get_robust_list, pid, head_ptr, len_ptr)
}

/// See [splice(2)](http://man7.org/linux/man-pages/man2/splice.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_splice(
    fd_in: i32,
    off_in: *mut i64,
    fd_out: i32,
    off_out: *mut i64,
    len: usize,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::splice, fd_in, off_in, fd_out, off_out, len, flags)
}

/// See [tee(2)](http://man7.org/linux/man-pages/man2/tee.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_tee(fdin: i32, fdout: i32, len: usize, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::tee, fdin, fdout, len, flags)
}

/// See [sync_file_range(2)](http://man7.org/linux/man-pages/man2/sync_file_range.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sync_file_range(
    fd: i32,
    offset: i64,
    nbytes: i64,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::sync_file_range, fd, offset, nbytes, flags)
}

/// See [vmsplice(2)](http://man7.org/linux/man-pages/man2/vmsplice.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_vmsplice(
    fd: i32,
    uiov: *const c_void,
    nr_segs: u64,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::vmsplice, fd, uiov, nr_segs, flags)
}

/// See [utimensat(2)](http://man7.org/linux/man-pages/man2/utimensat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_utimensat(
    dfd: i32,
    filename: *const c_char,
    utimes: *mut c_void,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::utimensat, dfd, filename, utimes, flags)
}

/// See [epoll_pwait(2)](http://man7.org/linux/man-pages/man2/epoll_pwait.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_pwait(
    epfd: i32,
    events: *mut c_void,
    maxevents: i32,
    timeout: i32,
    sigmask: *const c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::epoll_pwait,
        epfd,
        events,
        maxevents,
        timeout,
        sigmask,
        sigsetsize
    )
}

/// See [signalfd(2)](http://man7.org/linux/man-pages/man2/signalfd.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_signalfd(
    ufd: i32,
    user_mask: *mut c_void,
    sizemask: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::signalfd, ufd, user_mask, sizemask)
}

/// See [timerfd_create(2)](http://man7.org/linux/man-pages/man2/timerfd_create.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timerfd_create(clockid: i32, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::timerfd_create, clockid, flags)
}

/// See [eventfd(2)](http://man7.org/linux/man-pages/man2/eventfd.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_eventfd(count: u32) -> Result<usize, Errno> {
    syscall!(Sysno::eventfd, count)
}

/// See [fallocate(2)](http://man7.org/linux/man-pages/man2/fallocate.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fallocate(fd: i32, mode: i32, offset: i64, len: i64) -> Result<usize, Errno> {
    syscall!(Sysno::fallocate, fd, mode, offset, len)
}

/// See [timerfd_settime(2)](http://man7.org/linux/man-pages/man2/timerfd_settime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timerfd_settime(
    ufd: i32,
    flags: i32,
    utmr: *const c_void,
    otmr: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::timerfd_settime, ufd, flags, utmr, otmr)
}

/// See [timerfd_gettime(2)](http://man7.org/linux/man-pages/man2/timerfd_gettime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_timerfd_gettime(ufd: i32, otmr: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::timerfd_gettime, ufd, otmr)
}

/// See [accept4(2)](http://man7.org/linux/man-pages/man2/accept4.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_accept4(
    fd: i32,
    upeer_sockaddr: *mut c_void,
    upeer_addrlen: *mut i32,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::accept4, fd, upeer_sockaddr, upeer_addrlen, flags)
}

/// See [signalfd4(2)](http://man7.org/linux/man-pages/man2/signalfd4.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_signalfd4(
    ufd: i32,
    user_mask: *mut c_void,
    sizemask: usize,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::signalfd4, ufd, user_mask, sizemask, flags)
}

/// See [eventfd2(2)](http://man7.org/linux/man-pages/man2/eventfd2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_eventfd2(count: u32, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::eventfd2, count, flags)
}

/// See [epoll_create1(2)](http://man7.org/linux/man-pages/man2/epoll_create1.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_create1(flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::epoll_create1, flags)
}

/// See [dup3(2)](http://man7.org/linux/man-pages/man2/dup3.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_dup3(oldfd: u32, newfd: u32, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::dup3, oldfd, newfd, flags)
}

/// See [pipe2(2)](http://man7.org/linux/man-pages/man2/pipe2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pipe2(fildes: *mut i32, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::pipe2, fildes, flags)
}

/// See [inotify_init1(2)](http://man7.org/linux/man-pages/man2/inotify_init1.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_inotify_init1(flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::inotify_init1, flags)
}

/// See [preadv(2)](http://man7.org/linux/man-pages/man2/preadv.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_preadv(
    fd: u64,
    vec: *const c_void,
    vlen: u64,
    pos_l: u64,
    pos_h: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::preadv, fd, vec, vlen, pos_l, pos_h)
}

/// See [pwritev(2)](http://man7.org/linux/man-pages/man2/pwritev.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pwritev(
    fd: u64,
    vec: *const c_void,
    vlen: u64,
    pos_l: u64,
    pos_h: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::pwritev, fd, vec, vlen, pos_l, pos_h)
}

/// See [rt_tgsigqueueinfo(2)](http://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rt_tgsigqueueinfo(
    tgid: i32,
    pid: i32,
    sig: i32,
    uinfo: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::rt_tgsigqueueinfo, tgid, pid, sig, uinfo)
}

/// See [perf_event_open(2)](http://man7.org/linux/man-pages/man2/perf_event_open.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_perf_event_open(
    attr_uptr: *mut c_void,
    pid: i32,
    cpu: i32,
    group_fd: i32,
    flags: u64,
) -> Result<usize, Errno> {
    syscall!(Sysno::perf_event_open, attr_uptr, pid, cpu, group_fd, flags)
}

/// See [recvmmsg(2)](http://man7.org/linux/man-pages/man2/recvmmsg.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_recvmmsg(
    fd: i32,
    mmsg: *mut c_void,
    vlen: u32,
    flags: u32,
    timeout: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::recvmmsg, fd, mmsg, vlen, flags, timeout)
}

/// See [fanotify_init(2)](http://man7.org/linux/man-pages/man2/fanotify_init.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fanotify_init(flags: u32, event_f_flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fanotify_init, flags, event_f_flags)
}

/// See [fanotify_mark(2)](http://man7.org/linux/man-pages/man2/fanotify_mark.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fanotify_mark(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dfd: i32,
    pathname: *const c_char,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::fanotify_mark,
        fanotify_fd,
        flags,
        mask,
        dfd,
        pathname
    )
}

/// See [prlimit64(2)](http://man7.org/linux/man-pages/man2/prlimit64.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_prlimit64(
    pid: i32,
    resource: u32,
    new_rlim: *const c_void,
    old_rlim: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::prlimit64, pid, resource, new_rlim, old_rlim)
}

/// See [name_to_handle_at(2)](http://man7.org/linux/man-pages/man2/name_to_handle_at.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_name_to_handle_at(
    dfd: i32,
    name: *const c_char,
    handle: *mut c_void,
    mnt_id: *mut i32,
    flag: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::name_to_handle_at, dfd, name, handle, mnt_id, flag)
}

/// See [open_by_handle_at(2)](http://man7.org/linux/man-pages/man2/open_by_handle_at.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_open_by_handle_at(
    mountdirfd: i32,
    handle: *mut c_void,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::open_by_handle_at, mountdirfd, handle, flags)
}

/// See [clock_adjtime(2)](http://man7.org/linux/man-pages/man2/clock_adjtime.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clock_adjtime(which_clock: i32, utx: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::clock_adjtime, which_clock, utx)
}

/// See [syncfs(2)](http://man7.org/linux/man-pages/man2/syncfs.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_syncfs(fd: i32) -> Result<usize, Errno> {
    syscall!(Sysno::syncfs, fd)
}

/// See [sendmmsg(2)](http://man7.org/linux/man-pages/man2/sendmmsg.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sendmmsg(
    fd: i32,
    mmsg: *mut c_void,
    vlen: u32,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::sendmmsg, fd, mmsg, vlen, flags)
}

/// See [setns(2)](http://man7.org/linux/man-pages/man2/setns.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_setns(fd: i32, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::setns, fd, flags)
}

/// See [getcpu(2)](http://man7.org/linux/man-pages/man2/getcpu.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getcpu(
    cpup: *mut u32,
    nodep: *mut u32,
    unused: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::getcpu, cpup, nodep, unused)
}

/// See [process_vm_readv(2)](http://man7.org/linux/man-pages/man2/process_vm_readv.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_process_vm_readv(
    pid: i32,
    lvec: *const c_void,
    liovcnt: u64,
    rvec: *const c_void,
    riovcnt: u64,
    flags: u64,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::process_vm_readv,
        pid,
        lvec,
        liovcnt,
        rvec,
        riovcnt,
        flags
    )
}

/// See [process_vm_writev(2)](http://man7.org/linux/man-pages/man2/process_vm_writev.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_process_vm_writev(
    pid: i32,
    lvec: *const c_void,
    liovcnt: u64,
    rvec: *const c_void,
    riovcnt: u64,
    flags: u64,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::process_vm_writev,
        pid,
        lvec,
        liovcnt,
        rvec,
        riovcnt,
        flags
    )
}

/// See [sched_setattr(2)](http://man7.org/linux/man-pages/man2/sched_setattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_setattr(pid: i32, uattr: *mut c_void, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setattr, pid, uattr, flags)
}

/// See [sched_getattr(2)](http://man7.org/linux/man-pages/man2/sched_getattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_sched_getattr(
    pid: i32,
    uattr: *mut c_void,
    size: u32,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getattr, pid, uattr, size, flags)
}

/// See [renameat2(2)](http://man7.org/linux/man-pages/man2/renameat2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_renameat2(
    olddfd: i32,
    oldname: *const c_char,
    newdfd: i32,
    newname: *const c_char,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::renameat2, olddfd, oldname, newdfd, newname, flags)
}

/// See [seccomp(2)](http://man7.org/linux/man-pages/man2/seccomp.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_seccomp(op: u32, flags: u32, uargs: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::seccomp, op, flags, uargs)
}

/// See [getrandom(2)](http://man7.org/linux/man-pages/man2/getrandom.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_getrandom(ubuf: *mut c_char, len: usize, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::getrandom, ubuf, len, flags)
}

/// See [memfd_create(2)](http://man7.org/linux/man-pages/man2/memfd_create.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_memfd_create(uname: *const c_char, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::memfd_create, uname, flags)
}

/// See [bpf(2)](http://man7.org/linux/man-pages/man2/bpf.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_bpf(cmd: i32, uattr: *mut c_void, size: u32) -> Result<usize, Errno> {
    syscall!(Sysno::bpf, cmd, uattr, size)
}

/// See [execveat(2)](http://man7.org/linux/man-pages/man2/execveat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_execveat(
    fd: i32,
    filename: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::execveat, fd, filename, argv, envp, flags)
}

/// See [membarrier(2)](http://man7.org/linux/man-pages/man2/membarrier.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_membarrier(cmd: i32, flags: u32, cpu_id: i32) -> Result<usize, Errno> {
    syscall!(Sysno::membarrier, cmd, flags, cpu_id)
}

/// See [mlock2(2)](http://man7.org/linux/man-pages/man2/mlock2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mlock2(start: u64, len: usize, flags: i32) -> Result<usize, Errno> {
    syscall!(Sysno::mlock2, start, len, flags)
}

/// See [copy_file_range(2)](http://man7.org/linux/man-pages/man2/copy_file_range.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_copy_file_range(
    fd_in: i32,
    off_in: *mut i64,
    fd_out: i32,
    off_out: *mut i64,
    len: usize,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::copy_file_range,
        fd_in,
        off_in,
        fd_out,
        off_out,
        len,
        flags
    )
}

/// See [preadv2(2)](http://man7.org/linux/man-pages/man2/preadv2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_preadv2(
    fd: u64,
    vec: *const c_void,
    vlen: u64,
    pos_l: u64,
    pos_h: u64,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::preadv2, fd, vec, vlen, pos_l, pos_h, flags)
}

/// See [pwritev2(2)](http://man7.org/linux/man-pages/man2/pwritev2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pwritev2(
    fd: u64,
    vec: *const c_void,
    vlen: u64,
    pos_l: u64,
    pos_h: u64,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::pwritev2, fd, vec, vlen, pos_l, pos_h, flags)
}

/// See [pkey_mprotect(2)](http://man7.org/linux/man-pages/man2/pkey_mprotect.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pkey_mprotect(
    start: u64,
    len: usize,
    prot: u64,
    pkey: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::pkey_mprotect, start, len, prot, pkey)
}

/// See [pkey_alloc(2)](http://man7.org/linux/man-pages/man2/pkey_alloc.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pkey_alloc(flags: u64, init_val: u64) -> Result<usize, Errno> {
    syscall!(Sysno::pkey_alloc, flags, init_val)
}

/// See [pkey_free(2)](http://man7.org/linux/man-pages/man2/pkey_free.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pkey_free(pkey: i32) -> Result<usize, Errno> {
    syscall!(Sysno::pkey_free, pkey)
}

/// See [statx(2)](http://man7.org/linux/man-pages/man2/statx.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_statx(
    dfd: i32,
    filename: *const c_char,
    flags: u32,
    mask: u32,
    buffer: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(Sysno::statx, dfd, filename, flags, mask, buffer)
}

/// See [io_pgetevents(2)](http://man7.org/linux/man-pages/man2/io_pgetevents.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_pgetevents(
    ctx_id: u64,
    min_nr: i64,
    nr: i64,
    events: *mut c_void,
    timeout: *mut c_void,
    usig: *mut c_void,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::io_pgetevents,
        ctx_id,
        min_nr,
        nr,
        events,
        timeout,
        usig
    )
}

/// See [rseq(2)](http://man7.org/linux/man-pages/man2/rseq.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_rseq(
    rseq: *mut c_void,
    rseq_len: u32,
    flags: i32,
    sig: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::rseq, rseq, rseq_len, flags, sig)
}

/// See [pidfd_send_signal(2)](http://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pidfd_send_signal(
    pidfd: i32,
    sig: i32,
    info: *mut c_void,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::pidfd_send_signal, pidfd, sig, info, flags)
}

/// See [io_uring_setup(2)](http://man7.org/linux/man-pages/man2/io_uring_setup.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_uring_setup(entries: u32, params: *mut c_void) -> Result<usize, Errno> {
    syscall!(Sysno::io_uring_setup, entries, params)
}

/// See [io_uring_enter(2)](http://man7.org/linux/man-pages/man2/io_uring_enter.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_uring_enter(
    fd: u32,
    to_submit: u32,
    min_complete: u32,
    flags: u32,
    argp: *const c_void,
    argsz: usize,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::io_uring_enter,
        fd,
        to_submit,
        min_complete,
        flags,
        argp,
        argsz
    )
}

/// See [io_uring_register(2)](http://man7.org/linux/man-pages/man2/io_uring_register.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_io_uring_register(
    fd: u32,
    opcode: u32,
    arg: *mut c_void,
    nr_args: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::io_uring_register, fd, opcode, arg, nr_args)
}

/// See [open_tree(2)](http://man7.org/linux/man-pages/man2/open_tree.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_open_tree(dfd: i32, filename: *const c_char, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::open_tree, dfd, filename, flags)
}

/// See [move_mount(2)](http://man7.org/linux/man-pages/man2/move_mount.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_move_mount(
    from_dfd: i32,
    from_pathname: *const c_char,
    to_dfd: i32,
    to_pathname: *const c_char,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::move_mount,
        from_dfd,
        from_pathname,
        to_dfd,
        to_pathname,
        flags
    )
}

/// See [fsopen(2)](http://man7.org/linux/man-pages/man2/fsopen.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fsopen(_fs_name: *const c_char, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fsopen, _fs_name, flags)
}

/// See [fsconfig(2)](http://man7.org/linux/man-pages/man2/fsconfig.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fsconfig(
    fd: i32,
    cmd: u32,
    _key: *const c_char,
    _value: *const c_void,
    aux: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::fsconfig, fd, cmd, _key, _value, aux)
}

/// See [fsmount(2)](http://man7.org/linux/man-pages/man2/fsmount.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fsmount(fs_fd: i32, flags: u32, attr_flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fsmount, fs_fd, flags, attr_flags)
}

/// See [fspick(2)](http://man7.org/linux/man-pages/man2/fspick.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fspick(dfd: i32, path: *const c_char, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::fspick, dfd, path, flags)
}

/// See [pidfd_open(2)](http://man7.org/linux/man-pages/man2/pidfd_open.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pidfd_open(pid: i32, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::pidfd_open, pid, flags)
}

/// See [clone3(2)](http://man7.org/linux/man-pages/man2/clone3.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_clone3(uargs: *mut c_void, size: usize) -> Result<usize, Errno> {
    syscall!(Sysno::clone3, uargs, size)
}

/// See [close_range(2)](http://man7.org/linux/man-pages/man2/close_range.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_close_range(fd: u32, max_fd: u32, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::close_range, fd, max_fd, flags)
}

/// See [openat2(2)](http://man7.org/linux/man-pages/man2/openat2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_openat2(
    dfd: i32,
    filename: *const c_char,
    how: *mut c_void,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::openat2, dfd, filename, how, size)
}

/// See [pidfd_getfd(2)](http://man7.org/linux/man-pages/man2/pidfd_getfd.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_pidfd_getfd(pidfd: i32, fd: i32, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::pidfd_getfd, pidfd, fd, flags)
}

/// See [faccessat2(2)](http://man7.org/linux/man-pages/man2/faccessat2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_faccessat2(
    dfd: i32,
    filename: *const c_char,
    mode: i32,
    flags: i32,
) -> Result<usize, Errno> {
    syscall!(Sysno::faccessat2, dfd, filename, mode, flags)
}

/// See [process_madvise(2)](http://man7.org/linux/man-pages/man2/process_madvise.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_process_madvise(
    pidfd: i32,
    vec: *const c_void,
    vlen: usize,
    behavior: i32,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::process_madvise, pidfd, vec, vlen, behavior, flags)
}

/// See [epoll_pwait2(2)](http://man7.org/linux/man-pages/man2/epoll_pwait2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_epoll_pwait2(
    epfd: i32,
    events: *mut c_void,
    maxevents: i32,
    timeout: *const c_void,
    sigmask: *const c_void,
    sigsetsize: usize,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::epoll_pwait2,
        epfd,
        events,
        maxevents,
        timeout,
        sigmask,
        sigsetsize
    )
}

/// See [mount_setattr(2)](http://man7.org/linux/man-pages/man2/mount_setattr.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mount_setattr(
    dfd: i32,
    path: *const c_char,
    flags: u32,
    uattr: *mut c_void,
    size: usize,
) -> Result<usize, Errno> {
    syscall!(Sysno::mount_setattr, dfd, path, flags, uattr, size)
}

/// See [landlock_create_ruleset(2)](http://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_landlock_create_ruleset(
    attr: *const c_void,
    size: usize,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::landlock_create_ruleset, attr, size, flags)
}

/// See [landlock_add_rule(2)](http://man7.org/linux/man-pages/man2/landlock_add_rule.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_landlock_add_rule(
    ruleset_fd: i32,
    rule_type: u32,
    rule_attr: *const c_void,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::landlock_add_rule,
        ruleset_fd,
        rule_type,
        rule_attr,
        flags
    )
}

/// See [landlock_restrict_self(2)](http://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_landlock_restrict_self(ruleset_fd: i32, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::landlock_restrict_self, ruleset_fd, flags)
}

/// See [memfd_secret(2)](http://man7.org/linux/man-pages/man2/memfd_secret.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_memfd_secret(flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::memfd_secret, flags)
}

/// See [process_mrelease(2)](http://man7.org/linux/man-pages/man2/process_mrelease.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_process_mrelease(pidfd: i32, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::process_mrelease, pidfd, flags)
}

/// See [futex_waitv(2)](http://man7.org/linux/man-pages/man2/futex_waitv.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_futex_waitv(
    waiters: *mut c_void,
    nr_futexes: u32,
    flags: u32,
    timeout: *mut c_void,
    clockid: i32,
) -> Result<usize, Errno> {
    syscall!(
        Sysno::futex_waitv,
        waiters,
        nr_futexes,
        flags,
        timeout,
        clockid
    )
}

/// See [cachestat(2)](http://man7.org/linux/man-pages/man2/cachestat.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_cachestat(
    fd: u32,
    cstat_range: *mut c_void,
    cstat: *mut c_void,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::cachestat, fd, cstat_range, cstat, flags)
}

/// See [fchmodat2(2)](http://man7.org/linux/man-pages/man2/fchmodat2.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_fchmodat2(
    dfd: i32,
    filename: *const c_char,
    mode: u32,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::fchmodat2, dfd, filename, mode, flags)
}

/// See [mseal(2)](http://man7.org/linux/man-pages/man2/mseal.2.html)
/// for more info on this syscall.
#[inline(always)]
pub unsafe fn sys_mseal(start: u64, len: usize, flags: u64) -> Result<usize, Errno> {
    syscall!(Sysno::mseal, start, len, flags)
}
