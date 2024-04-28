#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

mod errors;

use std::{ffi::{CStr, CString}, os::raw, ptr::{null, null_mut}};
use anyhow::{bail, Result};
use crate::errors::{UUUError, UUUErrorCodes};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_version() -> Result<String> {
    let ver_ptr: *const raw::c_char = unsafe { uuu_get_version_string() }.into();
    let ver_str = unsafe { CStr::from_ptr(ver_ptr) };

    let version = match ver_str.to_str().map(|s| s.to_owned()) {
        Ok(v) => v,
        Err(_) => bail!(UUUError::new(UUUErrorCodes::VersionError, "failed to get version".to_owned())),
    };

    Ok(version)
}

pub fn get_last_err() -> Result<String> {
    let err_ptr: *const raw::c_char = unsafe { uuu_get_last_err_string() };
    let err_str = unsafe { CStr::from_ptr(err_ptr) };

    let error = match err_str.to_str().map(|s| s.to_owned()) {
        Ok(v) => v,
        Err(_) => bail!(UUUError::new(UUUErrorCodes::GetLastErrError, "failed to get last error".to_owned())),
    };

    Ok(error)
}

pub fn run_command(cmd: &str, dry_run: i32) -> Result<i32> {
    let cmd_str = CString::new(cmd).unwrap();
    let cmd_ptr: *const raw::c_char = cmd_str.as_ptr() as *const raw::c_char;

    let ret: raw::c_int = unsafe {
        uuu_run_cmd(cmd_ptr, dry_run.into())
    };

    Ok(ret.into())
}

pub fn run_script(script: &str, dry_run: i32) -> Result<i32> {
    let script_str = CString::new(script).unwrap();
    let script_ptr: *const raw::c_char = script_str.as_ptr() as *const raw::c_char;

    let ret: raw::c_int = unsafe {
        uuu_run_cmd_script(script_ptr, dry_run.into())
    };

    Ok(ret.into())
}

extern fn collect_device(path: *const raw::c_char,
    chip: *const raw::c_char,
    pro: *const raw::c_char,
    vid: u16,
    pid: u16,
    bcd: u16,
    p: *mut raw::c_void) -> raw::c_int {
        println!("device");
        0.into()
}
pub fn get_devices() {
    // let collect_device = |
    //     | -> raw::c_int {
    //         println!("device!");
    //         0.into()
    // };
    // extern "C" {
    //     pub fn uuu_for_each_devices(
    //         fn_: uuu_ls_usb_devices,
    //         p: *mut ::std::os::raw::c_void,
    //     ) -> ::std::os::raw::c_int;
    // }
    // let _ = unsafe {
    //     uuu_for_each_devices(Some(collect_device), null_mut())
    // };
}

// pub fn get_trustm_chipinfo() {
//     let return_status: u32 = unsafe { _trustm_Open() }.into();

//     if return_status != OPTIGA_LIB_SUCCESS {
//         println!("error opening trustm i2c interface");
//     }

//     println!("trustm open status {:?}", return_status);

//     // let m_UID = ptr::null_mut();
//     // let mut m: u32 = 2; 
//     // let p_mut: *mut u32 = &mut m;

//     let mut UID: utrustm_UID_t = utrustm_UID_t {
//         b: [0; 27],
//     };
//     let m_UID: *mut utrustm_UID_t = &mut UID;

//     let return_status: u32 = unsafe { trustm_readUID(m_UID) }.into();

//     if return_status != OPTIGA_LIB_SUCCESS {
//         println!("error reading trustm  UID");
//     }

//     println!("trustm read status {:?}", return_status);

//     let UID: utrustm_UID_t = unsafe { *m_UID };

//     println!("Chip Identifier is {:?}", unsafe { UID.st});
// }
