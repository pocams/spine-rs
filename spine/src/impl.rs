use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{BufReader, Read},
    os::raw::{c_char, c_int, c_void},
    mem::forget
};

use spine_sys::spAtlasPage;

use super::{error::Error, result::Result};

#[no_mangle]
pub extern "C" fn _spUtil_readFile(path: *const c_char, length: *mut c_int) -> *const c_char {
    #[inline]
    fn read_file(path: *const c_char) -> Result<Vec<u8>> {
        let path = to_str(path)?;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }

    let data = match read_file(path) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("_spUtil_readFile: {}", error);
            return std::ptr::null();
        }
    };

    let data_ptr = data.as_ptr() as *const c_char;
    let data_length = data.len() as c_int;
    forget(data);

    unsafe {
        *length = data_length;
        data_ptr
    }
}

#[inline]
fn to_str<'a>(s: *const c_char) -> Result<&'a str> {
    let s = unsafe { CStr::from_ptr(s) }
        .to_str()
        .map_err(Error::invalid_input)?;

    Ok(s)
}
