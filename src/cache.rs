use ::libc;

use libfuse_sys::fuse::fuse_args;

extern "C" {
    pub type fuse_pollhandle;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GHashTable;
    fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut libc::c_void,
        opts: *const fuse_opt,
        proc_0: fuse_opt_proc_t,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_foreach_remove(
        hash_table: *mut GHashTable,
        func: GHRFunc,
        user_data: gpointer,
    ) -> guint;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_free(mem: gpointer);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_opt {
    pub templ: *const libc::c_char,
    pub offset: libc::c_ulong,
    pub value: libc::c_int,
}
pub type fuse_opt_proc_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        libc::c_int,
        *mut fuse_args,
    ) -> libc::c_int,
>;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: libc::c_int,
    #[bitfield(name = "writepage", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "direct_io", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "keep_cache", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "flush", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "nonseekable", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "flock_release", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "cache_readdir", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "noflush", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "padding", ty = "libc::c_uint", bits = "8..=31")]
    #[bitfield(name = "padding2", ty = "libc::c_uint", bits = "32..=63")]
    pub writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_noflush_padding_padding2: [u8; 8],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
    pub poll_events: uint32_t,
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
        unsafe extern "C" fn(*const libc::c_char, size_t, *mut uint64_t) -> libc::c_int,
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
pub type FILE = _IO_FILE;
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
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache {
    pub on: libc::c_int,
    pub stat_timeout_secs: libc::c_uint,
    pub dir_timeout_secs: libc::c_uint,
    pub link_timeout_secs: libc::c_uint,
    pub max_size: libc::c_uint,
    pub clean_interval_secs: libc::c_uint,
    pub min_clean_interval_secs: libc::c_uint,
    pub next_oper: *mut fuse_operations,
    pub table: *mut GHashTable,
    pub lock: pthread_mutex_t,
    pub last_cleaned: time_t,
    pub write_ctr: uint64_t,
}
pub type gpointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub stat: stat,
    pub stat_valid: time_t,
    pub dir: *mut *mut libc::c_char,
    pub dir_valid: time_t,
    pub link: *mut libc::c_char,
    pub link_valid: time_t,
    pub valid: time_t,
}
pub type gchar = libc::c_char;
pub type gconstpointer = *const libc::c_void;
pub type gboolean = gint;
pub type gint = libc::c_int;
pub type guint = libc::c_uint;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type gsize = libc::c_ulong;
pub type GHRFunc = Option::<
    unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_handle {
    pub is_open: libc::c_int,
    pub fs_fh: libc::c_ulong,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readdir_handle {
    pub path: *const libc::c_char,
    pub buf: *mut libc::c_void,
    pub filler: fuse_fill_dir_t,
    pub dir: *mut GPtrArray,
    pub wrctr: uint64_t,
}
static mut cache: cache = cache {
    on: 0,
    stat_timeout_secs: 0,
    dir_timeout_secs: 0,
    link_timeout_secs: 0,
    max_size: 0,
    clean_interval_secs: 0,
    min_clean_interval_secs: 0,
    next_oper: 0 as *const fuse_operations as *mut fuse_operations,
    table: 0 as *const GHashTable as *mut GHashTable,
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
    last_cleaned: 0,
    write_ctr: 0,
};
unsafe extern "C" fn free_node(mut node_: gpointer) {
    let mut node: *mut node = node_ as *mut node;
    g_strfreev((*node).dir);
    g_free(node as gpointer);
}
unsafe extern "C" fn cache_clean_entry(
    mut key_: *mut libc::c_void,
    mut node: *mut node,
    mut now: *mut time_t,
) -> libc::c_int {
    if *now > (*node).valid {
        return (0 as libc::c_int == 0) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn cache_clean() {
    let mut now: time_t = time(0 as *mut time_t);
    if now > cache.last_cleaned + cache.min_clean_interval_secs as libc::c_long
        && (g_hash_table_size(cache.table) > cache.max_size
            || now > cache.last_cleaned + cache.clean_interval_secs as libc::c_long)
    {
        g_hash_table_foreach_remove(
            cache.table,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut node,
                        *mut time_t,
                    ) -> libc::c_int,
                >,
                GHRFunc,
            >(
                Some(
                    cache_clean_entry
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut node,
                            *mut time_t,
                        ) -> libc::c_int,
                ),
            ),
            &mut now as *mut time_t as gpointer,
        );
        cache.last_cleaned = now;
    }
}
unsafe extern "C" fn cache_lookup(mut path: *const libc::c_char) -> *mut node {
    return g_hash_table_lookup(cache.table, path as gconstpointer) as *mut node;
}
unsafe extern "C" fn cache_purge(mut path: *const libc::c_char) {
    g_hash_table_remove(cache.table, path as gconstpointer);
}
unsafe extern "C" fn cache_purge_parent(mut path: *const libc::c_char) {
    let mut s: *const libc::c_char = strrchr(path, '/' as i32);
    if !s.is_null() {
        if s == path {
            g_hash_table_remove(
                cache.table,
                b"/\0" as *const u8 as *const libc::c_char as gconstpointer,
            );
        } else {
            let mut parent: *mut libc::c_char = g_strndup(
                path,
                s.offset_from(path) as libc::c_long as gsize,
            );
            cache_purge(parent);
            g_free(parent as gpointer);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cache_invalidate(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_invalidate_write(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    cache.write_ctr = (cache.write_ctr).wrapping_add(1);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_invalidate_dir(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    cache_purge_parent(path);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_del_children(
    mut key: *const libc::c_char,
    mut val_: *mut libc::c_void,
    mut path: *const libc::c_char,
) -> libc::c_int {
    if strncmp(key, path, strlen(path)) == 0 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn cache_do_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    pthread_mutex_lock(&mut cache.lock);
    g_hash_table_foreach_remove(
        cache.table,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            GHRFunc,
        >(
            Some(
                cache_del_children
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
        from as *mut libc::c_char as gpointer,
    );
    cache_purge(from);
    cache_purge(to);
    cache_purge_parent(from);
    cache_purge_parent(to);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_get(mut path: *const libc::c_char) -> *mut node {
    let mut node: *mut node = cache_lookup(path);
    if node.is_null() {
        let mut pathcopy: *mut libc::c_char = g_strdup(path);
        node = ({
            let mut __n: gsize = 1 as libc::c_int as gsize;
            let mut __s: gsize = ::std::mem::size_of::<node>() as libc::c_ulong;
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
        }) as *mut node;
        g_hash_table_insert(cache.table, pathcopy as gpointer, node as gpointer);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn cache_add_attr(
    mut path: *const libc::c_char,
    mut stbuf: *const stat,
    mut wrctr: uint64_t,
) {
    let mut node: *mut node = 0 as *mut node;
    pthread_mutex_lock(&mut cache.lock);
    if wrctr == cache.write_ctr {
        node = cache_get(path);
        (*node).stat = *stbuf;
        (*node)
            .stat_valid = time(0 as *mut time_t)
            + cache.stat_timeout_secs as libc::c_long;
        if (*node).stat_valid > (*node).valid {
            (*node).valid = (*node).stat_valid;
        }
        cache_clean();
    }
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_add_dir(
    mut path: *const libc::c_char,
    mut dir: *mut *mut libc::c_char,
) {
    let mut node: *mut node = 0 as *mut node;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_get(path);
    g_strfreev((*node).dir);
    let ref mut fresh0 = (*node).dir;
    *fresh0 = dir;
    (*node).dir_valid = time(0 as *mut time_t) + cache.dir_timeout_secs as libc::c_long;
    if (*node).dir_valid > (*node).valid {
        (*node).valid = (*node).dir_valid;
    }
    cache_clean();
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn my_strnlen(
    mut s: *const libc::c_char,
    mut maxsize: size_t,
) -> size_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s;
    while maxsize != 0 && *p as libc::c_int != 0 {
        maxsize = maxsize.wrapping_sub(1);
        p = p.offset(1);
    }
    return p.offset_from(s) as libc::c_long as size_t;
}
unsafe extern "C" fn cache_add_link(
    mut path: *const libc::c_char,
    mut link: *const libc::c_char,
    mut size: size_t,
) {
    let mut node: *mut node = 0 as *mut node;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_get(path);
    g_free((*node).link as gpointer);
    let ref mut fresh1 = (*node).link;
    *fresh1 = g_strndup(
        link,
        my_strnlen(link, size.wrapping_sub(1 as libc::c_int as libc::c_ulong)),
    );
    (*node)
        .link_valid = time(0 as *mut time_t) + cache.link_timeout_secs as libc::c_long;
    if (*node).link_valid > (*node).valid {
        (*node).valid = (*node).link_valid;
    }
    cache_clean();
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_get_attr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
) -> libc::c_int {
    let mut node: *mut node = 0 as *mut node;
    let mut err: libc::c_int = -(11 as libc::c_int);
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if !node.is_null() {
        let mut now: time_t = time(0 as *mut time_t);
        if (*node).stat_valid - now >= 0 as libc::c_int as libc::c_long {
            *stbuf = (*node).stat;
            err = 0 as libc::c_int;
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn cache_get_write_ctr() -> uint64_t {
    let mut res: uint64_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    res = cache.write_ctr;
    pthread_mutex_unlock(&mut cache.lock);
    return res;
}
unsafe extern "C" fn cache_init(
    mut conn: *mut fuse_conn_info,
    mut cfg: *mut fuse_config,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = 0 as *mut libc::c_void;
    res = ((*cache.next_oper).init).expect("non-null function pointer")(conn, cfg);
    (*cfg).nullpath_ok = 0 as libc::c_int;
    return res;
}
unsafe extern "C" fn cache_getattr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = cache_get_attr(path, stbuf);
    if err != 0 {
        let mut wrctr: uint64_t = cache_get_write_ctr();
        err = ((*cache.next_oper).getattr)
            .expect("non-null function pointer")(path, stbuf, fi);
        if err == 0 {
            cache_add_attr(path, stbuf, wrctr);
        }
    }
    return err;
}
unsafe extern "C" fn cache_readlink(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut node: *mut node = 0 as *mut node;
    let mut err: libc::c_int = 0;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if !node.is_null() {
        let mut now: time_t = time(0 as *mut time_t);
        if (*node).link_valid - now >= 0 as libc::c_int as libc::c_long {
            strncpy(
                buf,
                (*node).link,
                size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            *buf
                .offset(
                    size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            pthread_mutex_unlock(&mut cache.lock);
            return 0 as libc::c_int;
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    err = ((*cache.next_oper).readlink)
        .expect("non-null function pointer")(path, buf, size);
    if err == 0 {
        cache_add_link(path, buf, size);
    }
    return err;
}
unsafe extern "C" fn cache_opendir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    cfi = malloc(::std::mem::size_of::<file_handle>() as libc::c_ulong)
        as *mut file_handle;
    if cfi.is_null() {
        return -(12 as libc::c_int);
    }
    (*cfi).is_open = 0 as libc::c_int;
    (*fi).fh = cfi as libc::c_ulong;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cache_releasedir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    cfi = (*fi).fh as *mut file_handle;
    if (*cfi).is_open != 0 {
        (*fi).fh = (*cfi).fs_fh;
        err = ((*cache.next_oper).releasedir)
            .expect("non-null function pointer")(path, fi);
    } else {
        err = 0 as libc::c_int;
    }
    free(cfi as *mut libc::c_void);
    return err;
}
unsafe extern "C" fn cache_dirfill(
    mut buf: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut stbuf: *const stat,
    mut off: off_t,
    mut flags: fuse_fill_dir_flags,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut ch: *mut readdir_handle = 0 as *mut readdir_handle;
    ch = buf as *mut readdir_handle;
    err = ((*ch).filler)
        .expect("non-null function pointer")((*ch).buf, name, stbuf, off, flags);
    if err == 0 {
        g_ptr_array_add((*ch).dir, g_strdup(name) as gpointer);
        if (*stbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint != 0 {
            let mut fullpath: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut basepath: *const libc::c_char = if *((*ch).path)
                .offset(1 as libc::c_int as isize) == 0
            {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*ch).path
            };
            fullpath = g_strdup_printf(
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                basepath,
                name,
            );
            cache_add_attr(fullpath, stbuf, (*ch).wrctr);
            g_free(fullpath as gpointer);
        }
    }
    return err;
}
unsafe extern "C" fn cache_readdir(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut filler: fuse_fill_dir_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
    mut flags: fuse_readdir_flags,
) -> libc::c_int {
    let mut ch: readdir_handle = readdir_handle {
        path: 0 as *const libc::c_char,
        buf: 0 as *mut libc::c_void,
        filler: None,
        dir: 0 as *mut GPtrArray,
        wrctr: 0,
    };
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    let mut err: libc::c_int = 0;
    let mut dir: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut node: *mut node = 0 as *mut node;
    if offset == 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../cache.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"int cache_readdir(const char *, void *, fuse_fill_dir_t, off_t, struct fuse_file_info *, enum fuse_readdir_flags)\0",
            ))
                .as_ptr(),
        );
    }
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if !node.is_null() && !((*node).dir).is_null() {
        let mut now: time_t = time(0 as *mut time_t);
        if (*node).dir_valid - now >= 0 as libc::c_int as libc::c_long {
            dir = (*node).dir;
            while !(*dir).is_null() {
                filler
                    .expect(
                        "non-null function pointer",
                    )(
                    buf,
                    *dir,
                    0 as *const stat,
                    0 as libc::c_int as off_t,
                    0 as fuse_fill_dir_flags,
                );
                dir = dir.offset(1);
            }
            pthread_mutex_unlock(&mut cache.lock);
            return 0 as libc::c_int;
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    cfi = (*fi).fh as *mut file_handle;
    if (*cfi).is_open != 0 {
        (*fi).fh = (*cfi).fs_fh;
    } else {
        if ((*cache.next_oper).opendir).is_some() {
            err = ((*cache.next_oper).opendir)
                .expect("non-null function pointer")(path, fi);
            if err != 0 {
                return err;
            }
        }
        (*cfi).is_open = 1 as libc::c_int;
        (*cfi).fs_fh = (*fi).fh;
    }
    ch.path = path;
    ch.buf = buf;
    ch.filler = filler;
    ch.dir = g_ptr_array_new();
    ch.wrctr = cache_get_write_ctr();
    err = ((*cache.next_oper).readdir)
        .expect(
            "non-null function pointer",
        )(
        path,
        &mut ch as *mut readdir_handle as *mut libc::c_void,
        Some(
            cache_dirfill
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const stat,
                    off_t,
                    fuse_fill_dir_flags,
                ) -> libc::c_int,
        ),
        offset,
        fi,
        flags,
    );
    g_ptr_array_add(ch.dir, 0 as *mut libc::c_void);
    dir = (*ch.dir).pdata as *mut *mut libc::c_char;
    if err == 0 {
        cache_add_dir(path, dir);
    } else {
        g_strfreev(dir);
    }
    g_ptr_array_free(ch.dir, 0 as libc::c_int);
    return err;
}
unsafe extern "C" fn cache_mknod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut rdev: dev_t,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).mknod)
        .expect("non-null function pointer")(path, mode, rdev);
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_mkdir(
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).mkdir)
        .expect("non-null function pointer")(path, mode);
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_unlink(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).unlink)
        .expect("non-null function pointer")(path);
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_rmdir(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).rmdir)
        .expect("non-null function pointer")(path);
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_symlink(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).symlink)
        .expect("non-null function pointer")(from, to);
    if err == 0 {
        cache_invalidate_dir(to);
    }
    return err;
}
unsafe extern "C" fn cache_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).rename)
        .expect("non-null function pointer")(from, to, flags);
    if err == 0 {
        cache_do_rename(from, to);
    }
    return err;
}
unsafe extern "C" fn cache_link(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).link)
        .expect("non-null function pointer")(from, to);
    if err == 0 {
        cache_invalidate(from);
        cache_invalidate_dir(to);
    }
    return err;
}
unsafe extern "C" fn cache_chmod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).chmod)
        .expect("non-null function pointer")(path, mode, fi);
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_chown(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).chown)
        .expect("non-null function pointer")(path, uid, gid, fi);
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_utimens(
    mut path: *const libc::c_char,
    mut tv: *const timespec,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).utimens)
        .expect("non-null function pointer")(path, tv, fi);
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_write(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut res: libc::c_int = ((*cache.next_oper).write)
        .expect("non-null function pointer")(path, buf, size, offset, fi);
    if res >= 0 as libc::c_int {
        cache_invalidate_write(path);
    }
    return res;
}
unsafe extern "C" fn cache_create(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).create)
        .expect("non-null function pointer")(path, mode, fi);
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_truncate(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = ((*cache.next_oper).truncate)
        .expect("non-null function pointer")(path, size, fi);
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_fill(
    mut oper: *mut fuse_operations,
    mut cache_oper: *mut fuse_operations,
) {
    let ref mut fresh2 = (*cache_oper).access;
    *fresh2 = (*oper).access;
    let ref mut fresh3 = (*cache_oper).chmod;
    *fresh3 = if ((*oper).chmod).is_some() {
        Some(
            cache_chmod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh4 = (*cache_oper).chown;
    *fresh4 = if ((*oper).chown).is_some() {
        Some(
            cache_chown
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    uid_t,
                    gid_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh5 = (*cache_oper).create;
    *fresh5 = if ((*oper).create).is_some() {
        Some(
            cache_create
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh6 = (*cache_oper).flush;
    *fresh6 = (*oper).flush;
    let ref mut fresh7 = (*cache_oper).fsync;
    *fresh7 = (*oper).fsync;
    let ref mut fresh8 = (*cache_oper).getattr;
    *fresh8 = if ((*oper).getattr).is_some() {
        Some(
            cache_getattr
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut stat,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh9 = (*cache_oper).getxattr;
    *fresh9 = (*oper).getxattr;
    let ref mut fresh10 = (*cache_oper).init;
    *fresh10 = Some(
        cache_init
            as unsafe extern "C" fn(
                *mut fuse_conn_info,
                *mut fuse_config,
            ) -> *mut libc::c_void,
    );
    let ref mut fresh11 = (*cache_oper).link;
    *fresh11 = if ((*oper).link).is_some() {
        Some(
            cache_link
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh12 = (*cache_oper).listxattr;
    *fresh12 = (*oper).listxattr;
    let ref mut fresh13 = (*cache_oper).mkdir;
    *fresh13 = if ((*oper).mkdir).is_some() {
        Some(
            cache_mkdir
                as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh14 = (*cache_oper).mknod;
    *fresh14 = if ((*oper).mknod).is_some() {
        Some(
            cache_mknod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    dev_t,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh15 = (*cache_oper).open;
    *fresh15 = (*oper).open;
    let ref mut fresh16 = (*cache_oper).opendir;
    *fresh16 = Some(
        cache_opendir
            as unsafe extern "C" fn(
                *const libc::c_char,
                *mut fuse_file_info,
            ) -> libc::c_int,
    );
    let ref mut fresh17 = (*cache_oper).read;
    *fresh17 = (*oper).read;
    let ref mut fresh18 = (*cache_oper).readdir;
    *fresh18 = if ((*oper).readdir).is_some() {
        Some(
            cache_readdir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    fuse_fill_dir_t,
                    off_t,
                    *mut fuse_file_info,
                    fuse_readdir_flags,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh19 = (*cache_oper).readlink;
    *fresh19 = if ((*oper).readlink).is_some() {
        Some(
            cache_readlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh20 = (*cache_oper).release;
    *fresh20 = (*oper).release;
    let ref mut fresh21 = (*cache_oper).releasedir;
    *fresh21 = Some(
        cache_releasedir
            as unsafe extern "C" fn(
                *const libc::c_char,
                *mut fuse_file_info,
            ) -> libc::c_int,
    );
    let ref mut fresh22 = (*cache_oper).removexattr;
    *fresh22 = (*oper).removexattr;
    let ref mut fresh23 = (*cache_oper).rename;
    *fresh23 = if ((*oper).rename).is_some() {
        Some(
            cache_rename
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_uint,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh24 = (*cache_oper).rmdir;
    *fresh24 = if ((*oper).rmdir).is_some() {
        Some(cache_rmdir as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int)
    } else {
        None
    };
    let ref mut fresh25 = (*cache_oper).setxattr;
    *fresh25 = (*oper).setxattr;
    let ref mut fresh26 = (*cache_oper).statfs;
    *fresh26 = (*oper).statfs;
    let ref mut fresh27 = (*cache_oper).symlink;
    *fresh27 = if ((*oper).symlink).is_some() {
        Some(
            cache_symlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh28 = (*cache_oper).truncate;
    *fresh28 = if ((*oper).truncate).is_some() {
        Some(
            cache_truncate
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh29 = (*cache_oper).unlink;
    *fresh29 = if ((*oper).unlink).is_some() {
        Some(cache_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int)
    } else {
        None
    };
    let ref mut fresh30 = (*cache_oper).utimens;
    *fresh30 = if ((*oper).utimens).is_some() {
        Some(
            cache_utimens
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const timespec,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
    let ref mut fresh31 = (*cache_oper).write;
    *fresh31 = if ((*oper).write).is_some() {
        Some(
            cache_write
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        )
    } else {
        None
    };
}
#[no_mangle]
pub unsafe extern "C" fn cache_wrap(
    mut oper: *mut fuse_operations,
) -> *mut fuse_operations {
    static mut cache_oper: fuse_operations = fuse_operations {
        getattr: None,
        readlink: None,
        mknod: None,
        mkdir: None,
        unlink: None,
        rmdir: None,
        symlink: None,
        rename: None,
        link: None,
        chmod: None,
        chown: None,
        truncate: None,
        open: None,
        read: None,
        write: None,
        statfs: None,
        flush: None,
        release: None,
        fsync: None,
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: None,
        readdir: None,
        releasedir: None,
        fsyncdir: None,
        init: None,
        destroy: None,
        access: None,
        create: None,
        lock: None,
        utimens: None,
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
    cache.next_oper = oper;
    cache_fill(oper, &mut cache_oper);
    pthread_mutex_init(&mut cache.lock, 0 as *const pthread_mutexattr_t);
    cache
        .table = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(free_node as unsafe extern "C" fn(gpointer) -> ()),
    );
    if (cache.table).is_null() {
        fprintf(
            stderr,
            b"failed to create cache\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut fuse_operations;
    }
    return &mut cache_oper;
}
static mut cache_opts: [fuse_opt; 19] = [
    {
        let mut init = fuse_opt {
            templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 4 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 8 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 12 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_stat_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 4 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_dir_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 8 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_link_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 12 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_max_size=%u\0" as *const u8 as *const libc::c_char,
            offset: 16 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_clean_interval=%u\0" as *const u8 as *const libc::c_char,
            offset: 20 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"dcache_min_clean_interval=%u\0" as *const u8 as *const libc::c_char,
            offset: 24 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 4 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 8 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 12 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_stat_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 4 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_dir_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 8 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_link_timeout=%u\0" as *const u8 as *const libc::c_char,
            offset: 12 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_max_size=%u\0" as *const u8 as *const libc::c_char,
            offset: 16 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_clean_interval=%u\0" as *const u8 as *const libc::c_char,
            offset: 20 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"cache_min_clean_interval=%u\0" as *const u8 as *const libc::c_char,
            offset: 24 as libc::c_ulong,
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
#[no_mangle]
pub unsafe extern "C" fn cache_parse_options(mut args: *mut fuse_args) -> libc::c_int {
    cache.stat_timeout_secs = 20 as libc::c_int as libc::c_uint;
    cache.dir_timeout_secs = 20 as libc::c_int as libc::c_uint;
    cache.link_timeout_secs = 20 as libc::c_int as libc::c_uint;
    cache.max_size = 10000 as libc::c_int as libc::c_uint;
    cache.clean_interval_secs = 60 as libc::c_int as libc::c_uint;
    cache.min_clean_interval_secs = 5 as libc::c_int as libc::c_uint;
    return fuse_opt_parse(
        args,
        &mut cache as *mut cache as *mut libc::c_void,
        cache_opts.as_ptr(),
        None,
    );
}
