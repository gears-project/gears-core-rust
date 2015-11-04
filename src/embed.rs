//
// Embedded config
//

#[cfg(feature = "embedded")]
pub mod embed {

    use libc::{c_char, uint32_t};
    use std::{mem, str};
    use std::collections::HashMap;
    use std::ffi::CStr;

    use xfstruct::*;

#[no_mangle]
    pub extern fn char_count(s: *const c_char) -> uint32_t {
        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };

        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
        r_str.chars().count() as uint32_t
    }

#[no_mangle]
    pub extern fn xflowstruct_new() -> *mut XFlowStruct {

        let xfs = XFlowStruct::new();

        unsafe {
            mem::transmute(Box::new(xfs))
        }
    }

#[no_mangle]
    pub extern fn xflowstruct_new_from_string(s: *const c_char) -> *mut XFlowStruct {

        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };

        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

        let xfs = XFlowStruct::from_json(r_str);

        unsafe {
            mem::transmute(Box::new(xfs))
        }
    }

#[no_mangle]
    pub extern fn xflowstruct_free(ptr: *mut XFlowStruct) {
        if ptr.is_null() {
            return
        } else {
            let _: Box<XFlowStruct> = unsafe {
                mem::transmute(ptr)
            };
        }
    }
}


