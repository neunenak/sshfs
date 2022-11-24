use libfuse_sys::fuse::fuse_opt;

// This is the `show_version` offset
const DISCARD: libc::c_int = -4;
const KEY_CONFIGFILE: libc::c_int = 2;
const KEY_PORT: libc::c_int = 0;
const KEY_COMPRESS: libc::c_int = 1;

pub fn set_fuse_opts(sshfs_opts_handle: &mut [fuse_opt; 55]) {
    *sshfs_opts_handle = [
        {
            fuse_opt {
                templ: b"directport=%s\0" as *const u8 as *const libc::c_char,
                offset: 0 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"ssh_command=%s\0" as *const u8 as *const libc::c_char,
                offset: 8 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"sftp_server=%s\0" as *const u8 as *const libc::c_char,
                offset: 16 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"max_read=%u\0" as *const u8 as *const libc::c_char,
                offset: 184 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"max_write=%u\0" as *const u8 as *const libc::c_char,
                offset: 188 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"ssh_protocol=%u\0" as *const u8 as *const libc::c_char,
                offset: 192 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-1\0" as *const u8 as *const libc::c_char,
                offset: 192 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"workaround=%s\0" as *const u8 as *const libc::c_char,
                offset: 48 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"idmap=none\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"idmap=user\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"idmap=file\0" as *const u8 as *const libc::c_char,
                offset: 100 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"uidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: 136 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"gidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: 144 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"nomap=ignore\0" as *const u8 as *const libc::c_char,
                offset: 104 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"nomap=error\0" as *const u8 as *const libc::c_char,
                offset: 104 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"sshfs_sync\0" as *const u8 as *const libc::c_char,
                offset: 196 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"no_readahead\0" as *const u8 as *const libc::c_char,
                offset: 200 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"sync_readdir\0" as *const u8 as *const libc::c_char,
                offset: 204 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"sshfs_debug\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"sshfs_verbose\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"reconnect\0" as *const u8 as *const libc::c_char,
                offset: 224 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"transform_symlinks\0" as *const u8 as *const libc::c_char,
                offset: 84 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"follow_symlinks\0" as *const u8 as *const libc::c_char,
                offset: 88 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"no_check_root\0" as *const u8 as *const libc::c_char,
                offset: 92 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"password_stdin\0" as *const u8 as *const libc::c_char,
                offset: 440 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"delay_connect\0" as *const u8 as *const libc::c_char,
                offset: 228 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"slave\0" as *const u8 as *const libc::c_char,
                offset: 232 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"passive\0" as *const u8 as *const libc::c_char,
                offset: 232 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"disable_hardlink\0" as *const u8 as *const libc::c_char,
                offset: 108 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"dir_cache=yes\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"dir_cache=no\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"direct_io\0" as *const u8 as *const libc::c_char,
                offset: 208 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"max_conns=%u\0" as *const u8 as *const libc::c_char,
                offset: 316 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-h\0" as *const u8 as *const libc::c_char,
                offset: 120 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"--help\0" as *const u8 as *const libc::c_char,
                offset: 120 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-d\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"debug\0" as *const u8 as *const libc::c_char,
                offset: 212 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-v\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"verbose\0" as *const u8 as *const libc::c_char,
                offset: 216 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-f\0" as *const u8 as *const libc::c_char,
                offset: 116 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-s\0" as *const u8 as *const libc::c_char,
                offset: 124 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-p \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-C\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"-F \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"cache=yes\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"cache=no\0" as *const u8 as *const libc::c_char,
                offset: 112 as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"writeback_cache=no\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"unreliable_append\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"auto\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"noauto\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"user\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"nouser\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"users\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: b"_netdev\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: DISCARD,
            }
        },
        {
            fuse_opt {
                templ: 0 as *const libc::c_char,
                offset: 0 as libc::c_int as libc::c_ulong,
                value: DISCARD,
            }
        },
    ];
}
