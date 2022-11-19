use crate::global_settings;
use crate::sshfs;
use crate::statics::counters;
use crate::{close_conn, connect_to, sftp_check_root, sftp_init, start_ssh, Connection};
use libc::{signal, SIGPIPE, SIG_IGN};
use std::ffi::CString;

pub unsafe fn ssh_connect(
    max_conns: u32,
    no_check_root: bool,
    delay_connect: bool,
) -> Result<(), ()> {
    processing_init(max_conns)?;

    if !delay_connect {
        let connection = &mut crate::statics::global_connections[0];
        connect_remote(connection)?;

        let base_path_cstring = CString::new(
            global_settings
                .base_path
                .as_ref()
                .unwrap()
                .to_string()
                .into_bytes(),
        )
        .unwrap();
        let ptr = base_path_cstring.as_ptr();

        if !no_check_root && sftp_check_root(connection, ptr) != 0 as libc::c_int {
            return Err(());
        }
    }
    Ok(())
}

unsafe fn processing_init(max_conns: u32) -> Result<(), ()> {
    signal(SIGPIPE, SIG_IGN);

    sshfs.reqtab = crate::g_hash_table_new(None, None);
    if (sshfs.reqtab).is_null() {
        eprintln!("failed to create hash table");
        return Err(());
    }

    if max_conns > 1 {
        sshfs.conntab = crate::g_hash_table_new_full(
            Some(crate::g_str_hash as unsafe extern "C" fn(crate::gconstpointer) -> crate::guint),
            Some(
                crate::g_str_equal
                    as unsafe extern "C" fn(
                        crate::gconstpointer,
                        crate::gconstpointer,
                    ) -> crate::gboolean,
            ),
            Some(crate::g_free as unsafe extern "C" fn(crate::gpointer) -> ()),
            None,
        );
        if (sshfs.conntab).is_null() {
            eprintln!("failed to create hash table");
            return Err(());
        }
    }
    Ok(())
}

pub unsafe fn connect_remote(mut conn: *mut Connection) -> Result<(), ()> {
    let mut err = match (global_settings.passive, global_settings.directport.as_ref()) {
        (true, _) => connect_passive(conn),
        (false, Some(ref port)) => {
            let port_cstring = CString::new(port.to_string().into_bytes()).unwrap();
            connect_to(
                conn,
                global_settings.host.as_ref().unwrap(),
                port_cstring.as_ptr(),
            )
        }
        _ => start_ssh(conn),
    };

    if err == 0 {
        err = sftp_init(conn);
    };

    if err == 0 {
        counters.num_connect = (counters.num_connect).wrapping_add(1);
        Ok(())
    } else {
        close_conn(conn);
        Err(())
    }
}

unsafe fn connect_passive(mut conn: *mut Connection) -> libc::c_int {
    (*conn).rfd = 0 as libc::c_int;
    (*conn).wfd = 1 as libc::c_int;
    return 0 as libc::c_int;
}
