// Embedded config
//

#[cfg(feature = "embedded")]
pub mod embed {

    use libc::{c_char, uint32_t};
    use std::{mem, str};
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};


    use xfstruct::*;
    use errors::*;
    use validation::*;

    // #[no_mangle]
    // #[repr(C)]
    //     pub struct extern "C"alValidationError {
    //         pub code:    i32,
    //         pub message: String,
    //         pub paths:Vec<CString>,
    //     }
    //
    //     // let mut paths: Vec<*const libc::c_char> = vec![];
    //     //
    //     // paths.push(CString::new(string_var.value.to_string()).unwrap().into_ptr());
    //
    //
    //     fn convert_validation_to_extern "C"al(val:ValidationError) -> ExternalValidationError {
    //
    //         let mut paths: Vec<*const c_char> = vec![];
    //
    //         for path in val.paths.iter() {
    //             paths.push(
    //                 CString::new(
    //                     path.value.to_string()
    //                     ).unwrap().into_ptr()
    //                 );
    //         }
    //
    //         extern "C"alValidationError {
    //             code: val.code,
    //             message: val.message,
    //             paths: paths
    //         }
    //
    //     }
    //
    #[no_mangle]
    pub extern "C" fn char_count(s: *const c_char) -> uint32_t {
        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };

        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
        r_str.chars().count() as uint32_t
    }

    #[no_mangle]
    pub extern "C" fn xflowstruct_new() -> *mut XFlowStruct {

        let xfs = XFlowStruct::default();

        unsafe { mem::transmute(Box::new(xfs)) }
    }

    #[no_mangle]
    pub extern "C" fn xflowstruct_new_from_string(s: *const c_char) -> *mut XFlowStruct {

        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };

        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

        let xfs = XFlowStruct::from_json(r_str);

        unsafe { mem::transmute(Box::new(xfs)) }
    }

    #[no_mangle]
    pub extern "C" fn xflowstruct_free(ptr: *mut XFlowStruct) {
        if ptr.is_null() {
            return;
        } else {
            let _: Box<XFlowStruct> = unsafe { mem::transmute(ptr) };
        }
    }

    #[no_mangle]
    pub extern "C" fn xflowstruct_to_string(ptr: *mut XFlowStruct) -> *const c_char {
        let xfs: Box<XFlowStruct> = unsafe {
            assert!(!ptr.is_null());
            mem::transmute(ptr)
        };

        let to_print = CString::new(xfs.to_string()).unwrap();
        // let to_print = CString::new("Zork").unwrap();
        to_print.into_raw()
    }


    fn to_c_string(rust_string: String) -> *const libc::c_char {
        ffi::CString::from_slice(rust_string.as_bytes()).as_ptr()
    }


    #[no_mangle]
    pub extern "C" fn cleanup_string(s: *mut c_char) {
        unsafe {
            assert!(!s.is_null());
            CString::from_raw(s)
        };
    }
}
