use libuuu_rs::{get_devices, get_last_err, get_version, run_command};

fn run_command_test() {
    let ex = run_command("SDPU: delay 1000", 0);
    println!("error is {}", ex.unwrap());
}

fn get_last_err_test() {
    let e = get_last_err();
}

fn get_version_test() {
    let ver = get_version();
    if ver.is_err() {
        println!("version error {:?}", ver.as_ref().err());
        return;
    }

    println!("version {}", ver.unwrap());
}

fn get_devices_test() {
    let _ = get_devices();
}

fn main() {
    run_command_test();
}
