use std::fs;
use anyhow::{self, bail, Result};
use libuuu_rs::{get_devices, get_last_err, get_version, register_notify_callback, run_command};

use crate::errors::{UUUError, UUUErrorCodes};

mod errors;


// fn exec_command_test() {
//     let exec = libuuu_rs::run_command("SDPV: delay 1000", 0);
//     if exec.is_err() {
//         println!("error running command");
//         return;
//     }
//     let code = exec.unwrap();

//     if code < 0 {
//         println!("uuu returned error status, err - {:?}", code);
//     }
// }

fn exec_command(cmd: String) -> Result<i32> {
    let exec = run_command(cmd.as_str(), 0);
    if exec.is_err() {
        bail!(UUUError::new(
            UUUErrorCodes::CommandError,
            format!("error running command, err - {:?}, cmd - {}", exec.err(), cmd),
        ))
    }
    let code = exec.unwrap();

    if code < 0 {
        bail!(UUUError::new(
            UUUErrorCodes::CommandError,
            format!("uuu returned error status, status - {:?}, cmd - {}", code, cmd),
        ))
    }

    Ok(code)
}

// fn run_script_test(script: String) {
//     let ex = run_script(&script, 0);
//     println!("error is {}", ex.unwrap());
// }

// fn get_last_err_test() {
//     let e = get_last_err();
//     println!("last err is {:?}", e.unwrap());
// }

// fn get_version_test() {
//     let ver = get_version();
//     if ver.is_err() {
//         println!("version error {:?}", ver.as_ref().err());
//         return;
//     }

//     println!("version {}", ver.unwrap());
// }

// fn get_devices_test() {
//     let _ = get_devices();
// }

fn exec_script(filename: String) -> Result<bool> {
    for line in fs::read_to_string(filename).unwrap().lines() {
        let cmd = line.to_string();

        // ignore version banner
        if cmd.starts_with("uuu_version") {
            continue;
        }

        // ignore comment
        if cmd.starts_with("#") {
            continue;
        }

        // ignore empty
        if cmd.trim() == "" {
            continue;
        }

        println!("running command - {}", cmd);

        let res = exec_command(cmd);

        if res.is_err() {
            // print last_error and exit
            let last_err = get_last_err();
            bail!(UUUError::new(
                UUUErrorCodes::CommandError,
                format!("error executing script script, error - {:?}, message - {}", res.err(), last_err.unwrap_or("".to_string()) ),
            ))
        }
    };

    Ok(true)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Script path missing in args");
        return;
    }

    // register notify callback
    register_notify_callback();

    let script_path = &args[1];
    let res  = exec_script(script_path.clone());

    if res.is_err() {
        println!("script exec failed, exiting.");
        return;
    }

    println!("script exec success");
}
