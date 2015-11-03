extern crate rustc_serialize;

#[cfg(feature = "embedded")]
extern crate libc;

mod actiondispatch;
pub mod xfstruct;
pub mod validation;
pub mod errors;

#[cfg(not(feature = "embedded"))]
pub use self::xfstruct::*;

#[cfg(not(feature = "embedded"))]
pub use self::validation::*;


//
// Embedded config
//

#[cfg(feature = "embedded")]
mod embed {

    use libc::{c_char, uint32_t};
    use std::ffi::CStr;
    use std::str;

#[no_mangle]
    pub extern fn char_count(s: *const c_char) -> uint32_t {
        let c_str = unsafe {
            assert!(!s.is_null());

            CStr::from_ptr(s)
        };

        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
        r_str.chars().count() as uint32_t
    }
}

#[cfg(feature = "embedded")]
pub use self::embed::{char_count};

