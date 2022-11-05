#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

mod help;
mod id_map;
mod ssh_opt;
mod ssh;

use ::libsshfs::*;
use libfuse_sys::fuse::{fuse_opt, fuse_args, fuse_file_info, fuse_opt_free_args, fuse_opt_proc_t};
use libc::time_t;
use std::ffi::{CString, CStr};


extern "C" {
    pub type fuse_session;
    pub type fuse_pollhandle;
    pub type fuse;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _GHashTable;
    fn fuse_daemonize(foreground: libc::c_int) -> libc::c_int;
    fn fuse_pkgversion() -> *const libc::c_char;
    fn fuse_set_signal_handlers(se: *mut fuse_session) -> libc::c_int;
    fn fuse_remove_signal_handlers(se: *mut fuse_session);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fuse_opt_insert_arg(
        args: *mut fuse_args,
        pos: libc::c_int,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn fuse_opt_add_arg(args: *mut fuse_args, arg: *const libc::c_char) -> libc::c_int;
    fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut libc::c_void,
        opts: *const fuse_opt,
        proc_0: fuse_opt_proc_t,
    ) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn fuse_lib_help(args: *mut fuse_args);
    fn fuse_new(
        args: *mut fuse_args,
        op: *const fuse_operations,
        op_size: size_t,
        private_data: *mut libc::c_void,
    ) -> *mut fuse;
    fn fuse_mount(f: *mut fuse, mountpoint: *const libc::c_char) -> libc::c_int;
    fn fuse_unmount(f: *mut fuse);
    fn fuse_destroy(f: *mut fuse);
    fn fuse_loop(f: *mut fuse) -> libc::c_int;
    fn fuse_loop_mt_31(f: *mut fuse, clone_fd: libc::c_int) -> libc::c_int;
    fn fuse_get_session(f: *mut fuse) -> *mut fuse_session;
    fn fuse_lowlevel_version();
    fn fuse_session_fd(se: *mut fuse_session) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn grantpt(__fd: libc::c_int) -> libc::c_int;
    fn unlockpt(__fd: libc::c_int) -> libc::c_int;
    fn ptsname(__fd: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn fork() -> __pid_t;
    fn getpagesize() -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mlock(__addr: *const libc::c_void, __len: size_t) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_first(list: *mut GList) -> *mut GList;
    fn g_hash_table_new(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
    ) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_lookup_extended(
        hash_table: *mut GHashTable,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_foreach_remove(
        hash_table: *mut GHashTable,
        func: GHRFunc,
        user_data: gpointer,
    ) -> guint;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn cache_wrap(oper: *mut fuse_operations) -> *mut fuse_operations;
    fn cache_parse_options(args: *mut fuse_args) -> libc::c_int;
    fn cache_add_attr(path: *const libc::c_char, stbuf: *const stat, wrctr: u64);
    fn cache_invalidate(path: *const libc::c_char);
    fn cache_get_write_ctr() -> u64;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uintptr_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_conn_info {
    pub proto_major: libc::c_uint,
    pub proto_minor: libc::c_uint,
    pub max_write: libc::c_uint,
    pub max_read: libc::c_uint,
    pub max_readahead: libc::c_uint,
    pub capable: libc::c_uint,
    pub want: libc::c_uint,
    pub max_background: libc::c_uint,
    pub congestion_threshold: libc::c_uint,
    pub time_gran: libc::c_uint,
    pub reserved: [libc::c_uint; 22],
}
pub type fuse_buf_flags = libc::c_uint;
pub const FUSE_BUF_FD_RETRY: fuse_buf_flags = 8;
pub const FUSE_BUF_FD_SEEK: fuse_buf_flags = 4;
pub const FUSE_BUF_IS_FD: fuse_buf_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_buf {
    pub size: size_t,
    pub flags: fuse_buf_flags,
    pub mem: *mut libc::c_void,
    pub fd: libc::c_int,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [fuse_buf; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type fuse_readdir_flags = libc::c_uint;
pub const FUSE_READDIR_PLUS: fuse_readdir_flags = 1;
pub type fuse_fill_dir_flags = libc::c_uint;
pub const FUSE_FILL_DIR_PLUS: fuse_fill_dir_flags = 2;
pub type fuse_fill_dir_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const stat,
        off_t,
        fuse_fill_dir_flags,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_config {
    pub set_gid: libc::c_int,
    pub gid: libc::c_uint,
    pub set_uid: libc::c_int,
    pub uid: libc::c_uint,
    pub set_mode: libc::c_int,
    pub umask: libc::c_uint,
    pub entry_timeout: libc::c_double,
    pub negative_timeout: libc::c_double,
    pub attr_timeout: libc::c_double,
    pub intr: libc::c_int,
    pub intr_signal: libc::c_int,
    pub remember: libc::c_int,
    pub hard_remove: libc::c_int,
    pub use_ino: libc::c_int,
    pub readdir_ino: libc::c_int,
    pub direct_io: libc::c_int,
    pub kernel_cache: libc::c_int,
    pub auto_cache: libc::c_int,
    pub no_rofd_flush: libc::c_int,
    pub ac_attr_timeout_set: libc::c_int,
    pub ac_attr_timeout: libc::c_double,
    pub nullpath_ok: libc::c_int,
    pub show_help: libc::c_int,
    pub modules: *mut libc::c_char,
    pub debug: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut stat,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub readlink: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub mknod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t, dev_t) -> libc::c_int,
    >,
    pub mkdir: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub unlink: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub rmdir: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub symlink: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub rename: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub link: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub chmod: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub chown: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            uid_t,
            gid_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub truncate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub open: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub statfs: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut statvfs) -> libc::c_int,
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub release: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsync: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub setxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub getxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub listxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub removexattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub readdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_void,
            fuse_fill_dir_t,
            off_t,
            *mut fuse_file_info,
            fuse_readdir_flags,
        ) -> libc::c_int,
    >,
    pub releasedir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsyncdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*mut fuse_conn_info, *mut fuse_config) -> *mut libc::c_void,
    >,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub access: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub create: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub lock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
            *mut flock,
        ) -> libc::c_int,
    >,
    pub utimens: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const timespec,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub bmap: Option::<
        unsafe extern "C" fn(*const libc::c_char, size_t, *mut u64) -> libc::c_int,
    >,
    pub ioctl: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            *mut fuse_file_info,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub poll: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            *mut fuse_pollhandle,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_bufvec,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub read_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut *mut fuse_bufvec,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub flock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub fallocate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            off_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub copy_file_range: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            off_t,
            *const libc::c_char,
            *mut fuse_file_info,
            off_t,
            size_t,
            libc::c_int,
        ) -> ssize_t,
    >,
    pub lseek: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            libc::c_int,
            *mut fuse_file_info,
        ) -> off_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
