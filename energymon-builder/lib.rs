extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

pub fn find_or_build(lib: &str) -> Result<(), String> {
    // force behavior for static libraries (risks over-linking, but better than missing transitive dependencies)
    env::set_var("PKG_CONFIG_ALL_STATIC", "");
    match pkg_config::find_library(lib) {
        Ok(_) => Ok(()),
        Err(_) => {
            // get source and build directories
            let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                                   .parent().unwrap().join("energymon");
            let build = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("_build");
            // get extra CMake parameters
            let default_impl = match env::var_os("ENERGYMON_DEFAULT_IMPL") {
                Some(d) => format!("-DDEFAULT={}", d.to_str().unwrap()),
                None => "".to_owned(),
            };
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            let cmake_toolchain = match target_parts[target_parts.len() - 1].starts_with("android") {
                true => format!("-DCMAKE_TOOLCHAIN_FILE={}",
                                src.join("cmake-toolchain").join("android.toolchain.cmake").display()),
                false => "".to_owned(),
            };
            let cmake_mingw = match env::var("MSYSTEM") {
                Ok(val) => {
                    if val.contains("MINGW") {
                        "-GMSYS Makefiles".to_owned()
                    } else {
                        "".to_owned()
                    }
                },
                Err(_) => "".to_owned(),
            };
            // always remake the build directory
            fs::remove_dir_all(&build).ok();
            fs::create_dir_all(&build).unwrap();
            // run the build commands
            run(Command::new("cmake").arg(default_impl).arg("-DBUILD_SHARED_LIBS=false").arg(cmake_toolchain)
                .arg(&cmake_mingw).arg(src.to_str().unwrap()).current_dir(&build));
            run(Command::new("make").arg(lib).current_dir(&build));
            // run pkg-config on compiled dir to get any transitive dependencies of static lib
            set_pkg_config_path(build);
            // this might be a cross-compile, so we need to force the search
            env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
            pkg_config::find_library(lib).map(|_| ())
        },
    }
}

fn set_pkg_config_path(build: PathBuf) {
    let var = match env::var_os("PKG_CONFIG_PATH") {
        Some(p) => {
            let mut paths = env::split_paths(&p).collect::<Vec<_>>();
            paths.push(build);
            env::join_paths(paths).unwrap()
        },
        None => build.into_os_string(),
    };
    env::set_var("PKG_CONFIG_PATH", &var);
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}
