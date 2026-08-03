//! Open Kernel environment storage.

pub use super::common::Env;

use crate::collections::HashMap;
use crate::ffi::{CStr, OsStr, OsString};
use crate::io;
use crate::sync::{Mutex, OnceLock};

type Environment = Mutex<HashMap<OsString, OsString>>;
static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

/// Copies `KEY=VALUE` entries from the process startup block.
///
/// The environment vector directly follows the null-terminated argument
/// vector. Values are copied into owned `OsString`s, preserving encoded bytes.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    let environment = ENVIRONMENT.get_or_init(|| Mutex::new(HashMap::new()));
    let mut values = environment.lock().unwrap();
    values.clear();
    if argv.is_null() {
        return;
    }

    let mut entry = unsafe { argv.add(argc.max(0) as usize + 1) };
    loop {
        let pointer = unsafe { entry.read() };
        if pointer.is_null() {
            break;
        }
        let bytes = unsafe { CStr::from_ptr(pointer.cast()).to_bytes() };
        if let Some(separator) = bytes.iter().position(|byte| *byte == b'=') {
            let key = unsafe { OsString::from_encoded_bytes_unchecked(bytes[..separator].to_vec()) };
            let value =
                unsafe { OsString::from_encoded_bytes_unchecked(bytes[separator + 1..].to_vec()) };
            values.insert(key, value);
        }
        entry = unsafe { entry.add(1) };
    }
}

fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn env() -> Env {
    let values = environment().lock().unwrap().iter().map(|(key, value)| {
        (key.clone(), value.clone())
    }).collect();
    Env::new(values)
}

pub fn getenv(key: &OsStr) -> Option<OsString> {
    environment().lock().unwrap().get(key).cloned()
}

pub unsafe fn setenv(key: &OsStr, value: &OsStr) -> io::Result<()> {
    environment().lock().unwrap().insert(key.to_owned(), value.to_owned());
    Ok(())
}

pub unsafe fn unsetenv(key: &OsStr) -> io::Result<()> {
    environment().lock().unwrap().remove(key);
    Ok(())
}
