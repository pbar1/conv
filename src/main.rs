use std::io::{self, Read};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;


extern "C" {
    fn HCLtoJSON(inHCL: GoString) -> *const c_char;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

pub fn hcl_to_json(in_hcl: &str) -> Result<&str, &str> {
    let c_in_hcl = CString::new(in_hcl).expect("CString::new failed");
    let ptr = c_in_hcl.as_ptr();
    let go_string = GoString {
        a: ptr,
        b: c_in_hcl.as_bytes().len() as i64,
    };
    let result = unsafe { HCLtoJSON(go_string) };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("Error translating from HCL library");
    match string.is_empty() || string.starts_with("error: ") {
        true => Err("foo"),
        false => Ok(string),
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    match hcl_to_json(buffer.as_str()) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}
