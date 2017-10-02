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
            cmake_config.define("ENERGYMON_BUILD_LIB", &lib);
            cmake_config.define("ENERGYMON_BUILD_SHMEM_PROVIDERS", "False");
            cmake_config.define("ENERGYMON_BUILD_UTILITIES", "False");
            cmake_config.define("ENERGYMON_BUILD_TESTS", "False");
            cmake_config.define("ENERGYMON_BUILD_EXAMPLES", "False");
            // handle default impl
            match lib {
                "default" | "energymon-default" => {
                    match env::var_os("ENERGYMON_DEFAULT_IMPL") {
                        // build requested default impl
                        Some(d) => cmake_config.define("ENERGYMON_BUILD_DEFAULT", d),
                        // default to dummy impl
                        None => cmake_config.define("ENERGYMON_BUILD_DEFAULT", "energymon-dummy")
                    }
                },
                // don't build default impl
                _ => cmake_config.define("ENERGYMON_BUILD_DEFAULT", "NONE")
            };
            if let Ok(msystem) = env::var("MSYSTEM") {
                if msystem.contains("MINGW") {
                    cmake_config.generator("MSYS Makefiles");
                }
            }
            // run the build commands
            let install_path = cmake_config.build();
            // run pkg-config on compiled dir to get any transitive dependencies of static lib
            set_pkg_config_path(install_path.join("lib").join("pkgconfig"));
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
