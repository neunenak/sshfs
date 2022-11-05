use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref UID_MAP: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);
    static ref REVERSE_UID_MAP: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);

    static ref GID_MAP: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);
    static ref REVERSE_GID_MAP: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);
}

enum IdType {
    Uid,
    Gid,
}

pub enum MapBehavior {
    Ignore,
    Error,
}

/* given a pointer to the uid/gid, and the mapping table, remap the
 * uid/gid, if necessary */
#[inline]
pub fn translate_id(id_ptr: *mut u32, operation: &str, nomap: c_int) -> c_int {

    let table = match operation {
        "uid" => UID_MAP.lock().unwrap(),
        "ruid" => REVERSE_UID_MAP.lock().unwrap(),
        "gid" => GID_MAP.lock().unwrap(),
        "rgid" => REVERSE_UID_MAP.lock().unwrap(),
        _ => {
            eprintln!("internal error");
            std::process::exit(1);
        }
    };

    let table = match table.as_ref() {
        Some(table) => table,
        None => return 0,
    };

    let map_behavior = if nomap == 0 {
        // NOMAP_IGNORE
        MapBehavior::Ignore
    } else if nomap == 1 {
        //NOMAP_ERROR
        MapBehavior::Error
    } else {
        eprintln!("internal error");
        std::process::exit(1);
    };

    let id = unsafe { *id_ptr };

    let mapped_id = match table.get(&id) {
        Some(id) => id.clone(),
        None => match map_behavior {
            MapBehavior::Error => return -1,
            MapBehavior::Ignore => return 0,
        },
    };
    unsafe {
        *id_ptr = mapped_id;
    }
    0
}

#[no_mangle]
pub extern "C" fn handle_id_maps(uid_file_path: *mut c_char, gid_file_path: *mut c_char) {
    if uid_file_path.is_null() && gid_file_path.is_null() {
        eprintln!("Need a uidfile or gidfile with idmap=file");
        std::process::exit(1);
    }

    if !uid_file_path.is_null() {
        let uid_path_cstr = unsafe { CStr::from_ptr(uid_file_path) };
        let (um, rum) = read_id_map(&uid_path_cstr.to_string_lossy(), IdType::Uid);
        let mut mtx = UID_MAP.lock().unwrap();
        *mtx = Some(um);

        let mut mtx = REVERSE_UID_MAP.lock().unwrap();
        *mtx = Some(rum);
    }

    if !gid_file_path.is_null() {
        let gid_path_cstr = unsafe { CStr::from_ptr(gid_file_path) };
        let (gm, rgm) = read_id_map(&gid_path_cstr.to_string_lossy(), IdType::Gid);

        let mut mtx = GID_MAP.lock().unwrap();
        *mtx = Some(gm);

        let mut mtx = REVERSE_GID_MAP.lock().unwrap();
        *mtx = Some(rgm);
    }
}

fn read_id_map(file_name: &str, id_type: IdType) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
    use std::os::unix::fs::MetadataExt;

    let current_uid = users::get_current_uid();
    let mut map = HashMap::new();
    let mut reverse_map = HashMap::new();

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("failed to open '{}': {}", file_name, e);
            std::process::exit(1);
        }
    };

    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("failed to stat '{}': {}", file_name, e);
            std::process::exit(1);
        }
    };

    let file_uid = metadata.uid();
    if file_uid != current_uid {
        eprintln!("'{}' is not owned by uid {}", file_name, current_uid);
    }

    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let (remote_id, name) = parse_idmap_line(&line, file_name, line_number);
        let maybe_local_id = match id_type {
            IdType::Uid => username_to_uid(name),
            IdType::Gid => groupname_to_gid(name),
        };
        let local_id = match maybe_local_id {
            Some(id) => id,
            None => {
                unsafe {
                    if crate::sshfs.debug != 0 {
                        let name_id = match id_type {
                            IdType::Uid => "uid",
                            IdType::Gid => "gid",
                        };
                        eprintln!("{}({}): no local {}", name, remote_id, name_id);
                    }
                }
                continue;
            }
        };

        map.insert(local_id, remote_id);
        reverse_map.insert(remote_id, local_id);
    }

    (map, reverse_map)
}

// Expected format (I think) is colon-separated, either `name:id` or `name:???:id`
fn parse_idmap_line<'a>(line: &'a str, filename: &'a str, line_num: usize) -> (u32, &'a str) {
    let mut iter = line.split(":");

    let name = match iter.next() {
        Some(s) => s,
        None => {
            eprintln!("{}:{} unknown format", filename, line_num);
            std::process::exit(1);
        }
    };

    let second_token = iter.next();
    let third_token = iter.next();
    let id_tok = match (second_token, third_token) {
        (_, Some(s)) => s,
        (Some(s), None) => s,
        (None, None) => {
            eprintln!("{}:{} unknown format", filename, line_num);
            std::process::exit(1);
        }
    };

    let id: u32 = match u32::from_str(id_tok) {
        Ok(n) => n,
        Err(e) => {
            eprintln!(
                "Invalid id number on line {} of '{}': {}",
                line_num, filename, e
            );
            std::process::exit(1);
        }
    };

    (id, name)
}

fn username_to_uid(username: &str) -> Option<u32> {
    users::get_user_by_name(username).map(|u| u.uid())
}

fn groupname_to_gid(group_name: &str) -> Option<u32> {
    users::get_group_by_name(group_name).map(|g| g.gid())
}

