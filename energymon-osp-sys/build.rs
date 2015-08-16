extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let pc = pkg_config::find_library("energymon-osp-static");
    if pc.is_err() {
        let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                               .join("energymon");
        let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
        let _ = fs::create_dir(&dst);
        run(Command::new("make").arg("clean").current_dir(&src));
        run(Command::new("make").arg("energymon-osp-static").current_dir(&src));
        println!("cargo:rustc-link-lib=static=energymon-osp-static");
        println!("cargo:rustc-link-search=native={}/_build/lib", src.display())
    }
    println!("cargo:rustc-flags=-l hidapi-libusb");
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}
