use crate::sshfs;
use crate::{
    connect_remote, pthread_cond_init, pthread_condattr_t, pthread_mutex_init, pthread_mutexattr_t,
    sftp_check_root,
};
use libc::{signal, SIGPIPE, SIG_IGN};

pub unsafe fn ssh_connect(max_conns: u32, no_check_root: bool, delay_connect: bool) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = processing_init(max_conns);
    if res == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !delay_connect {
        if connect_remote(&mut *(sshfs.conns).offset(0 as libc::c_int as isize))
            == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if !no_check_root
            && sftp_check_root(
                &mut *(sshfs.conns).offset(0 as libc::c_int as isize),
                sshfs.base_path,
            ) != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}

unsafe fn processing_init(max_conns: u32) -> libc::c_int {
    let mut i: libc::c_int = 0;
    signal(SIGPIPE, SIG_IGN);

    pthread_mutex_init(&mut sshfs.lock, 0 as *const pthread_mutexattr_t);
    i = 0 as libc::c_int;
    while i < max_conns.try_into().unwrap() {
        pthread_mutex_init(
            &mut (*(sshfs.conns).offset(i as isize)).lock_write,
            0 as *const pthread_mutexattr_t,
        );
        i += 1;
    }
    pthread_cond_init(&mut sshfs.outstanding_cond, 0 as *const pthread_condattr_t);
    sshfs.reqtab = crate::g_hash_table_new(None, None);
    if (sshfs.reqtab).is_null() {
        eprintln!("failed to create hash table");

        return -1;
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
            return -1;
        }
    }
    return 0 as libc::c_int;
}
