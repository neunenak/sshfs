use crate::{IDMAP_DEFAULT, SSHFS_VERSION};
use libfuse_sys::fuse::{fuse_args, fuse_lib_help};
use std::ffi::CStr;

use clap::builder::TypedValueParser;
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

#[derive(Debug, Clone)]
struct SshFSOptionValueParser;

#[derive(Clone, Debug, PartialEq)]
pub enum IdMap {
    None,
    User,
    File,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NoMap {
    Ignore,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Workaround {
    None,
    Rename(bool),
    RenameXDev(bool),
    Truncate(bool),
    Buflimit(bool),
    Fstat(bool),
    Createmode(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SshFSOption {
    DirectPort(String), // directport=PORT
    SshCommand(String),
    SftpServer(String), //sftp_server=SERV
    MaxRead(u32),
    MaxWrite(u32),
    SshProtocol(u32), // ssh_protocol
    Workaround(Workaround),
    IdMap(IdMap),
    UidFile(String),
    GidFile(String),
    NoMap(NoMap),
    SshfsSync,
    NoReadahead,
    SyncReaddir,
    SyncReadahead,
    Debug, // -o sshfs_debug as opposed to -d or --debug
    Verbose,
    Reconnect,
    TransformSymlinks,
    FollowSymlinks,
    NoCheckRoot,
    PasswordStdin,
    DelayConnect,
    Slave, // passive or slave
    DisableHardlink,
    DirCache(bool),
    DirectIO,
    MaxConns(u32),
    Discarded,
    SSHOption(String),
    OtherOption(String),
}

impl TypedValueParser for SshFSOptionValueParser {
    type Value = SshFSOption;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        use clap::error::{Error, ErrorKind};
        use std::str::FromStr;

        let value = value.to_string_lossy().to_string();
        println!("Value: {}", value);

        if let Some(rest) = value.strip_prefix("directport=") {
            return Ok(SshFSOption::DirectPort(rest.to_string()));
        }
        if let Some(rest) = value.strip_prefix("ssh_command=") {
            return Ok(SshFSOption::SshCommand(rest.to_string()));
        }
        if let Some(rest) = value.strip_prefix("sftp_server=") {
            return Ok(SshFSOption::SftpServer(rest.to_string()));
        }
        if let Some(rest) = value.strip_prefix("max_read=") {
            let n = u32::from_str(rest)
                .map_err(|_| Error::new(ErrorKind::InvalidValue).with_cmd(cmd))?;
            return Ok(SshFSOption::MaxRead(n));
        }
        if let Some(rest) = value.strip_prefix("max_write=") {
            let n = u32::from_str(rest)
                .map_err(|_| Error::new(ErrorKind::InvalidValue).with_cmd(cmd))?;
            return Ok(SshFSOption::MaxWrite(n));
        }
        if let Some(rest) = value.strip_prefix("ssh_protocol=") {
            let n = u32::from_str(rest)
                .map_err(|_| Error::new(ErrorKind::InvalidValue).with_cmd(cmd))?;
            return Ok(SshFSOption::SshProtocol(n));
        }
        //TODO parse workarounds
        if let Some(rest) = value.strip_prefix("idmap=") {
            let idmap = match rest {
                "none" => IdMap::None,
                "user" => IdMap::User,
                "file" => IdMap::File,
                _ => return Err(Error::new(ErrorKind::InvalidValue).with_cmd(cmd)),
            };
            return Ok(SshFSOption::IdMap(idmap));
        }

        if let Some(rest) = value.strip_prefix("uidfile=") {
            return Ok(SshFSOption::UidFile(rest.to_string()));
        }
        if let Some(rest) = value.strip_prefix("gidfile=") {
            return Ok(SshFSOption::GidFile(rest.to_string()));
        }
        if let Some(rest) = value.strip_prefix("nomap=") {
            let nomap = match rest {
                "ignore" => NoMap::Ignore,
                "error" => NoMap::Error,
                _ => return Err(Error::new(ErrorKind::InvalidValue).with_cmd(cmd)),
            };
            return Ok(SshFSOption::NoMap(nomap));
        }

        if let Some(rest) = value.strip_prefix("dir_cache=") {
            let b = match rest {
                "yes" => true,
                "no" => false,
                _ => return Err(Error::new(ErrorKind::InvalidValue).with_cmd(cmd)),
            };
            return Ok(SshFSOption::DirCache(b));
        }
        /* for backwards compatibility */
        if let Some(rest) = value.strip_prefix("cache=") {
            let b = match rest {
                "yes" => true,
                "no" => false,
                _ => return Err(Error::new(ErrorKind::InvalidValue).with_cmd(cmd)),
            };
            return Ok(SshFSOption::DirCache(b));
        }
        if let Some(rest) = value.strip_prefix("max_conns=") {
            let n = u32::from_str(rest)
                .map_err(|_| Error::new(ErrorKind::InvalidValue).with_cmd(cmd))?;
            return Ok(SshFSOption::MaxConns(n));
        }

        let output = match value.as_ref() {
            "sshfs_sync" => SshFSOption::SshfsSync,
            "no_readahead" => SshFSOption::NoReadahead,
            "sync_readdir" => SshFSOption::SyncReaddir,
            "sshfs_debug" => SshFSOption::Debug,
            "sshfs_verbose" => SshFSOption::Verbose,
            "reconnect" => SshFSOption::Reconnect,
            "transform_symlinks" => SshFSOption::TransformSymlinks,
            "follow_symlinks" => SshFSOption::FollowSymlinks,
            "no_check_root" => SshFSOption::NoCheckRoot,
            "password_stdin" => SshFSOption::PasswordStdin,
            "delay_connect" => SshFSOption::DelayConnect,
            "slave" | "passive" => SshFSOption::Slave,
            "disable_hardlink" => SshFSOption::DisableHardlink,
            "direct_io" => SshFSOption::DirectIO,

            "writeback_cache=no" => SshFSOption::Discarded,
            "unreliable_append" => SshFSOption::Discarded,

            /* These may come in from /etc/fstab - we just ignore them */
            "auto" | "noauto" | "user" | "nouser" | "users" | "_netdev" => SshFSOption::Discarded,
            other if crate::ssh_opt::is_ssh_opt_str(other) => {
                SshFSOption::SSHOption(other.to_string())
            }
            other => SshFSOption::OtherOption(other.into()),
        };

        Ok(output)
    }
}

pub fn sshfs_options() -> Command {
    Command::new("sshfs")
        .version(version_string())
        .after_help(print_usage())
        .arg(
            Arg::new("ssh_protocol_1")
                .short('1')
                .action(ArgAction::SetTrue)
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
                .action(ArgAction::SetTrue)
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
                .value_parser(SshFSOptionValueParser)
                .help("mount options (see below)"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("equivlaent to '-o port=PORT'"),
        )
        .arg(
            Arg::new("singlethreaded")
                .short('s')
                .help("disable multi-threaded operation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("print ssh replies and messages"),
        )
        .arg(
            Arg::new("host")
                .value_names(["[user@]host:[dir]"])
                .required(true),
        )
        .arg(Arg::new("mountpoint").required(true))
}

/*
fn show_help(fuse_args: *mut fuse_args) {
    print_usage();

    println!("FUSE Options:");
    unsafe {
        fuse_lib_help(fuse_args);
    }
    std::process::exit(0);
}
*/

fn print_usage() -> String {
    let first_arg = std::env::args_os().next();
    let progname = match first_arg {
        Some(ref os_str) => os_str.to_string_lossy(),
        None => "".into(),
    };

    format!(
        r#"
-o Mount Options:

    -o reconnect           reconnect to server
    -o delay_connect       delay connection to server
    -o sshfs_sync          synchronous writes
    -o no_readahead        synchronous reads (no speculative readahead)
    -o sync_readdir        synchronous readdir
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
    )
}
