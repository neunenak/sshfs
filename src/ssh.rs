use crate::global_settings;
use crate::sshfs;
use crate::{connect_remote, sftp_check_root};
use libc::{signal, SIGPIPE, SIG_IGN};
use std::ffi::CString;

pub unsafe fn ssh_connect(max_conns: u32, no_check_root: bool, delay_connect: bool) -> Result<(), ()> {
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
