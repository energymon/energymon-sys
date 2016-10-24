extern crate pkg_config;
extern crate cmake;

use std::env;
use std::fs::{self};
use std::path::PathBuf;

pub fn find_or_build(lib: &str) -> Result<(), String> {
    // force behavior for static libraries (risks over-linking, but better than missing transitive dependencies)
    env::set_var("PKG_CONFIG_ALL_STATIC", "");
    match pkg_config::find_library(lib) {
        Ok(_) => Ok(()),
        Err(_) => {
            // get source and build directories
            let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                                   .parent().unwrap().join("energymon");
            let build = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("build");
            let mut cmake_config = cmake::Config::new(&src);
            // always remake the build directory
            fs::remove_dir_all(&build).ok();
            fs::create_dir_all(&build).unwrap();
            // set CMake parameters
            cmake_config.build_target(&lib);
            if let Some(default_impl) = env::var_os("ENERGYMON_DEFAULT_IMPL") {
                cmake_config.define("DEFAULT", default_impl.to_str().unwrap());
            }
            if let Ok(msystem) = env::var("MSYSTEM") {
                if msystem.contains("MINGW") {
                    cmake_config.generator("MSYS Makefiles");
                }
            }
            // run the build commands
            cmake_config.build();
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
