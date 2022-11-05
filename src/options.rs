use crate::{IDMAP_DEFAULT, SSHFS_VERSION};
use libfuse_sys::fuse::{fuse_args, fuse_lib_help};
use std::ffi::CStr;

use clap::{Arg, ArgAction, ArgMatches, Command};

fn version_string() -> String {
    use std::fmt::Write;

    let mut buf = String::new();
    writeln!(buf, "SSHFS version {}", SSHFS_VERSION).unwrap();

    let fuse_package_version = unsafe { CStr::from_ptr(crate::fuse_pkgversion()) };
    writeln!(
        buf,
        "FUSE library version {}",
        fuse_package_version.to_string_lossy()
    )
    .unwrap();
    //TODO can this print fuse_lowlevel_version() and fusermount information?
    buf
}

pub fn sshfs_options() -> Command {
    Command::new("sshfs")
        .version(version_string())
        .arg(
            Arg::new("ssh_protocol_1")
                .short('1')
                .help("equivalent to '-o ssh_protocol=1'"),
        )
        .arg(
            Arg::new("compression")
                .short('C')
                .action(ArgAction::SetTrue)
                .help("equivlaent to '-o compression=yes'"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("print some debugging information (implies -f)"),
        )
        .arg(
            Arg::new("foreground")
                .short('f')
                .action(ArgAction::SetTrue)
                .help("foreground operation"),
        )
        .arg(
            Arg::new("ssh_configfile")
                .short('F')
                .help("specifies alternative ssh configuration file"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .action(ArgAction::Append)
                .help("mount options"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("equivlaent to '-o port=PORT'"),
        )
        .arg(
            Arg::new("multithreaded")
                .short('s')
                .help("disable multi-threaded operation")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("print ssh replies and messages"),
        )
        .arg(
            Arg::new("connect_string")
                .value_names(["[user@]host:[dir]"])
                .required(true),
        )
        .arg(Arg::new("mountpoint").required(true))
}

pub fn show_help(fuse_args: *mut fuse_args) {
    let e = std::borrow::Cow::from("");
    let first_arg = std::env::args_os().next();
    let program_name = match first_arg {
        Some(ref os_str) => os_str.to_string_lossy(),
        None => e,
    };

    print_usage(&program_name);

    println!("FUSE Options:");
    unsafe {
        fuse_lib_help(fuse_args);
    }
    std::process::exit(0);
}

fn print_usage(progname: &str) {
    println!(
        r#"
usage: {progname} [user@]host:[dir] mountpoint [options]

    -h   --help            print help
    -V   --version         print version
    -f                     foreground operation
    -s                     disable multi-threaded operation
    -p PORT                equivalent to '-o port=PORT'
    -C                     equivalent to '-o compression=yes'
    -F ssh_configfile      specifies alternative ssh configuration file
    -1                     equivalent to '-o ssh_protocol=1'
    -o opt,[opt...]        mount options
    -o reconnect           reconnect to server
    -o delay_connect       delay connection to server
    -o sshfs_sync          synchronous writes
    -o no_readahead        synchronous reads (no speculative readahead)
    -o sync_readdir        synchronous readdir
    -d, --debug            print some debugging information (implies -f)
    -v, --verbose          print ssh replies and messages
    -o dir_cache=BOOL      enable caching of directory contents (names,
                           attributes, symlink targets) {{yes,no}} (default: yes)
    -o dcache_max_size=N   sets the maximum size of the directory cache (default: 10000)
    -o dcache_timeout=N    sets timeout for directory cache in seconds (default: 20)
    -o dcache_{{stat,link,dir}}_timeout=N
                           sets separate timeout for {{attributes, symlinks, names}}
    -o dcache_clean_interval=N
                           sets the interval for automatic cleaning of the
                           cache (default: 60)
    -o dcache_min_clean_interval=N
                           sets the interval for forced cleaning of the
                           cache if full (default: 5)
    -o direct_io           enable direct i/o
    -o workaround=LIST     colon separated list of workarounds
             none             no workarounds enabled
             [no]rename       fix renaming to existing file (default: off)
             [no]renamexdev   fix moving across filesystems (default: off)
             [no]truncate     fix truncate for old servers (default: off)
             [no]buflimit     fix buffer fillup bug in server (default: off)
             [no]fstat        always use stat() instead of fstat() (default: off)
             [no]createmode   always pass mode 0 to create (default: off)
    -o idmap=TYPE          user/group ID mapping (default: {IDMAP_DEFAULT})
             none             no translation of the ID space
             user             only translate UID/GID of connecting user
             file             translate UIDs/GIDs contained in uidfile/gidfile
    -o uidfile=FILE        file containing username:remote_uid mappings
    -o gidfile=FILE        file containing groupname:remote_gid mappings
    -o nomap=TYPE          with idmap=file, how to handle missing mappings
             ignore           don't do any re-mapping
             error            return an error (default)
    -o ssh_command=CMD     execute CMD instead of 'ssh'
    -o ssh_protocol=N      ssh protocol to use (default: 2)
    -o sftp_server=SERV    path to sftp server or subsystem (default: sftp)
    -o directport=PORT     directly connect to PORT bypassing ssh
    -o passive             communicate over stdin and stdout bypassing network
    -o disable_hardlink    link(2) will return with errno set to ENOSYS
    -o transform_symlinks  transform absolute symlinks to relative
    -o follow_symlinks     follow symlinks on the server
    -o no_check_root       don't check for existence of 'dir' on server
    -o password_stdin      read password from stdin (only for pam_mount!)
    -o max_conns=N         open parallel SSH connections
    -o SSHOPT=VAL          ssh options (see man ssh_config)
"#
    );
}

pub fn set_sshfs_options_from_matches(matches: ArgMatches) {}