pub type GHRFunc = Option::<
    unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn {
    pub lock_write: pthread_mutex_t,
    pub processing_thread_started: libc::c_int,
    pub rfd: libc::c_int,
    pub wfd: libc::c_int,
    pub connver: libc::c_int,
    pub req_count: libc::c_int,
    pub dir_count: libc::c_int,
    pub file_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub p: *mut u8,
    pub len: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir_handle {
    pub buf: buffer,
    pub conn: *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request {
    pub want_reply: libc::c_uint,
    pub ready: sem_t,
    pub reply_type: u8,
    pub id: u32,
    pub replied: libc::c_int,
    pub error: libc::c_int,
    pub reply: buffer,
    pub start: timeval,
    pub data: *mut libc::c_void,
    pub end_func: request_func,
    pub len: size_t,
    pub list: list_head,
    pub conn: *mut conn,
}
pub type request_func = Option::<unsafe extern "C" fn(*mut request) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs_io {
    pub num_reqs: libc::c_int,
    pub finished: pthread_cond_t,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_req {
    pub sio: *mut sshfs_io,
    pub list: list_head,
    pub data: buffer,
    pub size: size_t,
    pub res: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_chunk {
    pub offset: off_t,
    pub size: size_t,
    pub refs: libc::c_int,
    pub modifver: libc::c_long,
    pub reqs: list_head,
    pub sio: sshfs_io,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs_file {
    pub handle: buffer,
    pub write_reqs: list_head,
    pub write_finished: pthread_cond_t,
    pub write_error: libc::c_int,
    pub readahead: *mut read_chunk,
    pub next_pos: off_t,
    pub is_seq: libc::c_int,
    pub conn: *mut conn,
    pub connver: libc::c_int,
    pub modifver: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conntab_entry {
    pub refcount: libc::c_uint,
    pub conn: *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs {
    pub directport: *mut libc::c_char,
    pub ssh_command: *mut libc::c_char,
    pub sftp_server: *mut libc::c_char,
    pub ssh_args: fuse_args,
    pub workarounds: *mut libc::c_char,
    pub rename_workaround: libc::c_int,
    pub renamexdev_workaround: libc::c_int,
    pub truncate_workaround: libc::c_int,
    pub buflimit_workaround: libc::c_int,
    pub unrel_append: libc::c_int,
    pub fstat_workaround: libc::c_int,
    pub createmode_workaround: libc::c_int,
    pub transform_symlinks: libc::c_int,
    pub follow_symlinks: libc::c_int,
    pub no_check_root: libc::c_int,
    pub detect_uid: libc::c_int,
    pub idmap: libc::c_int,
    pub nomap: libc::c_int,
    pub disable_hardlink: libc::c_int,
    pub dir_cache: libc::c_int,
    pub show_version: libc::c_int,
    pub show_help: libc::c_int,
    pub singlethread: libc::c_int,
    pub mountpoint: *mut libc::c_char,
    pub uid_file: *mut libc::c_char,
    pub gid_file: *mut libc::c_char,
    pub uid_map: *mut GHashTable,
    pub gid_map: *mut GHashTable,
    pub r_uid_map: *mut GHashTable,
    pub r_gid_map: *mut GHashTable,
    pub max_read: libc::c_uint,
    pub max_write: libc::c_uint,
    pub ssh_ver: libc::c_uint,
    pub sync_write: libc::c_int,
    pub sync_read: libc::c_int,
    pub sync_readdir: libc::c_int,
    pub direct_io: libc::c_int,
    pub debug: libc::c_int,
    pub verbose: libc::c_int,
    pub foreground: libc::c_int,
    pub reconnect: libc::c_int,
    pub delay_connect: libc::c_int,
    pub passive: libc::c_int,
    pub host: *mut libc::c_char,
    pub base_path: *mut libc::c_char,
    pub reqtab: *mut GHashTable,
    pub conntab: *mut GHashTable,
    pub lock: pthread_mutex_t,
    pub randseed: libc::c_uint,
    pub max_conns: libc::c_int,
    pub conns: *mut conn,
    pub ptyfd: libc::c_int,
    pub ptypassivefd: libc::c_int,
    pub connvers: libc::c_int,
    pub server_version: libc::c_int,
    pub remote_uid: libc::c_uint,
    pub local_uid: libc::c_uint,
    pub remote_gid: libc::c_uint,
    pub local_gid: libc::c_uint,
    pub remote_uid_detected: libc::c_int,
    pub blksize: libc::c_uint,
    pub progname: *mut libc::c_char,
    pub modifver: libc::c_long,
    pub outstanding_len: libc::c_uint,
    pub max_outstanding_len: libc::c_uint,
    pub outstanding_cond: pthread_cond_t,
    pub password_stdin: libc::c_int,
    pub password: *mut libc::c_char,
    pub ext_posix_rename: libc::c_int,
    pub ext_statvfs: libc::c_int,
    pub ext_hardlink: libc::c_int,
    pub ext_fsync: libc::c_int,
    pub op: *mut fuse_operations,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub num_sent: u64,
    pub num_received: u64,
    pub min_rtt: libc::c_uint,
    pub max_rtt: libc::c_uint,
    pub total_rtt: u64,
    pub num_connect: libc::c_uint,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const KEY_CONFIGFILE: C2RustUnnamed_2 = 2;
pub const KEY_COMPRESS: C2RustUnnamed_2 = 1;
pub const KEY_PORT: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IDMAP_FILE: C2RustUnnamed_3 = 2;
pub const IDMAP_USER: C2RustUnnamed_3 = 1;
pub const IDMAP_NONE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NOMAP_ERROR: C2RustUnnamed_4 = 1;
pub const NOMAP_IGNORE: C2RustUnnamed_4 = 0;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: u32) -> u32 {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut sshfs: sshfs = sshfs {
    directport: 0 as *const libc::c_char as *mut libc::c_char,
    ssh_command: 0 as *const libc::c_char as *mut libc::c_char,
    sftp_server: 0 as *const libc::c_char as *mut libc::c_char,
    ssh_args: fuse_args {
        argc: 0,
        argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        allocated: 0,
    },
    workarounds: 0 as *const libc::c_char as *mut libc::c_char,
    rename_workaround: 0,
    renamexdev_workaround: 0,
    truncate_workaround: 0,
    buflimit_workaround: 0,
    unrel_append: 0,
    fstat_workaround: 0,
    createmode_workaround: 0,
    transform_symlinks: 0,
    follow_symlinks: 0,
    no_check_root: 0,
    detect_uid: 0,
    idmap: 0,
    nomap: 0,
    disable_hardlink: 0,
    dir_cache: 0,
    show_version: 0,
    show_help: 0,
    singlethread: 0,
    mountpoint: 0 as *const libc::c_char as *mut libc::c_char,
    uid_file: 0 as *const libc::c_char as *mut libc::c_char,
    gid_file: 0 as *const libc::c_char as *mut libc::c_char,
    uid_map: 0 as *const GHashTable as *mut GHashTable,
    gid_map: 0 as *const GHashTable as *mut GHashTable,
    r_uid_map: 0 as *const GHashTable as *mut GHashTable,
    r_gid_map: 0 as *const GHashTable as *mut GHashTable,
    max_read: 0,
    max_write: 0,
    ssh_ver: 0,
    sync_write: 0,
    sync_read: 0,
    sync_readdir: 0,
    direct_io: 0,
    debug: 0,
    verbose: 0,
    foreground: 0,
    reconnect: 0,
    delay_connect: 0,
    passive: 0,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    base_path: 0 as *const libc::c_char as *mut libc::c_char,
    reqtab: 0 as *const GHashTable as *mut GHashTable,
    conntab: 0 as *const GHashTable as *mut GHashTable,
    lock: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    randseed: 0,
    max_conns: 0,
    conns: 0 as *const conn as *mut conn,
    ptyfd: 0,
    ptypassivefd: 0,
    connvers: 0,
    server_version: 0,
    remote_uid: 0,
    local_uid: 0,
    remote_gid: 0,
    local_gid: 0,
    remote_uid_detected: 0,
    blksize: 0,
    progname: 0 as *const libc::c_char as *mut libc::c_char,
    modifver: 0,
    outstanding_len: 0,
    max_outstanding_len: 0,
    outstanding_cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter {
                __value64: 0,
            },
            __g1_start: __atomic_wide_counter {
                __value64: 0,
            },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    },
    password_stdin: 0,
    password: 0 as *const libc::c_char as *mut libc::c_char,
    ext_posix_rename: 0,
    ext_statvfs: 0,
    ext_hardlink: 0,
    ext_fsync: 0,
    op: 0 as *const fuse_operations as *mut fuse_operations,
    bytes_sent: 0,
    bytes_received: 0,
    num_sent: 0,
    num_received: 0,
    min_rtt: 0,
    max_rtt: 0,
    total_rtt: 0,
    num_connect: 0,
};
static mut sshfs_opts: [fuse_opt; 57] = [fuse_opt {
    templ: 0 as *const libc::c_char,
    offset: 0,
    value: 0,
}; 57];
static mut workaround_opts: [fuse_opt; 17] = [
    {
        let mut init = fuse_opt {
            templ: b"none\0" as *const u8 as *const libc::c_char,
            offset: 56 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"none\0" as *const u8 as *const libc::c_char,
            offset: 64 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"none\0" as *const u8 as *const libc::c_char,
            offset: 68 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"none\0" as *const u8 as *const libc::c_char,
            offset: 76 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"rename\0" as *const u8 as *const libc::c_char,
            offset: 56 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"norename\0" as *const u8 as *const libc::c_char,
            offset: 56 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"renamexdev\0" as *const u8 as *const libc::c_char,
            offset: 60 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"norenamexdev\0" as *const u8 as *const libc::c_char,
            offset: 60 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"truncate\0" as *const u8 as *const libc::c_char,
            offset: 64 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"notruncate\0" as *const u8 as *const libc::c_char,
            offset: 64 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"buflimit\0" as *const u8 as *const libc::c_char,
            offset: 68 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"nobuflimit\0" as *const u8 as *const libc::c_char,
            offset: 68 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"fstat\0" as *const u8 as *const libc::c_char,
            offset: 76 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"nofstat\0" as *const u8 as *const libc::c_char,
            offset: 76 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"createmode\0" as *const u8 as *const libc::c_char,
            offset: 80 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"nocreatemode\0" as *const u8 as *const libc::c_char,
            offset: 80 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: 0 as *const libc::c_char,
            offset: 0 as libc::c_int as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn type_name(mut type_0: u8) -> *const libc::c_char {
    match type_0 as libc::c_int {
        1 => return b"INIT\0" as *const u8 as *const libc::c_char,
        2 => return b"VERSION\0" as *const u8 as *const libc::c_char,
        3 => return b"OPEN\0" as *const u8 as *const libc::c_char,
        4 => return b"CLOSE\0" as *const u8 as *const libc::c_char,
        5 => return b"READ\0" as *const u8 as *const libc::c_char,
        6 => return b"WRITE\0" as *const u8 as *const libc::c_char,
        7 => return b"LSTAT\0" as *const u8 as *const libc::c_char,
        8 => return b"FSTAT\0" as *const u8 as *const libc::c_char,
        9 => return b"SETSTAT\0" as *const u8 as *const libc::c_char,
        10 => return b"FSETSTAT\0" as *const u8 as *const libc::c_char,
        11 => return b"OPENDIR\0" as *const u8 as *const libc::c_char,
        12 => return b"READDIR\0" as *const u8 as *const libc::c_char,
        13 => return b"REMOVE\0" as *const u8 as *const libc::c_char,
        14 => return b"MKDIR\0" as *const u8 as *const libc::c_char,
        15 => return b"RMDIR\0" as *const u8 as *const libc::c_char,
        16 => return b"REALPATH\0" as *const u8 as *const libc::c_char,
        17 => return b"STAT\0" as *const u8 as *const libc::c_char,
        18 => return b"RENAME\0" as *const u8 as *const libc::c_char,
        19 => return b"READLINK\0" as *const u8 as *const libc::c_char,
        20 => return b"SYMLINK\0" as *const u8 as *const libc::c_char,
        101 => return b"STATUS\0" as *const u8 as *const libc::c_char,
        102 => return b"HANDLE\0" as *const u8 as *const libc::c_char,
        103 => return b"DATA\0" as *const u8 as *const libc::c_char,
        104 => return b"NAME\0" as *const u8 as *const libc::c_char,
        105 => return b"ATTRS\0" as *const u8 as *const libc::c_char,
        200 => return b"EXTENDED\0" as *const u8 as *const libc::c_char,
        201 => return b"EXTENDED_REPLY\0" as *const u8 as *const libc::c_char,
        _ => return b"???\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    let ref mut fresh0 = (*head).next;
    *fresh0 = head;
    let ref mut fresh1 = (*head).prev;
    *fresh1 = head;
}
unsafe extern "C" fn list_add(mut new: *mut list_head, mut head: *mut list_head) {
    let mut prev: *mut list_head = head;
    let mut next: *mut list_head = (*head).next;
    let ref mut fresh2 = (*next).prev;
    *fresh2 = new;
    let ref mut fresh3 = (*new).next;
    *fresh3 = next;
    let ref mut fresh4 = (*new).prev;
    *fresh4 = prev;
    let ref mut fresh5 = (*prev).next;
    *fresh5 = new;
}
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    let mut prev: *mut list_head = (*entry).prev;
    let mut next: *mut list_head = (*entry).next;
    let ref mut fresh6 = (*next).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = next;
}
unsafe extern "C" fn list_empty(mut head: *const list_head) -> libc::c_int {
    return ((*head).next == head as *mut list_head) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_init(mut buf: *mut buffer, mut size: size_t) {
    if size != 0 {
        let ref mut fresh8 = (*buf).p;
        *fresh8 = malloc(size) as *mut u8;
        if ((*buf).p).is_null() {
            fprintf(
                stderr,
                b"sshfs: memory allocation failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    } else {
        let ref mut fresh9 = (*buf).p;
        *fresh9 = 0 as *mut u8;
    }
    (*buf).len = 0 as libc::c_int as size_t;
    (*buf).size = size;
}
#[inline]
unsafe extern "C" fn buf_free(mut buf: *mut buffer) {
    free((*buf).p as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn buf_finish(mut buf: *mut buffer) {
    (*buf).len = (*buf).size;
}
#[inline]
unsafe extern "C" fn buf_clear(mut buf: *mut buffer) {
    buf_free(buf);
    buf_init(buf, 0 as libc::c_int as size_t);
}
unsafe extern "C" fn buf_resize(mut buf: *mut buffer, mut len: size_t) {
    (*buf)
        .size = ((*buf).len)
        .wrapping_add(len)
        .wrapping_add(63 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int) as libc::c_ulong;
    let ref mut fresh10 = (*buf).p;
    *fresh10 = realloc((*buf).p as *mut libc::c_void, (*buf).size) as *mut u8;
    if ((*buf).p).is_null() {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
}
#[inline]
unsafe extern "C" fn buf_check_add(mut buf: *mut buffer, mut len: size_t) {
    if ((*buf).len).wrapping_add(len) > (*buf).size {
        buf_resize(buf, len);
    }
}
#[inline]
unsafe extern "C" fn buf_add_mem(
    mut buf: *mut buffer,
    mut data: *const libc::c_void,
    mut len: size_t,
) {
    buf_check_add(buf, len);
    memcpy(((*buf).p).offset((*buf).len as isize) as *mut libc::c_void, data, len);
    let ref mut fresh11 = (*buf).len;
    *fresh11 = (*fresh11 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_buf(mut buf: *mut buffer, mut bufa: *const buffer) {
    buf_check_add(buf, (*bufa).len);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        (*bufa).p as *const libc::c_void,
        (*bufa).len,
    );
    let ref mut fresh12 = (*buf).len;
    *fresh12 = (*fresh12 as libc::c_ulong).wrapping_add((*bufa).len) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_uint8(mut buf: *mut buffer, mut val: u8) {
    buf_check_add(buf, 1 as libc::c_int as size_t);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        &mut val as *mut u8 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    let ref mut fresh13 = (*buf).len;
    *fresh13 = (*fresh13 as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_uint32(mut buf: *mut buffer, mut val: u32) {
    let mut nval: u32 = __bswap_32(val);
    buf_check_add(buf, 4 as libc::c_int as size_t);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        &mut nval as *mut u32 as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    let ref mut fresh14 = (*buf).len;
    *fresh14 = (*fresh14 as libc::c_ulong)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_uint64(mut buf: *mut buffer, mut val: u64) {
    buf_add_uint32(buf, (val >> 32 as libc::c_int) as u32);
    buf_add_uint32(buf, (val & 0xffffffff as libc::c_uint as libc::c_ulong) as u32);
}
#[inline]
unsafe extern "C" fn buf_add_data(mut buf: *mut buffer, mut data: *const buffer) {
    buf_add_uint32(buf, (*data).len as u32);
    buf_add_mem(buf, (*data).p as *const libc::c_void, (*data).len);
}
#[inline]
unsafe extern "C" fn buf_add_string(mut buf: *mut buffer, mut str: *const libc::c_char) {
    let mut data: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    data.p = str as *mut u8;
    data.len = strlen(str);
    buf_add_data(buf, &mut data);
}
#[inline]
unsafe extern "C" fn buf_add_path(mut buf: *mut buffer, mut path: *const libc::c_char) {
    let mut realpath_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if *(sshfs.base_path).offset(0 as libc::c_int as isize) != 0 {
        if *path.offset(1 as libc::c_int as isize) != 0 {
            if *(sshfs.base_path)
                .offset(
                    (strlen(sshfs.base_path))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != '/' as i32
            {
                realpath_0 = g_strdup_printf(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    sshfs.base_path,
                    path.offset(1 as libc::c_int as isize),
                );
            } else {
                realpath_0 = g_strdup_printf(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    sshfs.base_path,
                    path.offset(1 as libc::c_int as isize),
                );
            }
        } else {
            realpath_0 = g_strdup(sshfs.base_path);
        }
    } else if *path.offset(1 as libc::c_int as isize) != 0 {
        realpath_0 = g_strdup(path.offset(1 as libc::c_int as isize));
    } else {
        realpath_0 = g_strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    buf_add_string(buf, realpath_0);
    g_free(realpath_0 as gpointer);
}
unsafe extern "C" fn buf_check_get(
    mut buf: *mut buffer,
    mut len: size_t,
) -> libc::c_int {
    if ((*buf).len).wrapping_add(len) > (*buf).size {
        fprintf(stderr, b"buffer too short\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn buf_get_mem(
    mut buf: *mut buffer,
    mut data: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    if buf_check_get(buf, len) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memcpy(data, ((*buf).p).offset((*buf).len as isize) as *const libc::c_void, len);
    let ref mut fresh15 = (*buf).len;
    *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_uint8(
    mut buf: *mut buffer,
    mut val: *mut u8,
) -> libc::c_int {
    return buf_get_mem(buf, val as *mut libc::c_void, 1 as libc::c_int as size_t);
}
#[inline]
unsafe extern "C" fn buf_get_uint32(
    mut buf: *mut buffer,
    mut val: *mut u32,
) -> libc::c_int {
    let mut nval: u32 = 0;
    if buf_get_mem(
        buf,
        &mut nval as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    *val = __bswap_32(nval);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_uint64(
    mut buf: *mut buffer,
    mut val: *mut u64,
) -> libc::c_int {
    let mut val1: u32 = 0;
    let mut val2: u32 = 0;
    if buf_get_uint32(buf, &mut val1) == -(1 as libc::c_int)
        || buf_get_uint32(buf, &mut val2) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    *val = ((val1 as u64) << 32 as libc::c_int).wrapping_add(val2 as libc::c_ulong);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_data(
    mut buf: *mut buffer,
    mut data: *mut buffer,
) -> libc::c_int {
    let mut len: u32 = 0;
    if buf_get_uint32(buf, &mut len) == -(1 as libc::c_int)
        || len as libc::c_ulong > ((*buf).size).wrapping_sub((*buf).len)
    {
        return -(1 as libc::c_int);
    }
    buf_init(data, len.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t);
    (*data).size = len as size_t;
    if buf_get_mem(buf, (*data).p as *mut libc::c_void, (*data).size)
        == -(1 as libc::c_int)
    {
        buf_free(data);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_string(
    mut buf: *mut buffer,
    mut str: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut data: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    if buf_get_data(buf, &mut data) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    *(data.p).offset(data.size as isize) = '\0' as i32 as u8;
    *str = data.p as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_attrs(
    mut buf: *mut buffer,
    mut stbuf: *mut stat,
    mut flagsp: *mut libc::c_int,
) -> libc::c_int {
    let mut flags: u32 = 0;
    let mut size: u64 = 0 as libc::c_int as u64;
    let mut uid: u32 = 0 as libc::c_int as u32;
    let mut gid: u32 = 0 as libc::c_int as u32;
    let mut atime: u32 = 0 as libc::c_int as u32;
    let mut mtime: u32 = 0 as libc::c_int as u32;
    let mut mode: u32 = (0o100000 as libc::c_int | 0o777 as libc::c_int)
        as u32;
    if buf_get_uint32(buf, &mut flags) == -(1 as libc::c_int) {
        return -(5 as libc::c_int);
    }
    if !flagsp.is_null() {
        *flagsp = flags as libc::c_int;
    }
    if flags & 0x1 as libc::c_int as libc::c_uint != 0
        && buf_get_uint64(buf, &mut size) == -(1 as libc::c_int)
    {
        return -(5 as libc::c_int);
    }
    if flags & 0x2 as libc::c_int as libc::c_uint != 0
        && (buf_get_uint32(buf, &mut uid) == -(1 as libc::c_int)
            || buf_get_uint32(buf, &mut gid) == -(1 as libc::c_int))
    {
        return -(5 as libc::c_int);
    }
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && buf_get_uint32(buf, &mut mode) == -(1 as libc::c_int)
    {
        return -(5 as libc::c_int);
    }
    if flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        if buf_get_uint32(buf, &mut atime) == -(1 as libc::c_int)
            || buf_get_uint32(buf, &mut mtime) == -(1 as libc::c_int)
        {
            return -(5 as libc::c_int);
        }
    }
    if flags & 0x80000000 as libc::c_uint != 0 {
        let mut extcount: u32 = 0;
        let mut i: libc::c_uint = 0;
        if buf_get_uint32(buf, &mut extcount) == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < extcount {
            let mut tmp: buffer = buffer {
                p: 0 as *mut u8,
                len: 0,
                size: 0,
            };
            if buf_get_data(buf, &mut tmp) == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
            buf_free(&mut tmp);
            if buf_get_data(buf, &mut tmp) == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
            buf_free(&mut tmp);
            i = i.wrapping_add(1);
        }
    }
    if sshfs.remote_uid_detected != 0 {
        if uid == sshfs.remote_uid {
            uid = sshfs.local_uid;
        }
        if gid == sshfs.remote_gid {
            gid = sshfs.local_gid;
        }
    }
    if sshfs.idmap == IDMAP_FILE as libc::c_int && !(sshfs.uid_map).is_null() {
        if id_map::translate_id(&mut uid, "uid", sshfs.nomap) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if sshfs.idmap == IDMAP_FILE as libc::c_int && !(sshfs.gid_map).is_null() {
        if id_map::translate_id(&mut gid, "gid", sshfs.nomap) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    memset(
        stbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
    );
    (*stbuf).st_mode = mode;
    (*stbuf).st_nlink = 1 as libc::c_int as __nlink_t;
    (*stbuf).st_size = size as __off_t;
    if sshfs.blksize != 0 {
        (*stbuf).st_blksize = sshfs.blksize as __blksize_t;
        (*stbuf)
            .st_blocks = ((size
            .wrapping_add(sshfs.blksize as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong
            & !(sshfs.blksize as libc::c_ulonglong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong))
            >> 9 as libc::c_int) as __blkcnt_t;
    }
    (*stbuf).st_uid = uid;
    (*stbuf).st_gid = gid;
    (*stbuf).st_atim.tv_sec = atime as time_t;
    let ref mut fresh16 = (*stbuf).st_mtim.tv_sec;
    *fresh16 = mtime as time_t;
    (*stbuf).st_ctim.tv_sec = *fresh16;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_statvfs(
    mut buf: *mut buffer,
    mut stbuf: *mut statvfs,
) -> libc::c_int {
    let mut bsize: u64 = 0;
    let mut frsize: u64 = 0;
    let mut blocks: u64 = 0;
    let mut bfree: u64 = 0;
    let mut bavail: u64 = 0;
    let mut files: u64 = 0;
    let mut ffree: u64 = 0;
    let mut favail: u64 = 0;
    let mut fsid: u64 = 0;
    let mut flag: u64 = 0;
    let mut namemax: u64 = 0;
    if buf_get_uint64(buf, &mut bsize) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut frsize) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut blocks) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut bfree) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut bavail) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut files) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut ffree) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut favail) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut fsid) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut flag) == -(1 as libc::c_int)
        || buf_get_uint64(buf, &mut namemax) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    memset(
        stbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<statvfs>() as libc::c_ulong,
    );
    (*stbuf).f_bsize = bsize;
    (*stbuf).f_frsize = frsize;
    (*stbuf).f_blocks = blocks;
    (*stbuf).f_bfree = bfree;
    (*stbuf).f_bavail = bavail;
    (*stbuf).f_files = files;
    (*stbuf).f_ffree = ffree;
    (*stbuf).f_favail = favail;
    (*stbuf).f_namemax = namemax;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_entries(
    mut buf: *mut buffer,
    mut dbuf: *mut libc::c_void,
    mut filler: fuse_fill_dir_t,
) -> libc::c_int {
    let mut count: u32 = 0;
    let mut i: libc::c_uint = 0;
    if buf_get_uint32(buf, &mut count) == -(1 as libc::c_int) {
        return -(5 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < count {
        let mut err: libc::c_int = -(1 as libc::c_int);
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut longname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut stbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if buf_get_string(buf, &mut name) == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
        if buf_get_string(buf, &mut longname) != -(1 as libc::c_int) {
            free(longname as *mut libc::c_void);
            err = buf_get_attrs(buf, &mut stbuf, 0 as *mut libc::c_int);
            if err == 0 {
                if sshfs.follow_symlinks != 0
                    && stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                {
                    stbuf.st_mode = 0 as libc::c_int as __mode_t;
                }
                filler
                    .expect(
                        "non-null function pointer",
                    )(
                    dbuf,
                    name,
                    &mut stbuf,
                    0 as libc::c_int as off_t,
                    0 as fuse_fill_dir_flags,
                );
            }
        }
        free(name as *mut libc::c_void);
        if err != 0 {
            return err;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ssh_add_arg(mut arg: *const libc::c_char) {
    if fuse_opt_add_arg(&mut sshfs.ssh_args, arg) == -(1 as libc::c_int) {
        _exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn pty_expect_loop(mut conn: *mut conn) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut passwd_str: *const libc::c_char = b"assword:\0" as *const u8
        as *const libc::c_char;
    let mut timeout: libc::c_int = 60 as libc::c_int * 1000 as libc::c_int;
    let mut passwd_len: libc::c_int = strlen(passwd_str) as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    loop {
        let mut fds: [pollfd; 2] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 2];
        fds[0 as libc::c_int as usize].fd = (*conn).rfd;
        fds[0 as libc::c_int as usize].events = 0x1 as libc::c_int as libc::c_short;
        fds[1 as libc::c_int as usize].fd = sshfs.ptyfd;
        fds[1 as libc::c_int as usize].events = 0x1 as libc::c_int as libc::c_short;
        res = poll(fds.as_mut_ptr(), 2 as libc::c_int as nfds_t, timeout);
        if res == -(1 as libc::c_int) {
            perror(b"poll\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            fprintf(
                stderr,
                b"Timeout waiting for prompt\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if fds[0 as libc::c_int as usize].revents != 0 {
            break;
        }
        res = read(
            sshfs.ptyfd,
            &mut c as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            fprintf(
                stderr,
                b"EOF while waiting for prompt\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        buf[len as usize] = c;
        len += 1;
        if len == passwd_len {
            if memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                passwd_str as *const libc::c_void,
                passwd_len as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                write(
                    sshfs.ptyfd,
                    sshfs.password as *const libc::c_void,
                    strlen(sshfs.password),
                );
            }
            memmove(
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (passwd_len - 1 as libc::c_int) as libc::c_ulong,
            );
            len -= 1;
        }
    }
    if sshfs.reconnect == 0 {
        let mut size: size_t = getpagesize() as size_t;
        memset(sshfs.password as *mut libc::c_void, 0 as libc::c_int, size);
        munmap(sshfs.password as *mut libc::c_void, size);
        sshfs.password = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_conn(
    mut sf: *const sshfs_file,
    mut path: *const libc::c_char,
) -> *mut conn {
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut i: libc::c_int = 0;
    if sshfs.max_conns == 1 as libc::c_int {
        return &mut *(sshfs.conns).offset(0 as libc::c_int as isize) as *mut conn;
    }
    if !sf.is_null() {
        return (*sf).conn;
    }
    if !path.is_null() {
        pthread_mutex_lock(&mut sshfs.lock);
        ce = g_hash_table_lookup(sshfs.conntab, path as gconstpointer)
            as *mut conntab_entry;
        if !ce.is_null() {
            let mut conn: *mut conn = (*ce).conn;
            pthread_mutex_unlock(&mut sshfs.lock);
            return conn;
        }
        pthread_mutex_unlock(&mut sshfs.lock);
    }
    let mut best_index: libc::c_int = 0 as libc::c_int;
    let mut best_score: u64 = !(0 as libc::c_ulonglong) as u64;
    i = 0 as libc::c_int;
    while i < sshfs.max_conns {
        let mut score: u64 = (((*(sshfs.conns).offset(i as isize)).req_count
            as u64) << 43 as libc::c_int)
            .wrapping_add(
                ((*(sshfs.conns).offset(i as isize)).dir_count as u64)
                    << 22 as libc::c_int,
            )
            .wrapping_add(
                ((*(sshfs.conns).offset(i as isize)).file_count as u64)
                    << 1 as libc::c_int,
            )
            .wrapping_add(
                (if (*(sshfs.conns).offset(i as isize)).rfd >= 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as u64,
            );
        if score < best_score {
            best_index = i;
            best_score = score;
        }
        i += 1;
    }
    return &mut *(sshfs.conns).offset(best_index as isize) as *mut conn;
}
unsafe extern "C" fn pty_master(mut name: *mut *mut libc::c_char) -> libc::c_int {
    let mut mfd: libc::c_int = 0;
    mfd = open(
        b"/dev/ptmx\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int | 0o400 as libc::c_int,
    );
    if mfd == -(1 as libc::c_int) {
        perror(b"failed to open pty\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if grantpt(mfd) != 0 as libc::c_int {
        perror(b"grantpt\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if unlockpt(mfd) != 0 as libc::c_int {
        perror(b"unlockpt\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    *name = ptsname(mfd);
    return mfd;
}
unsafe extern "C" fn replace_arg(
    mut argp: *mut *mut libc::c_char,
    mut newarg: *const libc::c_char,
) {
    free(*argp as *mut libc::c_void);
    *argp = strdup(newarg);
    if (*argp).is_null() {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
}
unsafe extern "C" fn start_ssh(mut conn: *mut conn) -> libc::c_int {
    let mut ptyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sockpair: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    if sshfs.password_stdin != 0 {
        sshfs.ptyfd = pty_master(&mut ptyname);
        if sshfs.ptyfd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        sshfs.ptypassivefd = open(ptyname, 0o2 as libc::c_int | 0o400 as libc::c_int);
        if sshfs.ptypassivefd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        sockpair.as_mut_ptr(),
    ) == -(1 as libc::c_int)
    {
        perror(b"failed to create socket pair\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*conn).rfd = sockpair[0 as libc::c_int as usize];
    (*conn).wfd = sockpair[0 as libc::c_int as usize];
    pid = fork();
    if pid == -(1 as libc::c_int) {
        perror(b"failed to fork\0" as *const u8 as *const libc::c_char);
        close(sockpair[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    } else {
        if pid == 0 as libc::c_int {
            let mut devnull: libc::c_int = 0;
            devnull = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o1 as libc::c_int,
            );
            if dup2(sockpair[1 as libc::c_int as usize], 0 as libc::c_int)
                == -(1 as libc::c_int)
                || dup2(sockpair[1 as libc::c_int as usize], 1 as libc::c_int)
                    == -(1 as libc::c_int)
            {
                perror(
                    b"failed to redirect input/output\0" as *const u8
                        as *const libc::c_char,
                );
                _exit(1 as libc::c_int);
            }
            if sshfs.verbose == 0 && sshfs.foreground == 0
                && devnull != -(1 as libc::c_int)
            {
                dup2(devnull, 2 as libc::c_int);
            }
            close(devnull);
            close(sockpair[0 as libc::c_int as usize]);
            close(sockpair[1 as libc::c_int as usize]);
            match fork() {
                -1 => {
                    perror(b"failed to fork\0" as *const u8 as *const libc::c_char);
                    _exit(1 as libc::c_int);
                }
                0 => {}
                _ => {
                    _exit(0 as libc::c_int);
                }
            }
            chdir(b"/\0" as *const u8 as *const libc::c_char);
            unsetenv(b"OLDPWD\0" as *const u8 as *const libc::c_char);
            if sshfs.password_stdin != 0 {
                let mut sfd: libc::c_int = 0;
                setsid();
                sfd = open(ptyname, 0o2 as libc::c_int);
                if sfd == -(1 as libc::c_int) {
                    perror(ptyname);
                    _exit(1 as libc::c_int);
                }
                close(sfd);
                close(sshfs.ptypassivefd);
                close(sshfs.ptyfd);
            }
            if sshfs.debug != 0 {
                let mut i: libc::c_int = 0;
                fprintf(stderr, b"executing\0" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < sshfs.ssh_args.argc {
                    fprintf(
                        stderr,
                        b" <%s>\0" as *const u8 as *const libc::c_char,
                        *(sshfs.ssh_args.argv).offset(i as isize),
                    );
                    i += 1;
                }
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
            execvp(
                *(sshfs.ssh_args.argv).offset(0 as libc::c_int as isize),
                sshfs.ssh_args.argv as *const *mut libc::c_char,
            );
            fprintf(
                stderr,
                b"failed to execute '%s': %s\n\0" as *const u8 as *const libc::c_char,
                *(sshfs.ssh_args.argv).offset(0 as libc::c_int as isize),
                strerror(*__errno_location()),
            );
            _exit(1 as libc::c_int);
        }
    }
    waitpid(pid, 0 as *mut libc::c_int, 0 as libc::c_int);
    close(sockpair[1 as libc::c_int as usize]);
    return 0 as libc::c_int;
}
unsafe extern "C" fn connect_passive(mut conn: *mut conn) -> libc::c_int {
    (*conn).rfd = 0 as libc::c_int;
    (*conn).wfd = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn connect_to(
    mut conn: *mut conn,
    mut host: *mut libc::c_char,
    mut port: *mut libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hint: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    memset(
        &mut hint as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hint.ai_family = 2 as libc::c_int;
    hint.ai_socktype = SOCK_STREAM as libc::c_int;
    err = getaddrinfo(host, port, &mut hint, &mut ai);
    if err != 0 {
        fprintf(
            stderr,
            b"failed to resolve %s:%s: %s\n\0" as *const u8 as *const libc::c_char,
            host,
            port,
            gai_strerror(err),
        );
        return -(1 as libc::c_int);
    }
    sock = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
    if sock == -(1 as libc::c_int) {
        perror(b"failed to create socket\0" as *const u8 as *const libc::c_char);
        freeaddrinfo(ai);
        return -(1 as libc::c_int);
    }
    err = connect(
        sock,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*ai).ai_addr,
        },
        (*ai).ai_addrlen,
    );
    if err == -(1 as libc::c_int) {
        perror(b"failed to connect\0" as *const u8 as *const libc::c_char);
        freeaddrinfo(ai);
        close(sock);
        return -(1 as libc::c_int);
    }
    opt = 1 as libc::c_int;
    err = setsockopt(
        sock,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut opt as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if err == -(1 as libc::c_int) {
        perror(
            b"warning: failed to set TCP_NODELAY\0" as *const u8 as *const libc::c_char,
        );
    }
    freeaddrinfo(ai);
    (*conn).rfd = sock;
    (*conn).wfd = sock;
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_write(
    mut conn: *mut conn,
    mut iov: *mut iovec,
    mut count: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    while count != 0 {
        res = writev((*conn).wfd, iov, count as libc::c_int) as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else {
            if res == 0 as libc::c_int {
                fprintf(stderr, b"zero write\n\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
        }
        loop {
            if (res as libc::c_uint as libc::c_ulong) < (*iov).iov_len {
                let ref mut fresh17 = (*iov).iov_len;
                *fresh17 = (*fresh17 as libc::c_ulong).wrapping_sub(res as libc::c_ulong)
                    as size_t as size_t;
                let ref mut fresh18 = (*iov).iov_base;
                *fresh18 = (*fresh18).offset(res as isize);
                break;
            } else {
                res = (res as libc::c_ulong).wrapping_sub((*iov).iov_len) as libc::c_int
                    as libc::c_int;
                count = count.wrapping_sub(1);
                iov = iov.offset(1);
                if !(count != 0) {
                    break;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sftp_get_id() -> u32 {
    static mut idctr: u32 = 0;
    let fresh19 = idctr;
    idctr = idctr.wrapping_add(1);
    return fresh19;
}
unsafe extern "C" fn buf_to_iov(mut buf: *const buffer, mut iov: *mut iovec) {
    let ref mut fresh20 = (*iov).iov_base;
    *fresh20 = (*buf).p as *mut libc::c_void;
    (*iov).iov_len = (*buf).len;
}
unsafe extern "C" fn iov_length(
    mut iov: *const iovec,
    mut nr_segs: libc::c_ulong,
) -> size_t {
    let mut seg: libc::c_ulong = 0;
    let mut ret: size_t = 0 as libc::c_int as size_t;
    seg = 0 as libc::c_int as libc::c_ulong;
    while seg < nr_segs {
        ret = (ret as libc::c_ulong).wrapping_add((*iov.offset(seg as isize)).iov_len)
            as size_t as size_t;
        seg = seg.wrapping_add(1);
    }
    return ret;
}
unsafe extern "C" fn sftp_send_iov(
    mut conn: *mut conn,
    mut type_0: u8,
    mut id: u32,
    mut iov: *mut iovec,
    mut count: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut iovout: [iovec; 3] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 3];
    let mut i: libc::c_uint = 0;
    let mut nout: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if count <= (3 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"count <= SFTP_MAX_IOV - 1\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int sftp_send_iov(struct conn *, uint8_t, uint32_t, struct iovec *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    buf_init(&mut buf, 9 as libc::c_int as size_t);
    buf_add_uint32(
        &mut buf,
        (iov_length(iov as *const iovec, count))
            .wrapping_add(5 as libc::c_int as libc::c_ulong) as u32,
    );
    buf_add_uint8(&mut buf, type_0);
    buf_add_uint32(&mut buf, id);
    let fresh21 = nout;
    nout = nout.wrapping_add(1);
    buf_to_iov(&mut buf, &mut *iovout.as_mut_ptr().offset(fresh21 as isize));
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < count {
        let fresh22 = nout;
        nout = nout.wrapping_add(1);
        iovout[fresh22 as usize] = *iov.offset(i as isize);
        i = i.wrapping_add(1);
    }
    pthread_mutex_lock(&mut (*conn).lock_write);
    res = do_write(conn, iovout.as_mut_ptr(), nout as size_t);
    pthread_mutex_unlock(&mut (*conn).lock_write);
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn do_read(mut conn: *mut conn, mut buf: *mut buffer) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut p: *mut u8 = (*buf).p;
    let mut size: size_t = (*buf).size;
    while size != 0 {
        res = read((*conn).rfd, p as *mut libc::c_void, size) as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else {
            if res == 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"remote host has disconnected\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        size = (size as libc::c_ulong).wrapping_sub(res as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(res as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sftp_read(
    mut conn: *mut conn,
    mut type_0: *mut u8,
    mut buf: *mut buffer,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf2: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut len: u32 = 0;
    buf_init(&mut buf2, 5 as libc::c_int as size_t);
    res = do_read(conn, &mut buf2);
    if res != -(1 as libc::c_int) {
        if buf_get_uint32(&mut buf2, &mut len) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if len > ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint {
            fprintf(
                stderr,
                b"reply len too large: %u\n\0" as *const u8 as *const libc::c_char,
                len,
            );
            return -(1 as libc::c_int);
        }
        if buf_get_uint8(&mut buf2, type_0) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        buf_init(buf, len.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t);
        res = do_read(conn, buf);
    }
    buf_free(&mut buf2);
    return res;
}
unsafe extern "C" fn request_free(mut req: *mut request) {
    if ((*req).end_func).is_some() {
        ((*req).end_func).expect("non-null function pointer")(req);
    }
    let ref mut fresh23 = (*(*req).conn).req_count;
    *fresh23 -= 1;
    buf_free(&mut (*req).reply);
    sem_destroy(&mut (*req).ready);
    g_free(req as gpointer);
}
unsafe extern "C" fn chunk_free(mut chunk: *mut read_chunk) {
    while list_empty(&mut (*chunk).reqs) == 0 {
        let mut rreq: *mut read_req = 0 as *mut read_req;
        rreq = ({
            let mut __mptr: *const list_head = (*chunk).reqs.prev;
            (__mptr as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                as *mut read_req
        });
        list_del(&mut (*rreq).list);
        buf_free(&mut (*rreq).data);
        g_free(rreq as gpointer);
    }
    g_free(chunk as gpointer);
}
unsafe extern "C" fn chunk_put(mut chunk: *mut read_chunk) {
    if !chunk.is_null() {
        let ref mut fresh24 = (*chunk).refs;
        *fresh24 -= 1;
        if (*chunk).refs == 0 {
            chunk_free(chunk);
        }
    }
}
unsafe extern "C" fn chunk_put_locked(mut chunk: *mut read_chunk) {
    pthread_mutex_lock(&mut sshfs.lock);
    chunk_put(chunk);
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn clean_req(
    mut key: *mut libc::c_void,
    mut req: *mut request,
    mut user_data: gpointer,
) -> libc::c_int {
    let mut conn: *mut conn = user_data as *mut conn;
    if (*req).conn != conn {
        return 0 as libc::c_int;
    }
    (*req).error = -(5 as libc::c_int);
    if (*req).want_reply != 0 {
        sem_post(&mut (*req).ready);
    } else {
        request_free(req);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn process_one_request(mut conn: *mut conn) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut type_0: u8 = 0;
    let mut req: *mut request = 0 as *mut request;
    let mut id: u32 = 0;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    res = sftp_read(conn, &mut type_0, &mut buf);
    if res == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if buf_get_uint32(&mut buf, &mut id) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    pthread_mutex_lock(&mut sshfs.lock);
    req = g_hash_table_lookup(sshfs.reqtab, id as gulong as gpointer as gconstpointer)
        as *mut request;
    if req.is_null() {
        fprintf(
            stderr,
            b"request %i not found\n\0" as *const u8 as *const libc::c_char,
            id,
        );
    } else {
        let mut was_over: libc::c_int = 0;
        was_over = (sshfs.outstanding_len > sshfs.max_outstanding_len) as libc::c_int;
        sshfs
            .outstanding_len = (sshfs.outstanding_len as libc::c_ulong)
            .wrapping_sub((*req).len) as libc::c_uint as libc::c_uint;
        if was_over != 0 && sshfs.outstanding_len <= sshfs.max_outstanding_len {
            pthread_cond_broadcast(&mut sshfs.outstanding_cond);
        }
        g_hash_table_remove(sshfs.reqtab, id as gulong as gpointer as gconstpointer);
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if !req.is_null() {
        if sshfs.debug != 0 {
            let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
            let mut difftime: libc::c_uint = 0;
            let mut msgsize: libc::c_uint = (buf.size)
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_uint;
            gettimeofday(&mut now, 0 as *mut libc::c_void);
            difftime = ((now.tv_sec - (*req).start.tv_sec)
                * 1000 as libc::c_int as libc::c_long) as libc::c_uint;
            difftime = (difftime as libc::c_long
                + (now.tv_usec - (*req).start.tv_usec)
                    / 1000 as libc::c_int as libc::c_long) as libc::c_uint;
            if sshfs.debug != 0 {
                fprintf(
                    stderr,
                    b"  [%05i] %14s %8ubytes (%ims)\n\0" as *const u8
                        as *const libc::c_char,
                    id,
                    type_name(type_0),
                    msgsize,
                    difftime,
                );
            }
            if difftime < sshfs.min_rtt || sshfs.num_received == 0 {
                sshfs.min_rtt = difftime;
            }
            if difftime > sshfs.max_rtt {
                sshfs.max_rtt = difftime;
            }
            sshfs
                .total_rtt = (sshfs.total_rtt as libc::c_ulong)
                .wrapping_add(difftime as libc::c_ulong) as u64 as u64;
            sshfs.num_received = (sshfs.num_received).wrapping_add(1);
            sshfs
                .bytes_received = (sshfs.bytes_received as libc::c_ulong)
                .wrapping_add(msgsize as libc::c_ulong) as u64 as u64;
        }
        (*req).reply = buf;
        (*req).reply_type = type_0;
        (*req).replied = 1 as libc::c_int;
        if (*req).want_reply != 0 {
            sem_post(&mut (*req).ready);
        } else {
            pthread_mutex_lock(&mut sshfs.lock);
            request_free(req);
            pthread_mutex_unlock(&mut sshfs.lock);
        }
    } else {
        buf_free(&mut buf);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn close_conn(mut conn: *mut conn) {
    close((*conn).rfd);
    if (*conn).rfd != (*conn).wfd {
        close((*conn).wfd);
    }
    (*conn).rfd = -(1 as libc::c_int);
    (*conn).wfd = -(1 as libc::c_int);
    if sshfs.ptyfd != -(1 as libc::c_int) {
        close(sshfs.ptyfd);
        sshfs.ptyfd = -(1 as libc::c_int);
    }
    if sshfs.ptypassivefd != -(1 as libc::c_int) {
        close(sshfs.ptypassivefd);
        sshfs.ptypassivefd = -(1 as libc::c_int);
    }
}
unsafe extern "C" fn process_requests(
    mut data_: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut conn: *mut conn = data_ as *mut conn;
    while !(process_one_request(conn) == -(1 as libc::c_int)) {}
    pthread_mutex_lock(&mut sshfs.lock);
    (*conn).processing_thread_started = 0 as libc::c_int;
    close_conn(conn);
    g_hash_table_foreach_remove(
        sshfs.reqtab,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut request,
                    gpointer,
                ) -> libc::c_int,
            >,
            GHRFunc,
        >(
            Some(
                clean_req
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut request,
                        gpointer,
                    ) -> libc::c_int,
            ),
        ),
        conn as gpointer,
    );
    sshfs.connvers += 1;
    (*conn).connver = sshfs.connvers;
    sshfs.outstanding_len = 0 as libc::c_int as libc::c_uint;
    pthread_cond_broadcast(&mut sshfs.outstanding_cond);
    pthread_mutex_unlock(&mut sshfs.lock);
    if sshfs.reconnect == 0 {
        kill(getpid(), 15 as libc::c_int);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn sftp_init_reply_ok(
    mut conn: *mut conn,
    mut buf: *mut buffer,
    mut version: *mut u32,
) -> libc::c_int {
    let mut len: u32 = 0;
    let mut type_0: u8 = 0;
    if buf_get_uint32(buf, &mut len) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if len < 5 as libc::c_int as libc::c_uint
        || len > ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if buf_get_uint8(buf, &mut type_0) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if type_0 as libc::c_int != 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if buf_get_uint32(buf, version) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if sshfs.debug != 0 {
        fprintf(
            stderr,
            b"Server version: %u\n\0" as *const u8 as *const libc::c_char,
            *version,
        );
    }
    if len > 5 as libc::c_int as libc::c_uint {
        let mut buf2: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        buf_init(
            &mut buf2,
            len.wrapping_sub(5 as libc::c_int as libc::c_uint) as size_t,
        );
        if do_read(conn, &mut buf2) == -(1 as libc::c_int) {
            buf_free(&mut buf2);
            return -(1 as libc::c_int);
        }
        loop {
            let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut extdata: *mut libc::c_char = 0 as *mut libc::c_char;
            if buf_get_string(&mut buf2, &mut ext) == -(1 as libc::c_int)
                || buf_get_string(&mut buf2, &mut extdata) == -(1 as libc::c_int)
            {
                buf_free(&mut buf2);
                free(ext as *mut libc::c_void);
                free(extdata as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if sshfs.debug != 0 {
                fprintf(
                    stderr,
                    b"Extension: %s <%s>\n\0" as *const u8 as *const libc::c_char,
                    ext,
                    extdata,
                );
            }
            if strcmp(
                ext,
                b"posix-rename@openssh.com\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                && strcmp(extdata, b"1\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                sshfs.ext_posix_rename = 1 as libc::c_int;
                sshfs.rename_workaround = 0 as libc::c_int;
            }
            if strcmp(ext, b"statvfs@openssh.com\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                && strcmp(extdata, b"2\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                sshfs.ext_statvfs = 1 as libc::c_int;
            }
            if strcmp(ext, b"hardlink@openssh.com\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                && strcmp(extdata, b"1\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                sshfs.ext_hardlink = 1 as libc::c_int;
            }
            if strcmp(ext, b"fsync@openssh.com\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                && strcmp(extdata, b"1\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                sshfs.ext_fsync = 1 as libc::c_int;
            }
            free(ext as *mut libc::c_void);
            free(extdata as *mut libc::c_void);
            if !(buf2.len < buf2.size) {
                break;
            }
        }
        buf_free(&mut buf2);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sftp_find_init_reply(
    mut conn: *mut conn,
    mut version: *mut u32,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 9 as libc::c_int as size_t);
    res = do_read(conn, &mut buf);
    while res != -(1 as libc::c_int) {
        let mut buf2: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        res = sftp_init_reply_ok(conn, &mut buf, version);
        if res <= 0 as libc::c_int {
            break;
        }
        if sshfs.debug != 0 {
            fprintf(
                stderr,
                b"%c\0" as *const u8 as *const libc::c_char,
                *buf.p as libc::c_int,
            );
        }
        memmove(
            buf.p as *mut libc::c_void,
            (buf.p).offset(1 as libc::c_int as isize) as *const libc::c_void,
            (buf.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        buf.len = 0 as libc::c_int as size_t;
        buf2.p = (buf.p).offset(buf.size as isize).offset(-(1 as libc::c_int as isize));
        buf2.size = 1 as libc::c_int as size_t;
        res = do_read(conn, &mut buf2);
    }
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn sftp_init(mut conn: *mut conn) -> libc::c_int {
    let mut res: libc::c_int = -(1 as libc::c_int);
    let mut version: u32 = 0 as libc::c_int as u32;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if !(sftp_send_iov(
        conn,
        1 as libc::c_int as u8,
        3 as libc::c_int as u32,
        0 as *mut iovec,
        0 as libc::c_int as size_t,
    ) == -(1 as libc::c_int))
    {
        if !(sshfs.password_stdin != 0 && pty_expect_loop(conn) == -(1 as libc::c_int)) {
            if !(sftp_find_init_reply(conn, &mut version) == -(1 as libc::c_int)) {
                sshfs.server_version = version as libc::c_int;
                if version > 3 as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"Warning: server uses version: %i, we support: %i\n\0"
                            as *const u8 as *const libc::c_char,
                        version,
                        3 as libc::c_int,
                    );
                }
                res = 0 as libc::c_int;
            }
        }
    }
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn sftp_error_to_errno(mut error: u32) -> libc::c_int {
    match error {
        0 => return 0 as libc::c_int,
        2 => return 2 as libc::c_int,
        3 => return 13 as libc::c_int,
        4 => return 1 as libc::c_int,
        5 => return 74 as libc::c_int,
        6 => return 107 as libc::c_int,
        7 => return 103 as libc::c_int,
        8 => return 95 as libc::c_int,
        _ => return 5 as libc::c_int,
    };
}
unsafe extern "C" fn sftp_detect_uid(mut conn: *mut conn) {
    let mut flags: libc::c_int = 0;
    let mut id: u32 = sftp_get_id();
    let mut replid: u32 = 0;
    let mut type_0: u8 = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut iov: [iovec; 1] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1];
    buf_init(&mut buf, 5 as libc::c_int as size_t);
    buf_add_string(&mut buf, b".\0" as *const u8 as *const libc::c_char);
    buf_to_iov(&mut buf, &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize));
    if !(sftp_send_iov(
        conn,
        17 as libc::c_int as u8,
        id,
        iov.as_mut_ptr(),
        1 as libc::c_int as size_t,
    ) == -(1 as libc::c_int))
    {
        buf_clear(&mut buf);
        if !(sftp_read(conn, &mut type_0, &mut buf) == -(1 as libc::c_int)) {
            if type_0 as libc::c_int != 105 as libc::c_int
                && type_0 as libc::c_int != 101 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"protocol error\n\0" as *const u8 as *const libc::c_char,
                );
            } else if !(buf_get_uint32(&mut buf, &mut replid) == -(1 as libc::c_int)) {
                if replid != id {
                    fprintf(
                        stderr,
                        b"bad reply ID\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if type_0 as libc::c_int == 101 as libc::c_int {
                    let mut serr: u32 = 0;
                    if !(buf_get_uint32(&mut buf, &mut serr) == -(1 as libc::c_int)) {
                        fprintf(
                            stderr,
                            b"failed to stat home directory (%i)\n\0" as *const u8
                                as *const libc::c_char,
                            serr,
                        );
                    }
                } else if !(buf_get_attrs(&mut buf, &mut stbuf, &mut flags)
                    != 0 as libc::c_int)
                {
                    if !(flags & 0x2 as libc::c_int == 0) {
                        sshfs.remote_uid = stbuf.st_uid;
                        sshfs.local_uid = getuid();
                        sshfs.remote_gid = stbuf.st_gid;
                        sshfs.local_gid = getgid();
                        sshfs.remote_uid_detected = 1 as libc::c_int;
                        if sshfs.debug != 0 {
                            fprintf(
                                stderr,
                                b"remote_uid = %i\n\0" as *const u8 as *const libc::c_char,
                                sshfs.remote_uid,
                            );
                        }
                    }
                }
            }
        }
    }
    if sshfs.remote_uid_detected == 0 {
        fprintf(
            stderr,
            b"failed to detect remote user ID\n\0" as *const u8 as *const libc::c_char,
        );
    }
    buf_free(&mut buf);
}
unsafe extern "C" fn sftp_check_root(
    mut conn: *mut conn,
    mut base_path: *const libc::c_char,
) -> libc::c_int {
    let mut err2: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut id: u32 = sftp_get_id();
    let mut replid: u32 = 0;
    let mut type_0: u8 = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut iov: [iovec; 1] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1];
    let mut err: libc::c_int = -(1 as libc::c_int);
    let mut remote_dir: *const libc::c_char = if *base_path
        .offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        base_path
    } else {
        b".\0" as *const u8 as *const libc::c_char
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, remote_dir);
    buf_to_iov(&mut buf, &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize));
    if !(sftp_send_iov(
        conn,
        7 as libc::c_int as u8,
        id,
        iov.as_mut_ptr(),
        1 as libc::c_int as size_t,
    ) == -(1 as libc::c_int))
    {
        buf_clear(&mut buf);
        if !(sftp_read(conn, &mut type_0, &mut buf) == -(1 as libc::c_int)) {
            if type_0 as libc::c_int != 105 as libc::c_int
                && type_0 as libc::c_int != 101 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"protocol error\n\0" as *const u8 as *const libc::c_char,
                );
            } else if !(buf_get_uint32(&mut buf, &mut replid) == -(1 as libc::c_int)) {
                if replid != id {
                    fprintf(
                        stderr,
                        b"bad reply ID\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if type_0 as libc::c_int == 101 as libc::c_int {
                    let mut serr: u32 = 0;
                    if !(buf_get_uint32(&mut buf, &mut serr) == -(1 as libc::c_int)) {
                        fprintf(
                            stderr,
                            b"%s:%s: %s\n\0" as *const u8 as *const libc::c_char,
                            sshfs.host,
                            remote_dir,
                            strerror(sftp_error_to_errno(serr)),
                        );
                    }
                } else {
                    err2 = buf_get_attrs(&mut buf, &mut stbuf, &mut flags);
                    if err2 != 0 {
                        err = err2;
                    } else if !(flags & 0x4 as libc::c_int == 0) {
                        if !(stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                        {
                            fprintf(
                                stderr,
                                b"%s:%s: Not a directory\n\0" as *const u8
                                    as *const libc::c_char,
                                sshfs.host,
                                remote_dir,
                            );
                        } else {
                            err = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    buf_free(&mut buf);
    return err;
}
unsafe fn connect_remote(mut conn: *mut conn) -> libc::c_int {
    let mut err: libc::c_int = 0;
    if sshfs.passive != 0 {
        err = connect_passive(conn);
    } else if !(sshfs.directport).is_null() {
        err = connect_to(conn, sshfs.host, sshfs.directport);
    } else {
        err = start_ssh(conn);
    }
    if err == 0 {
        err = sftp_init(conn);
    }
    if err != 0 {
        close_conn(conn);
    } else {
        sshfs.num_connect = (sshfs.num_connect).wrapping_add(1);
    }
    return err;
}
unsafe extern "C" fn start_processing_thread(mut conn: *mut conn) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut thread_id: pthread_t = 0;
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut newset: sigset_t = sigset_t { __val: [0; 16] };
    if (*conn).processing_thread_started != 0 {
        return 0 as libc::c_int;
    }
    if (*conn).rfd == -(1 as libc::c_int) {
        err = connect_remote(conn);
        if err != 0 {
            return -(5 as libc::c_int);
        }
    }
    if sshfs.detect_uid != 0 {
        sftp_detect_uid(conn);
        sshfs.detect_uid = 0 as libc::c_int;
    }
    sigemptyset(&mut newset);
    sigaddset(&mut newset, 15 as libc::c_int);
    sigaddset(&mut newset, 2 as libc::c_int);
    sigaddset(&mut newset, 1 as libc::c_int);
    sigaddset(&mut newset, 3 as libc::c_int);
    pthread_sigmask(0 as libc::c_int, &mut newset, &mut oldset);
    err = pthread_create(
        &mut thread_id,
        0 as *const pthread_attr_t,
        Some(
            process_requests
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        conn as *mut libc::c_void,
    );
    if err != 0 {
        fprintf(
            stderr,
            b"failed to create thread: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(err),
        );
        return -(5 as libc::c_int);
    }
    pthread_detach(thread_id);
    pthread_sigmask(2 as libc::c_int, &mut oldset, 0 as *mut __sigset_t);
    (*conn).processing_thread_started = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_init(
    mut conn: *mut fuse_conn_info,
    mut cfg: *mut fuse_config,
) -> *mut libc::c_void {
    if (*conn).capable & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        sshfs.sync_read = 1 as libc::c_int;
    }
    (*cfg)
        .nullpath_ok = !(sshfs.truncate_workaround != 0 || sshfs.fstat_workaround != 0)
        as libc::c_int;
    if sshfs.max_conns > 1 as libc::c_int {
        (*cfg).nullpath_ok = 0 as libc::c_int;
    }
    (*conn).capable |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
    if sshfs.delay_connect == 0 {
        start_processing_thread(&mut *(sshfs.conns).offset(0 as libc::c_int as isize));
    }
    (*conn).time_gran = 1000000000 as libc::c_int as libc::c_uint;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn sftp_request_wait(
    mut req: *mut request,
    mut type_0: u8,
    mut expect_type: u8,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    if (*req).error != 0 {
        err = (*req).error;
    } else {
        while sem_wait(&mut (*req).ready) != 0 {}
        if (*req).error != 0 {
            err = (*req).error;
        } else {
            err = -(5 as libc::c_int);
            if (*req).reply_type as libc::c_int != expect_type as libc::c_int
                && (*req).reply_type as libc::c_int != 101 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"protocol error\n\0" as *const u8 as *const libc::c_char,
                );
            } else if (*req).reply_type as libc::c_int == 101 as libc::c_int {
                let mut serr: u32 = 0;
                if !(buf_get_uint32(&mut (*req).reply, &mut serr) == -(1 as libc::c_int))
                {
                    match serr {
                        0 => {
                            if expect_type as libc::c_int == 101 as libc::c_int {
                                err = 0 as libc::c_int;
                            } else {
                                err = -(5 as libc::c_int);
                            }
                        }
                        1 => {
                            if type_0 as libc::c_int == 5 as libc::c_int
                                || type_0 as libc::c_int == 12 as libc::c_int
                            {
                                err = 1 as libc::c_int;
                            } else {
                                err = -(5 as libc::c_int);
                            }
                        }
                        4 => {
                            if type_0 as libc::c_int == 15 as libc::c_int {
                                err = -(39 as libc::c_int);
                            } else {
                                err = -(1 as libc::c_int);
                            }
                        }
                        _ => {
                            err = -sftp_error_to_errno(serr);
                        }
                    }
                }
            } else {
                buf_init(outbuf, ((*req).reply.size).wrapping_sub((*req).reply.len));
                buf_get_mem(
                    &mut (*req).reply,
                    (*outbuf).p as *mut libc::c_void,
                    (*outbuf).size,
                );
                err = 0 as libc::c_int;
            }
        }
    }
    pthread_mutex_lock(&mut sshfs.lock);
    request_free(req);
    pthread_mutex_unlock(&mut sshfs.lock);
    return err;
}
unsafe extern "C" fn sftp_request_send(
    mut conn: *mut conn,
    mut type_0: u8,
    mut iov: *mut iovec,
    mut count: size_t,
    mut begin_func: request_func,
    mut end_func: request_func,
    mut want_reply: libc::c_int,
    mut data: *mut libc::c_void,
    mut reqp: *mut *mut request,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut id: u32 = 0;
    let mut req: *mut request = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::std::mem::size_of::<request>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut request;
    (*req).want_reply = want_reply as libc::c_uint;
    let ref mut fresh25 = (*req).end_func;
    *fresh25 = end_func;
    let ref mut fresh26 = (*req).data;
    *fresh26 = data;
    sem_init(&mut (*req).ready, 0 as libc::c_int, 0 as libc::c_int as libc::c_uint);
    buf_init(&mut (*req).reply, 0 as libc::c_int as size_t);
    pthread_mutex_lock(&mut sshfs.lock);
    if begin_func.is_some() {
        begin_func.expect("non-null function pointer")(req);
    }
    id = sftp_get_id();
    (*req).id = id;
    let ref mut fresh27 = (*req).conn;
    *fresh27 = conn;
    let ref mut fresh28 = (*(*req).conn).req_count;
    *fresh28 += 1;
    err = start_processing_thread(conn);
    if err != 0 {
        pthread_mutex_unlock(&mut sshfs.lock);
    } else {
        (*req)
            .len = (iov_length(iov, count))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
        sshfs
            .outstanding_len = (sshfs.outstanding_len as libc::c_ulong)
            .wrapping_add((*req).len) as libc::c_uint as libc::c_uint;
        while sshfs.outstanding_len > sshfs.max_outstanding_len {
            pthread_cond_wait(&mut sshfs.outstanding_cond, &mut sshfs.lock);
        }
        g_hash_table_insert(sshfs.reqtab, id as gulong as gpointer, req as gpointer);
        if sshfs.debug != 0 {
            gettimeofday(&mut (*req).start, 0 as *mut libc::c_void);
            sshfs.num_sent = (sshfs.num_sent).wrapping_add(1);
            sshfs
                .bytes_sent = (sshfs.bytes_sent as libc::c_ulong)
                .wrapping_add((*req).len) as u64 as u64;
        }
        if sshfs.debug != 0 {
            fprintf(
                stderr,
                b"[%05i] %s\n\0" as *const u8 as *const libc::c_char,
                id,
                type_name(type_0),
            );
        }
        pthread_mutex_unlock(&mut sshfs.lock);
        err = -(5 as libc::c_int);
        if sftp_send_iov(conn, type_0, id, iov, count) == -(1 as libc::c_int) {
            let mut rmed: gboolean = 0;
            pthread_mutex_lock(&mut sshfs.lock);
            rmed = g_hash_table_remove(
                sshfs.reqtab,
                id as gulong as gpointer as gconstpointer,
            );
            pthread_mutex_unlock(&mut sshfs.lock);
            if rmed == 0 && want_reply == 0 {
                return err;
            }
        } else {
            if want_reply != 0 {
                *reqp = req;
            }
            return 0 as libc::c_int;
        }
    }
    (*req).error = err;
    if want_reply == 0 {
        sftp_request_wait(req, type_0, 0 as libc::c_int as u8, 0 as *mut buffer);
    } else {
        *reqp = req;
    }
    return err;
}
unsafe extern "C" fn sftp_request_iov(
    mut conn: *mut conn,
    mut type_0: u8,
    mut iov: *mut iovec,
    mut count: size_t,
    mut expect_type: u8,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut req: *mut request = 0 as *mut request;
    err = sftp_request_send(
        conn,
        type_0,
        iov,
        count,
        None,
        None,
        expect_type as libc::c_int,
        0 as *mut libc::c_void,
        &mut req,
    );
    if expect_type as libc::c_int == 0 as libc::c_int {
        return err;
    }
    return sftp_request_wait(req, type_0, expect_type, outbuf);
}
unsafe extern "C" fn sftp_request(
    mut conn: *mut conn,
    mut type_0: u8,
    mut buf: *const buffer,
    mut expect_type: u8,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    buf_to_iov(buf, &mut iov);
    return sftp_request_iov(
        conn,
        type_0,
        &mut iov,
        1 as libc::c_int as size_t,
        expect_type,
        outbuf,
    );
}
unsafe extern "C" fn sshfs_access(
    mut path: *const libc::c_char,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut err: libc::c_int = 0 as libc::c_int;
    if mask & 1 as libc::c_int != 0 {
        err = ((*sshfs.op).getattr)
            .expect(
                "non-null function pointer",
            )(path, &mut stbuf, 0 as *mut fuse_file_info);
        if err == 0 {
            if stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                && stbuf.st_mode
                    & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint == 0
            {
                err = -(13 as libc::c_int);
            }
        }
    }
    return err;
}
unsafe extern "C" fn count_components(mut p: *const libc::c_char) -> libc::c_int {
    let mut ctr: libc::c_int = 0;
    while *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
    }
    ctr = 0 as libc::c_int;
    while *p != 0 {
        while *p as libc::c_int != 0 && *p as libc::c_int != '/' as i32 {
            p = p.offset(1);
        }
        while *p as libc::c_int == '/' as i32 {
            p = p.offset(1);
        }
        ctr += 1;
    }
    return ctr;
}
unsafe extern "C" fn strip_common(
    mut sp: *mut *const libc::c_char,
    mut tp: *mut *const libc::c_char,
) {
    let mut s: *const libc::c_char = *sp;
    let mut t: *const libc::c_char = *tp;
    loop {
        while *s as libc::c_int == '/' as i32 {
            s = s.offset(1);
        }
        while *t as libc::c_int == '/' as i32 {
            t = t.offset(1);
        }
        *tp = t;
        *sp = s;
        while *s as libc::c_int == *t as libc::c_int && *s as libc::c_int != 0
            && *s as libc::c_int != '/' as i32
        {
            s = s.offset(1);
            t = t.offset(1);
        }
        if !(*s as libc::c_int == *t as libc::c_int && *s as libc::c_int != 0
            || *s == 0 && *t as libc::c_int == '/' as i32
            || *s as libc::c_int == '/' as i32 && *t == 0)
        {
            break;
        }
    };
}
unsafe extern "C" fn transform_symlink(
    mut path: *const libc::c_char,
    mut linkp: *mut *mut libc::c_char,
) {
    let mut l: *const libc::c_char = *linkp;
    let mut b: *const libc::c_char = sshfs.base_path;
    let mut newlink: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dotdots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if *l.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
        || *b.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
    {
        return;
    }
    strip_common(&mut l, &mut b);
    if *b != 0 {
        return;
    }
    strip_common(&mut l, &mut path);
    dotdots = count_components(path);
    if dotdots == 0 {
        return;
    }
    dotdots -= 1;
    newlink = malloc(
        ((dotdots * 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(strlen(l))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if newlink.is_null() {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    s = newlink;
    i = 0 as libc::c_int;
    while i < dotdots {
        strcpy(s, b"../\0" as *const u8 as *const libc::c_char);
        i += 1;
        s = s.offset(3 as libc::c_int as isize);
    }
    if *l.offset(0 as libc::c_int as isize) != 0 {
        strcpy(s, l);
    } else if dotdots == 0 {
        strcpy(s, b".\0" as *const u8 as *const libc::c_char);
    } else {
        *s.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    free(*linkp as *mut libc::c_void);
    *linkp = newlink;
}
unsafe extern "C" fn sshfs_readlink(
    mut path: *const libc::c_char,
    mut linkbuf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut name: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    if size > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"int sshfs_readlink(const char *, char *, size_t)\0"))
                .as_ptr(),
        );
    }
    if sshfs.server_version < 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        19 as libc::c_int as u8,
        &mut buf,
        104 as libc::c_int as u8,
        &mut name,
    );
    if err == 0 {
        let mut count: u32 = 0;
        let mut link: *mut libc::c_char = 0 as *mut libc::c_char;
        err = -(5 as libc::c_int);
        if buf_get_uint32(&mut name, &mut count) != -(1 as libc::c_int)
            && count == 1 as libc::c_int as libc::c_uint
            && buf_get_string(&mut name, &mut link) != -(1 as libc::c_int)
        {
            if sshfs.transform_symlinks != 0 {
                transform_symlink(path, &mut link);
            }
            strncpy(linkbuf, link, size.wrapping_sub(1 as libc::c_int as libc::c_ulong));
            *linkbuf
                .offset(
                    size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            free(link as *mut libc::c_void);
            err = 0 as libc::c_int;
        }
        buf_free(&mut name);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sftp_readdir_send(
    mut conn: *mut conn,
    mut req: *mut *mut request,
    mut handle: *mut buffer,
) -> libc::c_int {
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    buf_to_iov(handle, &mut iov);
    return sftp_request_send(
        conn,
        12 as libc::c_int as u8,
        &mut iov,
        1 as libc::c_int as size_t,
        None,
        None,
        104 as libc::c_int,
        0 as *mut libc::c_void,
        req,
    );
}
unsafe extern "C" fn sshfs_req_pending(mut req: *mut request) -> libc::c_int {
    if !(g_hash_table_lookup(
        sshfs.reqtab,
        (*req).id as gulong as gpointer as gconstpointer,
    ))
        .is_null()
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn sftp_readdir_async(
    mut conn: *mut conn,
    mut handle: *mut buffer,
    mut buf: *mut libc::c_void,
    mut offset: off_t,
    mut filler: fuse_fill_dir_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut outstanding: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 2 as libc::c_int;
    let mut list: *mut GList = 0 as *mut GList;
    let mut done: libc::c_int = 0 as libc::c_int;
    if offset == 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"int sftp_readdir_async(struct conn *, struct buffer *, void *, off_t, fuse_fill_dir_t)\0",
            ))
                .as_ptr(),
        );
    }
    while done == 0 || outstanding != 0 {
        let mut req: *mut request = 0 as *mut request;
        let mut name: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        let mut tmperr: libc::c_int = 0;
        while done == 0 && outstanding < max {
            tmperr = sftp_readdir_send(conn, &mut req, handle);
            if tmperr != 0 && done == 0 {
                err = tmperr;
                done = 1 as libc::c_int;
                break;
            } else {
                list = g_list_append(list, req as gpointer);
                outstanding += 1;
            }
        }
        if !(outstanding != 0) {
            continue;
        }
        let mut first: *mut GList = 0 as *mut GList;
        first = g_list_first(list);
        req = (*first).data as *mut request;
        list = g_list_delete_link(list, first);
        outstanding -= 1;
        if done != 0 {
            let mut want_reply: libc::c_int = 0;
            pthread_mutex_lock(&mut sshfs.lock);
            if sshfs_req_pending(req) != 0 {
                (*req).want_reply = 0 as libc::c_int as libc::c_uint;
            }
            want_reply = (*req).want_reply as libc::c_int;
            pthread_mutex_unlock(&mut sshfs.lock);
            if want_reply == 0 {
                continue;
            }
        }
        tmperr = sftp_request_wait(
            req,
            12 as libc::c_int as u8,
            104 as libc::c_int as u8,
            &mut name,
        );
        if tmperr != 0 && done == 0 {
            err = tmperr;
            if err == 1 as libc::c_int {
                err = 0 as libc::c_int;
            }
            done = 1 as libc::c_int;
        }
        if done == 0 {
            err = buf_get_entries(&mut name, buf, filler);
            buf_free(&mut name);
            if max < 32 as libc::c_int {
                max += 1;
            }
            if err != 0 {
                done = 1 as libc::c_int;
            }
        }
    }
    if list.is_null() {} else {
        __assert_fail(
            b"list == NULL\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2273 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"int sftp_readdir_async(struct conn *, struct buffer *, void *, off_t, fuse_fill_dir_t)\0",
            ))
                .as_ptr(),
        );
    }
    return err;
}
unsafe extern "C" fn sftp_readdir_sync(
    mut conn: *mut conn,
    mut handle: *mut buffer,
    mut buf: *mut libc::c_void,
    mut offset: off_t,
    mut filler: fuse_fill_dir_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    if offset == 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2282 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"int sftp_readdir_sync(struct conn *, struct buffer *, void *, off_t, fuse_fill_dir_t)\0",
            ))
                .as_ptr(),
        );
    }
    loop {
        let mut name: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        err = sftp_request(
            conn,
            12 as libc::c_int as u8,
            handle,
            104 as libc::c_int as u8,
            &mut name,
        );
        if err == 0 {
            err = buf_get_entries(&mut name, buf, filler);
            buf_free(&mut name);
        }
        if !(err == 0) {
            break;
        }
    }
    if err == 1 as libc::c_int {
        err = 0 as libc::c_int;
    }
    return err;
}
unsafe extern "C" fn sshfs_opendir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut conn: *mut conn = 0 as *mut conn;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    handle = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::std::mem::size_of::<dir_handle>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut dir_handle;
    if handle.is_null() {
        return -(12 as libc::c_int);
    }
    conn = get_conn(0 as *const sshfs_file, 0 as *const libc::c_char);
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    err = sftp_request(
        conn,
        11 as libc::c_int as u8,
        &mut buf,
        102 as libc::c_int as u8,
        &mut (*handle).buf,
    );
    if err == 0 {
        buf_finish(&mut (*handle).buf);
        pthread_mutex_lock(&mut sshfs.lock);
        let ref mut fresh29 = (*handle).conn;
        *fresh29 = conn;
        let ref mut fresh30 = (*(*handle).conn).dir_count;
        *fresh30 += 1;
        pthread_mutex_unlock(&mut sshfs.lock);
        (*fi).fh = handle as libc::c_ulong;
    } else {
        g_free(handle as gpointer);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_readdir(
    mut path: *const libc::c_char,
    mut dbuf: *mut libc::c_void,
    mut filler: fuse_fill_dir_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
    mut flags: fuse_readdir_flags,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    handle = (*fi).fh as *mut dir_handle;
    if sshfs.sync_readdir != 0 {
        err = sftp_readdir_sync(
            (*handle).conn,
            &mut (*handle).buf,
            dbuf,
            offset,
            filler,
        );
    } else {
        err = sftp_readdir_async(
            (*handle).conn,
            &mut (*handle).buf,
            dbuf,
            offset,
            filler,
        );
    }
    return err;
}
unsafe extern "C" fn sshfs_releasedir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    handle = (*fi).fh as *mut dir_handle;
    err = sftp_request(
        (*handle).conn,
        4 as libc::c_int as u8,
        &mut (*handle).buf,
        0 as libc::c_int as u8,
        0 as *mut buffer,
    );
    pthread_mutex_lock(&mut sshfs.lock);
    let ref mut fresh31 = (*(*handle).conn).dir_count;
    *fresh31 -= 1;
    pthread_mutex_unlock(&mut sshfs.lock);
    buf_free(&mut (*handle).buf);
    g_free(handle as gpointer);
    return err;
}
unsafe extern "C" fn sshfs_mkdir(
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(&mut buf, 0x4 as libc::c_int as u32);
    buf_add_uint32(&mut buf, mode);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        14 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    if err == -(1 as libc::c_int) {
        if ((*sshfs.op).access)
            .expect("non-null function pointer")(path, 4 as libc::c_int)
            == 0 as libc::c_int
        {
            return -(17 as libc::c_int);
        }
    }
    return err;
}
unsafe extern "C" fn sshfs_mknod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut rdev: dev_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut conn: *mut conn = 0 as *mut conn;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut handle: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    if mode & 0o170000 as libc::c_int as libc::c_uint
        != 0o100000 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    conn = get_conn(0 as *const sshfs_file, 0 as *const libc::c_char);
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(
        &mut buf,
        (0x2 as libc::c_int | 0x8 as libc::c_int | 0x20 as libc::c_int) as u32,
    );
    buf_add_uint32(&mut buf, 0x4 as libc::c_int as u32);
    buf_add_uint32(&mut buf, mode);
    err = sftp_request(
        conn,
        3 as libc::c_int as u8,
        &mut buf,
        102 as libc::c_int as u8,
        &mut handle,
    );
    if err == 0 {
        let mut err2: libc::c_int = 0;
        buf_finish(&mut handle);
        err2 = sftp_request(
            conn,
            4 as libc::c_int as u8,
            &mut handle,
            101 as libc::c_int as u8,
            0 as *mut buffer,
        );
        if err == 0 {
            err = err2;
        }
        buf_free(&mut handle);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_symlink(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    if sshfs.server_version < 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, from);
    buf_add_path(&mut buf, to);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        20 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_unlink(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        13 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_rmdir(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        15 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_do_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, from);
    buf_add_path(&mut buf, to);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        18 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_ext_posix_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(
        &mut buf,
        b"posix-rename@openssh.com\0" as *const u8 as *const libc::c_char,
    );
    buf_add_path(&mut buf, from);
    buf_add_path(&mut buf, to);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        200 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn random_string(mut str: *mut libc::c_char, mut length: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        let fresh32 = str;
        str = str.offset(1);
        *fresh32 = ('0' as i32 + rand_r(&mut sshfs.randseed) % 10 as libc::c_int)
            as libc::c_char;
        i += 1;
    }
    *str = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn sshfs_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    if flags != 0 as libc::c_int as libc::c_uint {
        return -(22 as libc::c_int);
    }
    if sshfs.ext_posix_rename != 0 {
        err = sshfs_ext_posix_rename(from, to);
    } else {
        err = sshfs_do_rename(from, to);
    }
    if err == -(1 as libc::c_int) && sshfs.rename_workaround != 0 {
        let mut tolen: size_t = strlen(to);
        if tolen.wrapping_add(8 as libc::c_int as libc::c_ulong)
            < 4096 as libc::c_int as libc::c_ulong
        {
            let mut tmperr: libc::c_int = 0;
            let mut totmp: [libc::c_char; 4096] = [0; 4096];
            strcpy(totmp.as_mut_ptr(), to);
            random_string(totmp.as_mut_ptr().offset(tolen as isize), 8 as libc::c_int);
            tmperr = sshfs_do_rename(to, totmp.as_mut_ptr());
            if tmperr == 0 {
                err = sshfs_do_rename(from, to);
                if err == 0 {
                    err = sshfs_unlink(totmp.as_mut_ptr());
                } else {
                    sshfs_do_rename(totmp.as_mut_ptr(), to);
                }
            }
        }
    }
    if err == -(1 as libc::c_int) && sshfs.renamexdev_workaround != 0 {
        err = -(18 as libc::c_int);
    }
    if err == 0 && sshfs.max_conns > 1 as libc::c_int {
        pthread_mutex_lock(&mut sshfs.lock);
        ce = g_hash_table_lookup(sshfs.conntab, from as gconstpointer)
            as *mut conntab_entry;
        if !ce.is_null() {
            g_hash_table_replace(
                sshfs.conntab,
                g_strdup(to) as gpointer,
                ce as gpointer,
            );
            g_hash_table_remove(sshfs.conntab, from as gconstpointer);
        }
        pthread_mutex_unlock(&mut sshfs.lock);
    }
    return err;
}
unsafe extern "C" fn sshfs_link(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = -(38 as libc::c_int);
    if sshfs.ext_hardlink != 0 && sshfs.disable_hardlink == 0 {
        let mut buf: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_string(
            &mut buf,
            b"hardlink@openssh.com\0" as *const u8 as *const libc::c_char,
        );
        buf_add_path(&mut buf, from);
        buf_add_path(&mut buf, to);
        err = sftp_request(
            get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
            200 as libc::c_int as u8,
            &mut buf,
            101 as libc::c_int as u8,
            0 as *mut buffer,
        );
        buf_free(&mut buf);
    }
    return err;
}
#[inline]
unsafe extern "C" fn sshfs_file_is_conn(mut sf: *mut sshfs_file) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    pthread_mutex_lock(&mut sshfs.lock);
    ret = ((*sf).connver == (*(*sf).conn).connver) as libc::c_int;
    pthread_mutex_unlock(&mut sshfs.lock);
    return ret;
}
#[inline]
unsafe extern "C" fn get_sshfs_file(mut fi: *mut fuse_file_info) -> *mut sshfs_file {
    return (*fi).fh as *mut sshfs_file;
}
unsafe extern "C" fn sshfs_chmod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    if !fi.is_null() {
        sf = get_sshfs_file(fi);
        if sshfs_file_is_conn(sf) == 0 {
            return -(5 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf.is_null() {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle);
    }
    buf_add_uint32(&mut buf, 0x4 as libc::c_int as u32);
    buf_add_uint32(&mut buf, mode);
    err = sftp_request(
        get_conn(sf, 0 as *const libc::c_char),
        (if sf.is_null() { 9 as libc::c_int } else { 10 as libc::c_int }) as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_chown(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    if !fi.is_null() {
        sf = get_sshfs_file(fi);
        if sshfs_file_is_conn(sf) == 0 {
            return -(5 as libc::c_int);
        }
    }
    if sshfs.remote_uid_detected != 0 {
        if uid == sshfs.local_uid {
            uid = sshfs.remote_uid;
        }
        if gid == sshfs.local_gid {
            gid = sshfs.remote_gid;
        }
    }
    if sshfs.idmap == IDMAP_FILE as libc::c_int && !(sshfs.r_uid_map).is_null() {
        if id_map::translate_id(&mut uid, "ruid", sshfs.nomap) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if sshfs.idmap == IDMAP_FILE as libc::c_int && !(sshfs.r_gid_map).is_null() {
        if id_map::translate_id(&mut gid, "rgid", sshfs.nomap) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf.is_null() {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle);
    }
    buf_add_uint32(&mut buf, 0x2 as libc::c_int as u32);
    buf_add_uint32(&mut buf, uid);
    buf_add_uint32(&mut buf, gid);
    err = sftp_request(
        get_conn(sf, 0 as *const libc::c_char),
        (if sf.is_null() { 9 as libc::c_int } else { 10 as libc::c_int }) as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_inc_modifver() {
    pthread_mutex_lock(&mut sshfs.lock);
    sshfs.modifver += 1;
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn sshfs_utimens(
    mut path: *const libc::c_char,
    mut tv: *const timespec,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut asec: time_t = (*tv.offset(0 as libc::c_int as isize)).tv_sec;
    let mut msec: time_t = (*tv.offset(1 as libc::c_int as isize)).tv_sec;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    if asec == 0 as libc::c_int as libc::c_long {
        asec = now.tv_sec;
    }
    if msec == 0 as libc::c_int as libc::c_long {
        msec = now.tv_sec;
    }
    if !fi.is_null() {
        sf = get_sshfs_file(fi);
        if sshfs_file_is_conn(sf) == 0 {
            return -(5 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf.is_null() {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle);
    }
    buf_add_uint32(&mut buf, 0x8 as libc::c_int as u32);
    buf_add_uint32(&mut buf, asec as u32);
    buf_add_uint32(&mut buf, msec as u32);
    err = sftp_request(
        get_conn(sf, path),
        (if sf.is_null() { 9 as libc::c_int } else { 10 as libc::c_int }) as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_open_common(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut err2: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut open_req: *mut request = 0 as *mut request;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut pflags: u32 = 0 as libc::c_int as u32;
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut type_0: u8 = 0;
    let mut wrctr: u64 = 0 as libc::c_int as u64;
    if sshfs.dir_cache != 0 {
        wrctr = cache_get_write_ctr();
    }
    if sshfs.direct_io != 0 {
        (*fi).set_direct_io(1 as libc::c_int as libc::c_uint);
    }
    if (*fi).flags & 0o3 as libc::c_int == 0 as libc::c_int {
        pflags = 0x1 as libc::c_int as u32;
    } else if (*fi).flags & 0o3 as libc::c_int == 0o1 as libc::c_int {
        pflags = 0x2 as libc::c_int as u32;
    } else if (*fi).flags & 0o3 as libc::c_int == 0o2 as libc::c_int {
        pflags = (0x1 as libc::c_int | 0x2 as libc::c_int) as u32;
    } else {
        return -(22 as libc::c_int)
    }
    if (*fi).flags & 0o100 as libc::c_int != 0 {
        pflags |= 0x8 as libc::c_int as libc::c_uint;
    }
    if (*fi).flags & 0o200 as libc::c_int != 0 {
        pflags |= 0x20 as libc::c_int as libc::c_uint;
    }
    if (*fi).flags & 0o1000 as libc::c_int != 0 {
        pflags |= 0x10 as libc::c_int as libc::c_uint;
    }
    if (*fi).flags & 0o2000 as libc::c_int != 0 {
        pflags |= 0x4 as libc::c_int as libc::c_uint;
    }
    sf = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::std::mem::size_of::<sshfs_file>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut sshfs_file;
    list_init(&mut (*sf).write_reqs);
    pthread_cond_init(&mut (*sf).write_finished, 0 as *const pthread_condattr_t);
    (*sf).is_seq = 0 as libc::c_int;
    (*sf).next_pos = 0 as libc::c_int as off_t;
    pthread_mutex_lock(&mut sshfs.lock);
    (*sf).modifver = sshfs.modifver as libc::c_int;
    if sshfs.max_conns > 1 as libc::c_int {
        ce = g_hash_table_lookup(sshfs.conntab, path as gconstpointer)
            as *mut conntab_entry;
        if ce.is_null() {
            ce = g_malloc(::std::mem::size_of::<conntab_entry>() as libc::c_ulong)
                as *mut conntab_entry;
            (*ce).refcount = 0 as libc::c_int as libc::c_uint;
            let ref mut fresh33 = (*ce).conn;
            *fresh33 = get_conn(0 as *const sshfs_file, 0 as *const libc::c_char);
            g_hash_table_insert(
                sshfs.conntab,
                g_strdup(path) as gpointer,
                ce as gpointer,
            );
        }
        let ref mut fresh34 = (*sf).conn;
        *fresh34 = (*ce).conn;
        let ref mut fresh35 = (*ce).refcount;
        *fresh35 = (*fresh35).wrapping_add(1);
        let ref mut fresh36 = (*(*sf).conn).file_count;
        *fresh36 += 1;
        if (*(*sf).conn).file_count > 0 as libc::c_int {} else {
            __assert_fail(
                b"sf->conn->file_count > 0\0" as *const u8 as *const libc::c_char,
                b"../sshfs.c\0" as *const u8 as *const libc::c_char,
                2763 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"int sshfs_open_common(const char *, mode_t, struct fuse_file_info *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        let ref mut fresh37 = (*sf).conn;
        *fresh37 = &mut *(sshfs.conns).offset(0 as libc::c_int as isize) as *mut conn;
        ce = 0 as *mut conntab_entry;
    }
    (*sf).connver = (*(*sf).conn).connver;
    pthread_mutex_unlock(&mut sshfs.lock);
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(&mut buf, pflags);
    buf_add_uint32(&mut buf, 0x4 as libc::c_int as u32);
    buf_add_uint32(&mut buf, mode);
    buf_to_iov(&mut buf, &mut iov);
    sftp_request_send(
        (*sf).conn,
        3 as libc::c_int as u8,
        &mut iov,
        1 as libc::c_int as size_t,
        None,
        None,
        1 as libc::c_int,
        0 as *mut libc::c_void,
        &mut open_req,
    );
    buf_clear(&mut buf);
    buf_add_path(&mut buf, path);
    type_0 = (if sshfs.follow_symlinks != 0 {
        17 as libc::c_int
    } else {
        7 as libc::c_int
    }) as u8;
    err2 = sftp_request(
        (*sf).conn,
        type_0,
        &mut buf,
        105 as libc::c_int as u8,
        &mut outbuf,
    );
    if err2 == 0 {
        err2 = buf_get_attrs(&mut outbuf, &mut stbuf, 0 as *mut libc::c_int);
        buf_free(&mut outbuf);
    }
    err = sftp_request_wait(
        open_req,
        3 as libc::c_int as u8,
        102 as libc::c_int as u8,
        &mut (*sf).handle,
    );
    if err == 0 && err2 != 0 {
        buf_finish(&mut (*sf).handle);
        sftp_request(
            (*sf).conn,
            4 as libc::c_int as u8,
            &mut (*sf).handle,
            0 as libc::c_int as u8,
            0 as *mut buffer,
        );
        buf_free(&mut (*sf).handle);
        err = err2;
    }
    if err == 0 {
        if sshfs.dir_cache != 0 {
            cache_add_attr(path, &mut stbuf, wrctr);
        }
        buf_finish(&mut (*sf).handle);
        (*fi).fh = sf as libc::c_ulong;
    } else {
        if sshfs.dir_cache != 0 {
            cache_invalidate(path);
        }
        if sshfs.max_conns > 1 as libc::c_int {
            pthread_mutex_lock(&mut sshfs.lock);
            let ref mut fresh38 = (*(*sf).conn).file_count;
            *fresh38 -= 1;
            let ref mut fresh39 = (*ce).refcount;
            *fresh39 = (*fresh39).wrapping_sub(1);
            if (*ce).refcount == 0 as libc::c_int as libc::c_uint {
                g_hash_table_remove(sshfs.conntab, path as gconstpointer);
                g_free(ce as gpointer);
            }
            pthread_mutex_unlock(&mut sshfs.lock);
        }
        g_free(sf as gpointer);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_open(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    return sshfs_open_common(path, 0 as libc::c_int as mode_t, fi);
}
unsafe extern "C" fn sshfs_flush(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sf: *mut sshfs_file = get_sshfs_file(fi);
    let mut write_reqs: list_head = list_head {
        prev: 0 as *mut list_head,
        next: 0 as *mut list_head,
    };
    let mut curr_list: *mut list_head = 0 as *mut list_head;
    if sshfs_file_is_conn(sf) == 0 {
        return -(5 as libc::c_int);
    }
    if sshfs.sync_write != 0 {
        return 0 as libc::c_int;
    }
    pthread_mutex_lock(&mut sshfs.lock);
    if list_empty(&mut (*sf).write_reqs) == 0 {
        curr_list = (*sf).write_reqs.prev;
        list_del(&mut (*sf).write_reqs);
        list_init(&mut (*sf).write_reqs);
        list_add(&mut write_reqs, curr_list);
        while list_empty(&mut write_reqs) == 0 {
            pthread_cond_wait(&mut (*sf).write_finished, &mut sshfs.lock);
        }
    }
    err = (*sf).write_error;
    (*sf).write_error = 0 as libc::c_int;
    pthread_mutex_unlock(&mut sshfs.lock);
    return err;
}
unsafe extern "C" fn sshfs_fsync(
    mut path: *const libc::c_char,
    mut isdatasync: libc::c_int,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = sshfs_flush(path, fi);
    if err != 0 {
        return err;
    }
    if sshfs.ext_fsync == 0 {
        return err;
    }
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = get_sshfs_file(fi);
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, b"fsync@openssh.com\0" as *const u8 as *const libc::c_char);
    buf_add_buf(&mut buf, &mut (*sf).handle);
    err = sftp_request(
        (*sf).conn,
        200 as libc::c_int as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_release(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut sf: *mut sshfs_file = get_sshfs_file(fi);
    let mut handle: *mut buffer = &mut (*sf).handle;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    if sshfs_file_is_conn(sf) != 0 {
        sshfs_flush(path, fi);
        sftp_request(
            (*sf).conn,
            4 as libc::c_int as u8,
            handle,
            0 as libc::c_int as u8,
            0 as *mut buffer,
        );
    }
    buf_free(handle);
    chunk_put_locked((*sf).readahead);
    if sshfs.max_conns > 1 as libc::c_int {
        pthread_mutex_lock(&mut sshfs.lock);
        let ref mut fresh40 = (*(*sf).conn).file_count;
        *fresh40 -= 1;
        ce = g_hash_table_lookup(sshfs.conntab, path as gconstpointer)
            as *mut conntab_entry;
        let ref mut fresh41 = (*ce).refcount;
        *fresh41 = (*fresh41).wrapping_sub(1);
        if (*ce).refcount == 0 as libc::c_int as libc::c_uint {
            g_hash_table_remove(sshfs.conntab, path as gconstpointer);
            g_free(ce as gpointer);
        }
        pthread_mutex_unlock(&mut sshfs.lock);
    }
    g_free(sf as gpointer);
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_read_end(mut req: *mut request) {
    let mut rreq: *mut read_req = (*req).data as *mut read_req;
    if (*req).error != 0 {
        (*rreq).res = (*req).error as ssize_t;
    } else if (*req).replied != 0 {
        (*rreq).res = -(5 as libc::c_int) as ssize_t;
        if (*req).reply_type as libc::c_int == 101 as libc::c_int {
            let mut serr: u32 = 0;
            if buf_get_uint32(&mut (*req).reply, &mut serr) != -(1 as libc::c_int) {
                if serr == 1 as libc::c_int as libc::c_uint {
                    (*rreq).res = 0 as libc::c_int as ssize_t;
                } else {
                    (*rreq).res = -sftp_error_to_errno(serr) as ssize_t;
                }
            }
        } else if (*req).reply_type as libc::c_int == 103 as libc::c_int {
            let mut retsize: u32 = 0;
            if buf_get_uint32(&mut (*req).reply, &mut retsize) != -(1 as libc::c_int) {
                if retsize as libc::c_ulong > (*rreq).size {
                    fprintf(
                        stderr,
                        b"long read\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if buf_check_get(&mut (*req).reply, retsize as size_t)
                    != -(1 as libc::c_int)
                {
                    (*rreq).res = retsize as ssize_t;
                    (*rreq).data = (*req).reply;
                    buf_init(&mut (*req).reply, 0 as libc::c_int as size_t);
                }
            }
        } else {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        (*rreq).res = -(5 as libc::c_int) as ssize_t;
    }
    let ref mut fresh42 = (*(*rreq).sio).num_reqs;
    *fresh42 -= 1;
    if (*(*rreq).sio).num_reqs == 0 {
        pthread_cond_broadcast(&mut (*(*rreq).sio).finished);
    }
}
unsafe extern "C" fn sshfs_read_begin(mut req: *mut request) {
    let mut rreq: *mut read_req = (*req).data as *mut read_req;
    let ref mut fresh43 = (*(*rreq).sio).num_reqs;
    *fresh43 += 1;
}
unsafe extern "C" fn sshfs_send_read(
    mut sf: *mut sshfs_file,
    mut size: size_t,
    mut offset: off_t,
) -> *mut read_chunk {
    let mut chunk: *mut read_chunk = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::std::mem::size_of::<read_chunk>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut read_chunk;
    let mut handle: *mut buffer = &mut (*sf).handle;
    pthread_cond_init(&mut (*chunk).sio.finished, 0 as *const pthread_condattr_t);
    list_init(&mut (*chunk).reqs);
    (*chunk).size = size;
    (*chunk).offset = offset;
    (*chunk).refs = 1 as libc::c_int;
    while size != 0 {
        let mut err: libc::c_int = 0;
        let mut buf: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        let mut iov: [iovec; 1] = [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 1];
        let mut rreq: *mut read_req = 0 as *mut read_req;
        let mut bsize: size_t = if size < sshfs.max_read as libc::c_ulong {
            size
        } else {
            sshfs.max_read as libc::c_ulong
        };
        rreq = ({
            let mut __n: gsize = 1 as libc::c_int as gsize;
            let mut __s: gsize = ::std::mem::size_of::<read_req>() as libc::c_ulong;
            let mut __p: gpointer = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = g_malloc0(__n);
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut read_req;
        let ref mut fresh44 = (*rreq).sio;
        *fresh44 = &mut (*chunk).sio;
        (*rreq).size = bsize;
        buf_init(&mut (*rreq).data, 0 as libc::c_int as size_t);
        list_add(&mut (*rreq).list, &mut (*chunk).reqs);
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle);
        buf_add_uint64(&mut buf, offset as u64);
        buf_add_uint32(&mut buf, bsize as u32);
        buf_to_iov(&mut buf, &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize));
        err = sftp_request_send(
            (*sf).conn,
            5 as libc::c_int as u8,
            iov.as_mut_ptr(),
            1 as libc::c_int as size_t,
            Some(sshfs_read_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_read_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            rreq as *mut libc::c_void,
            0 as *mut *mut request,
        );
        buf_free(&mut buf);
        if err != 0 {
            break;
        }
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        offset = (offset as libc::c_ulong).wrapping_add(bsize) as off_t as off_t;
    }
    return chunk;
}
unsafe extern "C" fn wait_chunk(
    mut chunk: *mut read_chunk,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut rreq: *mut read_req = 0 as *mut read_req;
    pthread_mutex_lock(&mut sshfs.lock);
    while (*chunk).sio.num_reqs != 0 {
        pthread_cond_wait(&mut (*chunk).sio.finished, &mut sshfs.lock);
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if (*chunk).sio.error != 0 {
        if (*chunk).sio.error != 1 as libc::c_int {
            res = (*chunk).sio.error;
        }
    } else {
        while list_empty(&mut (*chunk).reqs) == 0 && size != 0 {
            rreq = ({
                let mut __mptr: *const list_head = (*chunk).reqs.prev;
                (__mptr as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                    as *mut read_req
            });
            if (*rreq).res < 0 as libc::c_int as libc::c_long {
                (*chunk).sio.error = (*rreq).res as libc::c_int;
                break;
            } else if (*rreq).res == 0 as libc::c_int as libc::c_long {
                (*chunk).sio.error = 1 as libc::c_int;
                break;
            } else if size < (*rreq).res as size_t {
                buf_get_mem(&mut (*rreq).data, buf as *mut libc::c_void, size);
                let ref mut fresh45 = (*rreq).res;
                *fresh45 = (*fresh45 as libc::c_ulong).wrapping_sub(size) as ssize_t
                    as ssize_t;
                let ref mut fresh46 = (*rreq).size;
                *fresh46 = (*fresh46 as libc::c_ulong).wrapping_sub(size) as size_t
                    as size_t;
                res = (res as libc::c_ulong).wrapping_add(size) as libc::c_int
                    as libc::c_int;
                break;
            } else {
                buf_get_mem(
                    &mut (*rreq).data,
                    buf as *mut libc::c_void,
                    (*rreq).res as size_t,
                );
                res = (res as libc::c_long + (*rreq).res) as libc::c_int;
                if ((*rreq).res as size_t) < (*rreq).size {
                    (*chunk).sio.error = 1 as libc::c_int;
                    break;
                } else {
                    buf = buf.offset((*rreq).res as isize);
                    size = (size as libc::c_ulong)
                        .wrapping_sub((*rreq).res as libc::c_ulong) as size_t as size_t;
                    list_del(&mut (*rreq).list);
                    buf_free(&mut (*rreq).data);
                    g_free(rreq as gpointer);
                }
            }
        }
        if res > 0 as libc::c_int {
            let ref mut fresh47 = (*chunk).offset;
            *fresh47 += res as libc::c_long;
            let ref mut fresh48 = (*chunk).size;
            *fresh48 = (*fresh48 as libc::c_ulong).wrapping_sub(res as libc::c_ulong)
                as size_t as size_t;
        }
    }
    chunk_put_locked(chunk);
    return res;
}
unsafe extern "C" fn sshfs_sync_read(
    mut sf: *mut sshfs_file,
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    chunk = sshfs_send_read(sf, size, offset);
    return wait_chunk(chunk, buf, size);
}
unsafe extern "C" fn submit_read(
    mut sf: *mut sshfs_file,
    mut size: size_t,
    mut offset: off_t,
    mut chunkp: *mut *mut read_chunk,
) {
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    chunk = sshfs_send_read(sf, size, offset);
    pthread_mutex_lock(&mut sshfs.lock);
    (*chunk).modifver = sshfs.modifver;
    chunk_put(*chunkp);
    *chunkp = chunk;
    let ref mut fresh49 = (*chunk).refs;
    *fresh49 += 1;
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn search_read_chunk(
    mut sf: *mut sshfs_file,
    mut offset: off_t,
) -> *mut read_chunk {
    let mut ch: *mut read_chunk = (*sf).readahead;
    if !ch.is_null() && (*ch).offset == offset && (*ch).modifver == sshfs.modifver {
        let ref mut fresh50 = (*ch).refs;
        *fresh50 += 1;
        return ch;
    } else {
        return 0 as *mut read_chunk
    };
}
unsafe extern "C" fn sshfs_async_read(
    mut sf: *mut sshfs_file,
    mut rbuf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut total: size_t = 0 as libc::c_int as size_t;
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    let mut chunk_prev: *mut read_chunk = 0 as *mut read_chunk;
    let mut origsize: size_t = size;
    let mut curr_is_seq: libc::c_int = 0;
    pthread_mutex_lock(&mut sshfs.lock);
    curr_is_seq = (*sf).is_seq;
    (*sf)
        .is_seq = ((*sf).next_pos == offset
        && (*sf).modifver as libc::c_long == sshfs.modifver) as libc::c_int;
    (*sf).next_pos = (offset as libc::c_ulong).wrapping_add(size) as off_t;
    (*sf).modifver = sshfs.modifver as libc::c_int;
    chunk = search_read_chunk(sf, offset);
    pthread_mutex_unlock(&mut sshfs.lock);
    if !chunk.is_null() && (*chunk).size < size {
        chunk_prev = chunk;
        size = (size as libc::c_ulong).wrapping_sub((*chunk).size) as size_t as size_t;
        offset = (offset as libc::c_ulong).wrapping_add((*chunk).size) as off_t as off_t;
        chunk = 0 as *mut read_chunk;
    }
    if chunk.is_null() {
        submit_read(sf, size, offset, &mut chunk);
    }
    if curr_is_seq != 0 && !chunk.is_null() && (*chunk).size <= size {
        submit_read(
            sf,
            origsize,
            (offset as libc::c_ulong).wrapping_add(size) as off_t,
            &mut (*sf).readahead,
        );
    }
    if !chunk_prev.is_null() {
        let mut prev_size: size_t = (*chunk_prev).size;
        res = wait_chunk(chunk_prev, rbuf, prev_size);
        if res < prev_size as libc::c_int {
            chunk_put_locked(chunk);
            return res;
        }
        rbuf = rbuf.offset(res as isize);
        total = (total as libc::c_ulong).wrapping_add(res as libc::c_ulong) as size_t
            as size_t;
    }
    res = wait_chunk(chunk, rbuf, size);
    if res > 0 as libc::c_int {
        total = (total as libc::c_ulong).wrapping_add(res as libc::c_ulong) as size_t
            as size_t;
    }
    if res < 0 as libc::c_int {
        return res;
    }
    return total as libc::c_int;
}
unsafe extern "C" fn sshfs_read(
    mut path: *const libc::c_char,
    mut rbuf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut sf: *mut sshfs_file = get_sshfs_file(fi);
    if sshfs_file_is_conn(sf) == 0 {
        return -(5 as libc::c_int);
    }
    if sshfs.sync_read != 0 {
        return sshfs_sync_read(sf, rbuf, size, offset)
    } else {
        return sshfs_async_read(sf, rbuf, size, offset)
    };
}
unsafe extern "C" fn sshfs_write_begin(mut req: *mut request) {
    let mut sf: *mut sshfs_file = (*req).data as *mut sshfs_file;
    list_add(&mut (*req).list, &mut (*sf).write_reqs);
}
unsafe extern "C" fn sshfs_write_end(mut req: *mut request) {
    let mut serr: u32 = 0;
    let mut sf: *mut sshfs_file = (*req).data as *mut sshfs_file;
    if (*req).error != 0 {
        (*sf).write_error = (*req).error;
    } else if (*req).replied != 0 {
        if (*req).reply_type as libc::c_int != 101 as libc::c_int {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        } else if buf_get_uint32(&mut (*req).reply, &mut serr) != -(1 as libc::c_int)
            && serr != 0 as libc::c_int as libc::c_uint
        {
            (*sf).write_error = -(5 as libc::c_int);
        }
    }
    list_del(&mut (*req).list);
    pthread_cond_broadcast(&mut (*sf).write_finished);
}
unsafe extern "C" fn sshfs_async_write(
    mut sf: *mut sshfs_file,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut handle: *mut buffer = &mut (*sf).handle;
    while err == 0 && size != 0 {
        let mut buf: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        let mut iov: [iovec; 2] = [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 2];
        let mut bsize: size_t = if size < sshfs.max_write as libc::c_ulong {
            size
        } else {
            sshfs.max_write as libc::c_ulong
        };
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle);
        buf_add_uint64(&mut buf, offset as u64);
        buf_add_uint32(&mut buf, bsize as u32);
        buf_to_iov(&mut buf, &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize));
        iov[1 as libc::c_int as usize].iov_base = wbuf as *mut libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = bsize;
        err = sftp_request_send(
            (*sf).conn,
            6 as libc::c_int as u8,
            iov.as_mut_ptr(),
            2 as libc::c_int as size_t,
            Some(sshfs_write_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_write_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            sf as *mut libc::c_void,
            0 as *mut *mut request,
        );
        buf_free(&mut buf);
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        wbuf = wbuf.offset(bsize as isize);
        offset = (offset as libc::c_ulong).wrapping_add(bsize) as off_t as off_t;
    }
    return err;
}
unsafe extern "C" fn sshfs_sync_write_begin(mut req: *mut request) {
    let mut sio: *mut sshfs_io = (*req).data as *mut sshfs_io;
    let ref mut fresh51 = (*sio).num_reqs;
    *fresh51 += 1;
}
unsafe extern "C" fn sshfs_sync_write_end(mut req: *mut request) {
    let mut serr: u32 = 0;
    let mut sio: *mut sshfs_io = (*req).data as *mut sshfs_io;
    if (*req).error != 0 {
        (*sio).error = (*req).error;
    } else if (*req).replied != 0 {
        if (*req).reply_type as libc::c_int != 101 as libc::c_int {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        } else if buf_get_uint32(&mut (*req).reply, &mut serr) != -(1 as libc::c_int)
            && serr != 0 as libc::c_int as libc::c_uint
        {
            (*sio).error = -(5 as libc::c_int);
        }
    }
    let ref mut fresh52 = (*sio).num_reqs;
    *fresh52 -= 1;
    if (*sio).num_reqs == 0 {
        pthread_cond_broadcast(&mut (*sio).finished);
    }
}
unsafe extern "C" fn sshfs_sync_write(
    mut sf: *mut sshfs_file,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut handle: *mut buffer = &mut (*sf).handle;
    let mut sio: sshfs_io = {
        let mut init = sshfs_io {
            num_reqs: 0 as libc::c_int,
            finished: pthread_cond_t {
                __data: __pthread_cond_s {
                    __wseq: __atomic_wide_counter {
                        __value64: 0,
                    },
                    __g1_start: __atomic_wide_counter {
                        __value64: 0,
                    },
                    __g_refs: [0; 2],
                    __g_size: [0; 2],
                    __g1_orig_size: 0,
                    __wrefs: 0,
                    __g_signals: [0; 2],
                },
            },
            error: 0 as libc::c_int,
        };
        init
    };
    pthread_cond_init(&mut sio.finished, 0 as *const pthread_condattr_t);
    while err == 0 && size != 0 {
        let mut buf: buffer = buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        };
        let mut iov: [iovec; 2] = [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 2];
        let mut bsize: size_t = if size < sshfs.max_write as libc::c_ulong {
            size
        } else {
            sshfs.max_write as libc::c_ulong
        };
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle);
        buf_add_uint64(&mut buf, offset as u64);
        buf_add_uint32(&mut buf, bsize as u32);
        buf_to_iov(&mut buf, &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize));
        iov[1 as libc::c_int as usize].iov_base = wbuf as *mut libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = bsize;
        err = sftp_request_send(
            (*sf).conn,
            6 as libc::c_int as u8,
            iov.as_mut_ptr(),
            2 as libc::c_int as size_t,
            Some(sshfs_sync_write_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_sync_write_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            &mut sio as *mut sshfs_io as *mut libc::c_void,
            0 as *mut *mut request,
        );
        buf_free(&mut buf);
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        wbuf = wbuf.offset(bsize as isize);
        offset = (offset as libc::c_ulong).wrapping_add(bsize) as off_t as off_t;
    }
    pthread_mutex_lock(&mut sshfs.lock);
    while sio.num_reqs != 0 {
        pthread_cond_wait(&mut sio.finished, &mut sshfs.lock);
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if err == 0 {
        err = sio.error;
    }
    return err;
}
unsafe extern "C" fn sshfs_write(
    mut path: *const libc::c_char,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sf: *mut sshfs_file = get_sshfs_file(fi);
    if sshfs_file_is_conn(sf) == 0 {
        return -(5 as libc::c_int);
    }
    sshfs_inc_modifver();
    if sshfs.sync_write == 0 && (*sf).write_error == 0 {
        err = sshfs_async_write(sf, wbuf, size, offset);
    } else {
        err = sshfs_sync_write(sf, wbuf, size, offset);
    }
    return if err != 0 { err } else { size as libc::c_int };
}
unsafe extern "C" fn sshfs_ext_statvfs(
    mut path: *const libc::c_char,
    mut stbuf: *mut statvfs,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(
        &mut buf,
        b"statvfs@openssh.com\0" as *const u8 as *const libc::c_char,
    );
    buf_add_path(&mut buf, path);
    err = sftp_request(
        get_conn(0 as *const sshfs_file, 0 as *const libc::c_char),
        200 as libc::c_int as u8,
        &mut buf,
        201 as libc::c_int as u8,
        &mut outbuf,
    );
    if err == 0 {
        if buf_get_statvfs(&mut outbuf, stbuf) == -(1 as libc::c_int) {
            err = -(5 as libc::c_int);
        }
        buf_free(&mut outbuf);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_statfs(
    mut path: *const libc::c_char,
    mut buf: *mut statvfs,
) -> libc::c_int {
    if sshfs.ext_statvfs != 0 {
        return sshfs_ext_statvfs(path, buf);
    }
    (*buf).f_namemax = 255 as libc::c_int as libc::c_ulong;
    (*buf).f_bsize = sshfs.blksize as libc::c_ulong;
    (*buf).f_frsize = (*buf).f_bsize;
    let ref mut fresh53 = (*buf).f_bavail;
    *fresh53 = (1000 as libc::c_ulonglong)
        .wrapping_mul(1024 as libc::c_int as libc::c_ulonglong)
        .wrapping_mul(1024 as libc::c_int as libc::c_ulonglong)
        .wrapping_mul(1024 as libc::c_int as libc::c_ulonglong)
        .wrapping_div((*buf).f_frsize as libc::c_ulonglong) as __fsblkcnt64_t;
    let ref mut fresh54 = (*buf).f_bfree;
    *fresh54 = *fresh53;
    (*buf).f_blocks = *fresh54;
    let ref mut fresh55 = (*buf).f_ffree;
    *fresh55 = 1000000000 as libc::c_int as __fsfilcnt64_t;
    (*buf).f_files = *fresh55;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_create(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    if sshfs.createmode_workaround != 0 {
        mode = 0 as libc::c_int as mode_t;
    }
    return sshfs_open_common(path, mode, fi);
}
unsafe extern "C" fn sshfs_truncate(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    if !fi.is_null() {
        sf = get_sshfs_file(fi);
        if sshfs_file_is_conn(sf) == 0 {
            return -(5 as libc::c_int);
        }
    }
    sshfs_inc_modifver();
    if sshfs.truncate_workaround != 0 {
        return sshfs_truncate_workaround(path, size, fi);
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if !sf.is_null() {
        buf_add_buf(&mut buf, &mut (*sf).handle);
    } else {
        buf_add_path(&mut buf, path);
    }
    buf_add_uint32(&mut buf, 0x1 as libc::c_int as u32);
    buf_add_uint64(&mut buf, size as u64);
    err = sftp_request(
        get_conn(sf, path),
        (if sf.is_null() { 9 as libc::c_int } else { 10 as libc::c_int }) as u8,
        &mut buf,
        101 as libc::c_int as u8,
        0 as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_getattr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut u8,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    if !fi.is_null() && sshfs.fstat_workaround == 0 {
        sf = get_sshfs_file(fi);
        if sshfs_file_is_conn(sf) == 0 {
            return -(5 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf.is_null() {
        buf_add_path(&mut buf, path);
        err = sftp_request(
            get_conn(sf, path),
            (if sshfs.follow_symlinks != 0 {
                17 as libc::c_int
            } else {
                7 as libc::c_int
            }) as u8,
            &mut buf,
            105 as libc::c_int as u8,
            &mut outbuf,
        );
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle);
        err = sftp_request(
            (*sf).conn,
            8 as libc::c_int as u8,
            &mut buf,
            105 as libc::c_int as u8,
            &mut outbuf,
        );
    }
    if err == 0 {
        err = buf_get_attrs(&mut outbuf, stbuf, 0 as *mut libc::c_int);
        buf_free(&mut outbuf);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_truncate_zero(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut fi: fuse_file_info = Default::default();

    fi.flags = 0o1 as libc::c_int | 0o1000 as libc::c_int;
    err = sshfs_open(path, &mut fi);
    if err == 0 {
        sshfs_release(path, &mut fi);
    }
    return err;
}
unsafe extern "C" fn calc_buf_size(mut size: off_t, mut offset: off_t) -> size_t {
    return (if (offset + sshfs.max_read as libc::c_long) < size {
        sshfs.max_read as libc::c_long
    } else {
        size - offset
    }) as size_t;
}
unsafe extern "C" fn sshfs_truncate_shrink(
    mut path: *const libc::c_char,
    mut size: off_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: off_t = 0;
    let mut fi: fuse_file_info = Default::default();
    data = calloc(size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    if data.is_null() {
        return -(12 as libc::c_int);
    }
    fi.flags = 0 as libc::c_int;
    res = sshfs_open(path, &mut fi);
    if !(res != 0) {
        offset = 0 as libc::c_int as off_t;
        while offset < size {
            let mut bufsize: size_t = calc_buf_size(size, offset);
            res = sshfs_read(
                path,
                data.offset(offset as isize),
                bufsize,
                offset,
                &mut fi,
            );
            if res <= 0 as libc::c_int {
                break;
            }
            offset += res as libc::c_long;
        }
        sshfs_release(path, &mut fi);
        if !(res < 0 as libc::c_int) {
            fi.flags = 0o1 as libc::c_int | 0o1000 as libc::c_int;
            res = sshfs_open(path, &mut fi);
            if !(res != 0) {
                offset = 0 as libc::c_int as off_t;
                while offset < size {
                    let mut bufsize_0: size_t = calc_buf_size(size, offset);
                    res = sshfs_write(
                        path,
                        data.offset(offset as isize),
                        bufsize_0,
                        offset,
                        &mut fi,
                    );
                    if res < 0 as libc::c_int {
                        break;
                    }
                    offset += res as libc::c_long;
                }
                if res >= 0 as libc::c_int {
                    res = sshfs_flush(path, &mut fi);
                }
                sshfs_release(path, &mut fi);
            }
        }
    }
    free(data as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn sshfs_truncate_extend(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut c: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut tmpfi: fuse_file_info = Default::default();
    let mut openfi: *mut fuse_file_info = fi;
    if fi.is_null() {
        openfi = &mut tmpfi;
        (*openfi).flags = 0o1 as libc::c_int;
        res = sshfs_open(path, openfi);
        if res != 0 {
            return res;
        }
    }
    res = sshfs_write(
        path,
        &mut c,
        1 as libc::c_int as size_t,
        size - 1 as libc::c_int as libc::c_long,
        openfi,
    );
    if res == 1 as libc::c_int {
        res = sshfs_flush(path, openfi);
    }
    if fi.is_null() {
        sshfs_release(path, openfi);
    }
    return res;
}
unsafe extern "C" fn sshfs_truncate_workaround(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    if size == 0 as libc::c_int as libc::c_long {
        return sshfs_truncate_zero(path)
    } else {
        let mut stbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut err: libc::c_int = 0;
        err = sshfs_getattr(path, &mut stbuf, fi);
        if err != 0 {
            return err;
        }
        if stbuf.st_size == size {
            return 0 as libc::c_int
        } else if stbuf.st_size > size {
            return sshfs_truncate_shrink(path, size)
        } else {
            return sshfs_truncate_extend(path, size, fi)
        }
    };
}
static mut sshfs_oper: fuse_operations = unsafe {
    {
        let mut init = fuse_operations {
            getattr: Some(
                sshfs_getattr
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            readlink: Some(
                sshfs_readlink
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
            mknod: Some(
                sshfs_mknod
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        mode_t,
                        dev_t,
                    ) -> libc::c_int,
            ),
            mkdir: Some(
                sshfs_mkdir
                    as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
            ),
            unlink: Some(
                sshfs_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
            rmdir: Some(
                sshfs_rmdir as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
            symlink: Some(
                sshfs_symlink
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
            rename: Some(
                sshfs_rename
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            link: Some(
                sshfs_link
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
            chmod: Some(
                sshfs_chmod
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        mode_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            chown: Some(
                sshfs_chown
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        uid_t,
                        gid_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            truncate: Some(
                sshfs_truncate
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            open: Some(
                sshfs_open
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            read: Some(
                sshfs_read
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            write: Some(
                sshfs_write
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            statfs: Some(
                sshfs_statfs
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut statvfs,
                    ) -> libc::c_int,
            ),
            flush: Some(
                sshfs_flush
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            release: Some(
                sshfs_release
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            fsync: Some(
                sshfs_fsync
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            setxattr: None,
            getxattr: None,
            listxattr: None,
            removexattr: None,
            opendir: Some(
                sshfs_opendir
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            readdir: Some(
                sshfs_readdir
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_void,
                        fuse_fill_dir_t,
                        off_t,
                        *mut fuse_file_info,
                        fuse_readdir_flags,
                    ) -> libc::c_int,
            ),
            releasedir: Some(
                sshfs_releasedir
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            fsyncdir: None,
            init: Some(
                sshfs_init
                    as unsafe extern "C" fn(
                        *mut fuse_conn_info,
                        *mut fuse_config,
                    ) -> *mut libc::c_void,
            ),
            destroy: None,
            access: Some(
                sshfs_access
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            create: Some(
                sshfs_create
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        mode_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            lock: None,
            utimens: Some(
                sshfs_utimens
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const timespec,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            bmap: None,
            ioctl: None,
            poll: None,
            write_buf: None,
            read_buf: None,
            flock: None,
            fallocate: None,
            copy_file_range: None,
            lseek: None,
        };
        init
    }
};
unsafe extern "C" fn sshfs_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    match key {
        -1 => {
            if ssh_opt::is_ssh_opt(arg) != 0 {
                tmp = g_strdup_printf(
                    b"-o%s\0" as *const u8 as *const libc::c_char,
                    arg,
                );
                ssh_add_arg(tmp);
                g_free(tmp as gpointer);
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        -2 => {
            if (sshfs.host).is_null() && !(strchr(arg, ':' as i32)).is_null() {
                sshfs.host = strdup(arg);
                return 0 as libc::c_int;
            } else {
                if (sshfs.mountpoint).is_null() {
                    let mut fd: libc::c_int = 0;
                    let mut len: libc::c_int = 0;
                    if sscanf(
                        arg,
                        b"/dev/fd/%u%n\0" as *const u8 as *const libc::c_char,
                        &mut fd as *mut libc::c_int,
                        &mut len as *mut libc::c_int,
                    ) == 1 as libc::c_int && len as libc::c_ulong == strlen(arg)
                    {
                        sshfs.mountpoint = strdup(arg);
                    } else {
                        sshfs.mountpoint = realpath(arg, 0 as *mut libc::c_char);
                    }
                    if (sshfs.mountpoint).is_null() {
                        fprintf(
                            stderr,
                            b"sshfs: bad mount point `%s': %s\n\0" as *const u8
                                as *const libc::c_char,
                            arg,
                            strerror(*__errno_location()),
                        );
                        return -(1 as libc::c_int);
                    }
                    return 0 as libc::c_int;
                }
            }
            fprintf(
                stderr,
                b"sshfs: invalid argument `%s'\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
            return -(1 as libc::c_int);
        }
        0 => {
            tmp = g_strdup_printf(
                b"-oPort=%s\0" as *const u8 as *const libc::c_char,
                arg.offset(2 as libc::c_int as isize),
            );
            ssh_add_arg(tmp);
            g_free(tmp as gpointer);
            return 0 as libc::c_int;
        }
        1 => {
            ssh_add_arg(b"-oCompression=yes\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        2 => {
            tmp = g_strdup_printf(
                b"-F%s\0" as *const u8 as *const libc::c_char,
                arg.offset(2 as libc::c_int as isize),
            );
            ssh_add_arg(tmp);
            g_free(tmp as gpointer);
            return 0 as libc::c_int;
        }
        _ => {
            fprintf(stderr, b"internal error\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    };
}
unsafe extern "C" fn workaround_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    fprintf(
        stderr,
        b"unknown workaround: '%s'\n\0" as *const u8 as *const libc::c_char,
        arg,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_workarounds() -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut argv0: [libc::c_char; 1] = *::std::mem::transmute::<
        &[u8; 1],
        &mut [libc::c_char; 1],
    >(b"\0");
    let mut argv1: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"-o\0");
    let mut argv: [*mut libc::c_char; 4] = [
        argv0.as_mut_ptr(),
        argv1.as_mut_ptr(),
        sshfs.workarounds,
        0 as *mut libc::c_char,
    ];
    let mut args: fuse_args = {
        let mut init = fuse_args {
            argc: 3 as libc::c_int,
            argv: argv.as_mut_ptr(),
            allocated: 0 as libc::c_int,
        };
        init
    };
    let mut s: *mut libc::c_char = sshfs.workarounds;
    if s.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        s = strchr(s, ':' as i32);
        if s.is_null() {
            break;
        }
        *s = ',' as i32 as libc::c_char;
    }
    res = fuse_opt_parse(
        &mut args,
        &mut sshfs as *mut sshfs as *mut libc::c_void,
        workaround_opts.as_mut_ptr() as *const fuse_opt,
        Some(
            workaround_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    );
    fuse_opt_free_args(&mut args);
    return res;
}
unsafe extern "C" fn read_password() -> libc::c_int {
    let mut size: libc::c_int = getpagesize();
    let mut max_password: libc::c_int = if (1024 as libc::c_int)
        < size - 1 as libc::c_int
    {
        1024 as libc::c_int
    } else {
        size - 1 as libc::c_int
    };
    let mut n: libc::c_int = 0;
    sshfs
        .password = mmap(
        0 as *mut libc::c_void,
        size as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x2 as libc::c_int | 0x20 as libc::c_int | 0x2000 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off64_t,
    ) as *mut libc::c_char;
    if sshfs.password == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
        perror(
            b"Failed to allocate locked page for password\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mlock(sshfs.password as *const libc::c_void, size as size_t) != 0 as libc::c_int {
        memset(
            sshfs.password as *mut libc::c_void,
            0 as libc::c_int,
            size as libc::c_ulong,
        );
        munmap(sshfs.password as *mut libc::c_void, size as size_t);
        sshfs.password = 0 as *mut libc::c_char;
        perror(
            b"Failed to allocate locked page for password\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    while n < max_password {
        let mut res: libc::c_int = 0;
        res = read(
            0 as libc::c_int,
            &mut *(sshfs.password).offset(n as isize) as *mut libc::c_char
                as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"Reading password\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            *(sshfs.password).offset(n as isize) = '\n' as i32 as libc::c_char;
            break;
        } else {
            if *(sshfs.password).offset(n as isize) as libc::c_int == '\n' as i32 {
                break;
            }
            n += 1;
        }
    }
    if n == max_password {
        fprintf(stderr, b"Password too long\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    *(sshfs.password)
        .offset((n + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    ssh_add_arg(b"-oNumberOfPasswordPrompts=1\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tokenize_on_space(mut str: *mut libc::c_char) -> *mut libc::c_char {
    static mut pos: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    if !str.is_null() {
        pos = str;
    }
    if pos.is_null() {
        return 0 as *mut libc::c_char;
    }
    while *pos as libc::c_int == ' ' as i32 {
        pos = pos.offset(1);
    }
    start = pos;
    while !pos.is_null() && *pos as libc::c_int != '\0' as i32 {
        if *pos as libc::c_int == ' ' as i32
            && *pos.offset(-(1 as libc::c_int as isize)) as libc::c_int != '\\' as i32
        {
            break;
        }
        pos = pos.offset(1);
    }
    if *pos as libc::c_int == '\0' as i32 {
        pos = 0 as *mut libc::c_char;
    } else {
        *pos = '\0' as i32 as libc::c_char;
        pos = pos.offset(1);
    }
    return start;
}
unsafe extern "C" fn set_ssh_command() {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    token = tokenize_on_space(sshfs.ssh_command);
    while !token.is_null() {
        if i == 0 as libc::c_int {
            replace_arg(
                &mut *(sshfs.ssh_args.argv).offset(0 as libc::c_int as isize),
                token,
            );
        } else if fuse_opt_insert_arg(&mut sshfs.ssh_args, i, token)
            == -(1 as libc::c_int)
        {
            _exit(1 as libc::c_int);
        }
        i += 1;
        token = tokenize_on_space(0 as *mut libc::c_char);
    }
}
unsafe extern "C" fn find_base_path() -> *mut libc::c_char {
    let mut s: *mut libc::c_char = sshfs.host;
    let mut d: *mut libc::c_char = s;
    while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
        if *s as libc::c_int == '[' as i32 {
            s = s.offset(1);
            while *s as libc::c_int != ']' as i32 {
                if *s == 0 {
                    fprintf(
                        stderr,
                        b"missing ']' in hostname\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                let fresh56 = d;
                d = d.offset(1);
                *fresh56 = *s;
                s = s.offset(1);
            }
        } else {
            let fresh57 = d;
            d = d.offset(1);
            *fresh57 = *s;
        }
        s = s.offset(1);
    }
    let fresh58 = d;
    d = d.offset(1);
    *fresh58 = '\0' as i32 as libc::c_char;
    s = s.offset(1);
    return s;
}


fn add_comma_escaped_hostname(args: *mut fuse_args, hostname: *const libc::c_char) {
    let hostname = unsafe { CStr::from_ptr(hostname) };
    let hostname = hostname.to_string_lossy();
    let mut buf = String::new();

    buf.push_str("-osubtype=sshfs,fsname=");

    for ch in hostname.chars() {
        if ch == '\\' || ch == ',' {
            buf.push('\\');
            buf.push(ch);
        } else {
            buf.push(ch);
        }
    }

    let cstring_arg = match CString::new(buf) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error allocating string: {}", err);
            std::process::exit(1);
        }
    };
    unsafe {
        libfuse_sys::fuse::fuse_opt_insert_arg(args, 1, cstring_arg.as_ptr());
    }
}




unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut args: fuse_args = {
        let mut init = fuse_args {
            argc,
            argv,
            allocated: 0 as libc::c_int,
        };
        init
    };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sftp_server: *const libc::c_char = 0 as *const libc::c_char;
    let mut fuse: *mut fuse = 0 as *mut fuse;
    let mut se: *mut fuse_session = 0 as *mut fuse_session;
    let mut i: libc::c_int = 0;
    sshfs.blksize = 4096 as libc::c_int as libc::c_uint;
    sshfs.max_read = 32768 as libc::c_int as libc::c_uint;
    sshfs.max_write = 32768 as libc::c_int as libc::c_uint;
    sshfs.rename_workaround = 0 as libc::c_int;
    sshfs.renamexdev_workaround = 0 as libc::c_int;
    sshfs.truncate_workaround = 0 as libc::c_int;
    sshfs.buflimit_workaround = 0 as libc::c_int;
    sshfs.createmode_workaround = 0 as libc::c_int;
    sshfs.ssh_ver = 2 as libc::c_int as libc::c_uint;
    sshfs.progname = *argv.offset(0 as libc::c_int as isize);
    sshfs.max_conns = 1 as libc::c_int;
    sshfs.ptyfd = -(1 as libc::c_int);
    sshfs.dir_cache = 1 as libc::c_int;
    sshfs.show_help = 0 as libc::c_int;
    sshfs.show_version = 0 as libc::c_int;
    sshfs.singlethread = 0 as libc::c_int;
    sshfs.foreground = 0 as libc::c_int;
    sshfs.ptypassivefd = -(1 as libc::c_int);
    sshfs.delay_connect = 0 as libc::c_int;
    sshfs.passive = 0 as libc::c_int;
    sshfs.detect_uid = 0 as libc::c_int;
    if strcmp(
        b"none\0" as *const u8 as *const libc::c_char,
        b"none\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        sshfs.idmap = IDMAP_NONE as libc::c_int;
    } else if strcmp(
        b"none\0" as *const u8 as *const libc::c_char,
        b"user\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        sshfs.idmap = IDMAP_USER as libc::c_int;
    } else {
        fprintf(
            stderr,
            b"bad idmap default value built into sshfs; assuming none (bad logic in configure script?)\n\0"
                as *const u8 as *const libc::c_char,
        );
        sshfs.idmap = IDMAP_NONE as libc::c_int;
    }
    sshfs.nomap = NOMAP_ERROR as libc::c_int;
    ssh_add_arg(b"ssh\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-x\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-a\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-oClearAllForwardings=yes\0" as *const u8 as *const libc::c_char);
    if fuse_opt_parse(
        &mut args,
        &mut sshfs as *mut sshfs as *mut libc::c_void,
        sshfs_opts.as_mut_ptr() as *const fuse_opt,
        Some(
            sshfs_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    ) == -(1 as libc::c_int) || parse_workarounds() == -(1 as libc::c_int)
    {
        exit(1 as libc::c_int);
    }
    if sshfs.show_version != 0 {
        printf(
            b"SSHFS version %s\n\0" as *const u8 as *const libc::c_char,
            b"3.7.3\0" as *const u8 as *const libc::c_char,
        );
        printf(
            b"FUSE library version %s\n\0" as *const u8 as *const libc::c_char,
            fuse_pkgversion(),
        );
        fuse_lowlevel_version();
        exit(0 as libc::c_int);
    }
    if sshfs.show_help != 0 {
        help::show_help(&mut args);
    } else {
        if (sshfs.host).is_null() {
            fprintf(stderr, b"missing host\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                stderr,
                b"see `%s -h' for usage\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        } else {
            if (sshfs.mountpoint).is_null() {
                fprintf(
                    stderr,
                    b"error: no mountpoint specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"see `%s -h' for usage\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if sshfs.idmap == IDMAP_USER as libc::c_int {
        sshfs.detect_uid = 1 as libc::c_int;
    } else if sshfs.idmap == IDMAP_FILE as libc::c_int {
        sshfs.uid_map = 0 as *mut GHashTable;
        sshfs.gid_map = 0 as *mut GHashTable;
        sshfs.r_uid_map = 0 as *mut GHashTable;
        sshfs.r_gid_map = 0 as *mut GHashTable;

        id_map::handle_id_maps(sshfs.uid_file, sshfs.gid_file);
    }
    free(sshfs.uid_file as *mut libc::c_void);
    free(sshfs.gid_file as *mut libc::c_void);
    if sshfs.debug != 0 {
        fprintf(
            stderr,
            b"SSHFS version %s\n\0" as *const u8 as *const libc::c_char,
            b"3.7.3\0" as *const u8 as *const libc::c_char,
        );
    }
    if sshfs.passive != 0 {
        sshfs.foreground = 1 as libc::c_int;
    }
    if sshfs.passive != 0 && sshfs.password_stdin != 0 {
        fprintf(
            stderr,
            b"the password_stdin and passive options cannot both be specified\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if sshfs.password_stdin != 0 {
        res = read_password();
        if res == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
    }
    if sshfs.debug != 0 {
        sshfs.foreground = 1 as libc::c_int;
    }
    if sshfs.buflimit_workaround != 0 {
        sshfs.max_outstanding_len = 8388608 as libc::c_int as libc::c_uint;
    } else {
        sshfs.max_outstanding_len = !(0 as libc::c_int) as libc::c_uint;
    }
    if sshfs.max_conns > 1 as libc::c_int {
        if sshfs.buflimit_workaround != 0 {
            fprintf(
                stderr,
                b"buflimit workaround is not supported with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if sshfs.password_stdin != 0 {
            fprintf(
                stderr,
                b"password_stdin option cannot be specified with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if sshfs.passive != 0 {
            fprintf(
                stderr,
                b"passive option cannot be specified with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    } else if sshfs.max_conns <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"value of max_conns option must be at least 1\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    sshfs
        .conns = ({
        let mut __n: gsize = sshfs.max_conns as gsize;
        let mut __s: gsize = ::std::mem::size_of::<conn>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut conn;
    i = 0 as libc::c_int;
    while i < sshfs.max_conns {
        (*(sshfs.conns).offset(i as isize)).rfd = -(1 as libc::c_int);
        (*(sshfs.conns).offset(i as isize)).wfd = -(1 as libc::c_int);
        i += 1;
    }
    fsname = g_strdup(sshfs.host);
    sshfs.base_path = g_strdup(find_base_path());
    if !(sshfs.ssh_command).is_null() {
        set_ssh_command();
    }
    tmp = g_strdup_printf(b"-%i\0" as *const u8 as *const libc::c_char, sshfs.ssh_ver);
    ssh_add_arg(tmp);
    g_free(tmp as gpointer);
    ssh_add_arg(sshfs.host);
    if !(sshfs.sftp_server).is_null() {
        sftp_server = sshfs.sftp_server;
    } else if sshfs.ssh_ver == 1 as libc::c_int as libc::c_uint {
        sftp_server = b"/usr/lib/sftp-server\0" as *const u8 as *const libc::c_char;
    } else {
        sftp_server = b"sftp\0" as *const u8 as *const libc::c_char;
    }
    if sshfs.ssh_ver != 1 as libc::c_int as libc::c_uint
        && (strchr(sftp_server, '/' as i32)).is_null()
    {
        ssh_add_arg(b"-s\0" as *const u8 as *const libc::c_char);
    }
    ssh_add_arg(sftp_server);
    free(sshfs.sftp_server as *mut libc::c_void);
    res = cache_parse_options(&mut args);
    if res == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    sshfs.randseed = time(0 as *mut time_t) as libc::c_uint;
    if sshfs.max_read > 65536 as libc::c_int as libc::c_uint {
        sshfs.max_read = 65536 as libc::c_int as libc::c_uint;
    }
    if sshfs.max_write > 65536 as libc::c_int as libc::c_uint {
        sshfs.max_write = 65536 as libc::c_int as libc::c_uint;
    }

    add_comma_escaped_hostname(&mut args, sshfs.host);


    if sshfs.dir_cache != 0 {
        sshfs.op = cache_wrap(&mut sshfs_oper);
    } else {
        sshfs.op = &mut sshfs_oper;
    }
    fuse = fuse_new(
        &mut args,
        sshfs.op,
        ::std::mem::size_of::<fuse_operations>() as libc::c_ulong,
        0 as *mut libc::c_void,
    );
    if fuse.is_null() {
        exit(1 as libc::c_int);
    }
    se = fuse_get_session(fuse);
    res = fuse_set_signal_handlers(se);
    if res != 0 as libc::c_int {
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    res = fuse_mount(fuse, sshfs.mountpoint);
    if res != 0 as libc::c_int {
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    res = fcntl(fuse_session_fd(se), 2 as libc::c_int, 1 as libc::c_int);
    if res == -(1 as libc::c_int) {
        perror(
            b"WARNING: failed to set FD_CLOEXEC on fuse device\0" as *const u8
                as *const libc::c_char,
        );
    }
    res = ssh::ssh_connect();
    if res == -(1 as libc::c_int) {
        fuse_unmount(fuse);
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    res = fuse_daemonize(sshfs.foreground);
    if res == -(1 as libc::c_int) {
        fuse_unmount(fuse);
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    if sshfs.singlethread != 0 {
        res = fuse_loop(fuse);
    } else {
        res = fuse_loop_mt_31(fuse, 0 as libc::c_int);
    }
    if res != 0 as libc::c_int {
        res = 1 as libc::c_int;
    } else {
        res = 0 as libc::c_int;
    }
    fuse_remove_signal_handlers(se);
    fuse_unmount(fuse);
    fuse_destroy(fuse);
    if sshfs.debug != 0 {
        let mut avg_rtt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if sshfs.num_sent != 0 {
            avg_rtt = (sshfs.total_rtt).wrapping_div(sshfs.num_sent) as libc::c_uint;
        }
        if sshfs.debug != 0 {
            fprintf(
                stderr,
                b"\nsent:               %llu messages, %llu bytes\nreceived:           %llu messages, %llu bytes\nrtt min/max/avg:    %ums/%ums/%ums\nnum connect:        %u\n\0"
                    as *const u8 as *const libc::c_char,
                sshfs.num_sent as libc::c_ulonglong,
                sshfs.bytes_sent as libc::c_ulonglong,
                sshfs.num_received as libc::c_ulonglong,
                sshfs.bytes_received as libc::c_ulonglong,
                sshfs.min_rtt,
                sshfs.max_rtt,
                avg_rtt,
                sshfs.num_connect,
            );
        }
    }
    fuse_opt_free_args(&mut args);
    fuse_opt_free_args(&mut sshfs.ssh_args);
    free(sshfs.directport as *mut libc::c_void);
    return res;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    sshfs_opts = [
        {
            let mut init = fuse_opt {
                templ: b"directport=%s\0" as *const u8 as *const libc::c_char,
                offset: 0 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"ssh_command=%s\0" as *const u8 as *const libc::c_char,
                offset: 8 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sftp_server=%s\0" as *const u8 as *const libc::c_char,
                offset: 16 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_read=%u\0" as *const u8 as *const libc::c_char,
                offset: 184 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_write=%u\0" as *const u8 as *const libc::c_char,
                offset: 188 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"ssh_protocol=%u\0" as *const u8 as *const libc::c_char,
                offset: 192 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-1\0" as *const u8 as *const libc::c_char,
                offset: 192 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"workaround=%s\0" as *const u8 as *const libc::c_char,
                offset: 48 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=none\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: IDMAP_NONE as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=user\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: IDMAP_USER as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=file\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: IDMAP_FILE as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"uidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: 136 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"gidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: 144 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nomap=ignore\0" as *const u8 as *const libc::c_char,
                offset: 104 as libc::c_ulong,
                value: NOMAP_IGNORE as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nomap=error\0" as *const u8 as *const libc::c_char,
                offset: 104 as libc::c_ulong,
                value: NOMAP_ERROR as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_sync\0" as *const u8 as *const libc::c_char,
                offset: 196 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"no_readahead\0" as *const u8 as *const libc::c_char,
                offset: 200 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sync_readdir\0" as *const u8 as *const libc::c_char,
                offset: 204 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_debug\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_verbose\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"reconnect\0" as *const u8 as *const libc::c_char,
                offset: 224 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"transform_symlinks\0" as *const u8 as *const libc::c_char,
                offset: 84 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"follow_symlinks\0" as *const u8 as *const libc::c_char,
                offset: 88 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"no_check_root\0" as *const u8 as *const libc::c_char,
                offset: 92 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"password_stdin\0" as *const u8 as *const libc::c_char,
                offset: 440 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"delay_connect\0" as *const u8 as *const libc::c_char,
                offset: 228 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"slave\0" as *const u8 as *const libc::c_char,
                offset: 232 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"passive\0" as *const u8 as *const libc::c_char,
                offset: 232 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"disable_hardlink\0" as *const u8 as *const libc::c_char,
                offset: 108 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dir_cache=yes\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dir_cache=no\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"direct_io\0" as *const u8 as *const libc::c_char,
                offset: 208 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_conns=%u\0" as *const u8 as *const libc::c_char,
                offset: 316 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-h\0" as *const u8 as *const libc::c_char,
                offset: 120 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"--help\0" as *const u8 as *const libc::c_char,
                offset: 120 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-V\0" as *const u8 as *const libc::c_char,
                offset: 116 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"--version\0" as *const u8 as *const libc::c_char,
                offset: 116 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-d\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"debug\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-v\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"verbose\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-f\0" as *const u8 as *const libc::c_char,
                offset: 220 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-s\0" as *const u8 as *const libc::c_char,
                offset: 124 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-p \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_PORT as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-C\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_COMPRESS as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-F \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_CONFIGFILE as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache=yes\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache=no\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"writeback_cache=no\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"unreliable_append\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"auto\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"noauto\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"user\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nouser\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"users\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"_netdev\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: 0 as *const libc::c_char,
                offset: 0 as libc::c_int as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
