use crate::options::{IdMap, NoMap};
use crate::Request;
use crate::IDMAP_DEFAULT;
use crate::{fuse_operations, Connection};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Condvar, Mutex};

/*
lazy_static::lazy_static! {
    static ref request_table: Mutex<HashMap<u32, Request>> = Mutex::new(HashMap::new());
}
*/

pub static mut global_connections: Vec<Connection> = Vec::new();

pub static global_lock: Mutex<()> = Mutex::new(());

pub static global_cond: Condvar = Condvar::new();

pub static mut password_ptr: *mut libc::c_char = std::ptr::null_mut();

pub static mut ptyfd: libc::c_int = 0;

#[derive(Debug, Clone)]
pub struct NewSettings {
    pub mountpoint: Option<PathBuf>,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub ssh_args: Vec<String>,
    pub debug: bool,
    pub verbose: bool,
    pub foreground: bool,
    pub passive: bool,
    pub ssh_ver: u8,
    pub directport: Option<String>,
    pub ssh_command: Option<String>,
    pub max_read: u32,
    pub max_write: u32,
    pub dir_cache: bool,
    pub direct_io: bool,
    pub password_stdin: bool,
    pub no_check_root: bool,
    pub delay_connect: bool,
    pub reconnect: bool,
    pub transform_symlinks: bool,
    pub follow_symlinks: bool,
    pub disable_hardlink: bool,
    pub idmap: IdMap,
    pub nomap: NoMap,
    pub uidfile: Option<String>,
    pub gidfile: Option<String>,
    pub sync_write: bool,
    pub sync_read: bool,
    pub sync_readdir: bool,
    pub max_conns: u32,
    pub blksize: u32,
    pub detect_uid: bool,
    pub rename_workaround: bool,
    pub renamexdev_workaround: bool,
    pub truncate_workaround: bool,
    pub buflimit_workaround: bool,
    pub fstat_workaround: bool,
    pub createmode_workaround: bool,
}

pub static mut global_settings: NewSettings = NewSettings {
    mountpoint: None,
    host: None,
    base_path: None,
    ssh_args: vec![],
    debug: false,
    verbose: false,
    foreground: false,
    passive: false,
    ssh_ver: 2,
    directport: None,
    ssh_command: None,
    max_read: 0,
    max_write: 0,
    dir_cache: true,
    direct_io: false,
    password_stdin: false,
    no_check_root: false,
    delay_connect: false,
    reconnect: false,
    transform_symlinks: false,
    follow_symlinks: false,
    disable_hardlink: false,
    idmap: IDMAP_DEFAULT,
    nomap: NoMap::Error,
    uidfile: None,
    gidfile: None,
    sync_write: false,
    sync_read: false,
    sync_readdir: false,
    max_conns: 1,
    blksize: 0,
    detect_uid: false,
    rename_workaround: false,
    renamexdev_workaround: false,
    truncate_workaround: false,
    buflimit_workaround: false,
    createmode_workaround: false,
    fstat_workaround: false,
};

#[derive(Default, Clone)]
pub struct Counters {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub num_sent: u64,
    pub num_received: u64,
    pub min_rtt: u64,
    pub max_rtt: u64,
    pub total_rtt: u64,
    pub num_connect: u64,
}

pub static mut counters: Counters = Counters {
    bytes_sent: 0,
    bytes_received: 0,
    num_sent: 0,
    num_received: 0,
    min_rtt: 0,
    max_rtt: 0,
    total_rtt: 0,
    num_connect: 0,
};

pub static mut sshfs_operations: *mut fuse_operations = 0 as *mut fuse_operations;
