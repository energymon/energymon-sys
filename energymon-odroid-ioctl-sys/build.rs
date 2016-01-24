extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let lib = "energymon-odroid-ioctl-static";
    env::set_var("ENERGYMON_ODROID_IOCTL_STATIC_STATIC", "");
    match pkg_config::find_library(&lib) {
        Ok(_) => (),
        Err(_) => {
            let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                                   .parent().unwrap().join("energymon");
            let build = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("_build");
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            let cmake_var = match target_parts[target_parts.len() - 1].starts_with("android") {
                true => format!("-DCMAKE_TOOLCHAIN_FILE={}",
                                src.join("cmake-toolchain").join("android.toolchain.cmake").display()),
                false => "".to_owned(),
            };
            let cmake_gen = match env::var("MSYSTEM") {
                Ok(val) => {
                    if val.contains("MINGW") {
                        "-GMSYS Makefiles".to_owned()
                    } else {
                        "".to_owned()
                    }
                },
                Err(_) => "".to_owned(),
            };
            fs::remove_dir_all(&build).ok();
            fs::create_dir_all(&build).unwrap();
            run(Command::new("cmake").arg(cmake_var).arg(&cmake_gen).arg(src.to_str().unwrap()).current_dir(&build));
            run(Command::new("make").arg(&lib).current_dir(&build));
            // run pkg-config on compiled dir to get any transitive dependencies of static lib
            set_pkg_config_path(&build, !cmake_gen.is_empty());
            pkg_config::find_library(&lib).unwrap();
        },
    }
}

fn set_pkg_config_path(build: &PathBuf, is_windows: bool) {
    let delimiter = match is_windows {
        true => ";",
        false => ":",
    };
    match env::var_os("PKG_CONFIG_PATH") {
        Some(p) => {
            let path = format!("{:?}{}{:?}", build, delimiter, p);
            env::set_var("PKG_CONFIG_PATH", &path);
        },
        None => env::set_var("PKG_CONFIG_PATH", build),
    }
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}