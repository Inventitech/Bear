/*  Copyright (C) 2012-2018 by László Nagy
    This file is part of Bear.

    Bear is a tool to generate compilation database for clang tooling.

    Bear is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Bear is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::InterceptMode;

pub const KEY_CC: &str = "CC";
pub const KEY_CXX: &str = "CXX";

pub const KEY_INTERCEPT_CC: &str = "INTERCEPT_CC";
pub const KEY_INTERCEPT_CXX: &str = "INTERCEPT_CXX";

pub const KEY_PARENT: &str = "INTERCEPT_PARENT_PID";

pub const KEY_LIBRARY: &str = "INTERCEPT_LIBRARY";
pub const KEY_REPORTER: &str = "INTERCEPT_REPORT_COMMAND";
pub const KEY_DESTINATION: &str = "INTERCEPT_REPORT_DESTINATION";
pub const KEY_VERBOSE: &str = "INTERCEPT_VERBOSE";

pub const KEY_OSX_PRELOAD: &str = "DYLD_INSERT_LIBRARIES";
pub const KEY_OSX_NAMESPACE: &str = "DYLD_FORCE_FLAT_NAMESPACE";
pub const KEY_GLIBC_PRELOAD: &str = "LD_PRELOAD";

pub type Environment = std::collections::HashMap<String, String>;

pub struct EnvironmentBuilder {
    state: Box<Environment>,
}

impl EnvironmentBuilder {

    fn new() -> EnvironmentBuilder {
        unimplemented!()
    }

    fn from(_environment: &Environment) -> EnvironmentBuilder {
        unimplemented!()
    }

    pub fn build(&self) -> Environment {
        unimplemented!()
    }

    pub fn with_modes(&mut self, modes: &[InterceptMode]) -> &mut EnvironmentBuilder {
        for mode in modes {
            match mode {
                InterceptMode::Library(path) =>
                    self.add_library(path),
                InterceptMode::WrapperCC { compiler, wrapper, .. } =>
                    self.add_cc_wrapper(compiler, wrapper),
                InterceptMode::WrapperCXX { compiler, wrapper, .. } =>
                    self.add_cxx_wrapper(compiler, wrapper),
            }
        }
        self
    }

    #[cfg(any(target_os = "android",
              target_os = "freebsd",
              target_os = "linux"))]
    fn add_library(&mut self, library: &std::path::Path) {
        self.insert_path(KEY_LIBRARY, library);
        self.add_preload(KEY_GLIBC_PRELOAD, library);
    }

    #[cfg(target_os = "macos")]
    fn add_library(&mut self, library: &std::path::Path) {
        self.insert_path(KEY_LIBRARY, library);
        self.insert_str(KEY_OSX_NAMESPACE, "1");
        self.add_preload(KEY_OSX_PRELOAD, library);
    }

    #[cfg(not(unix))]
    fn add_library(&mut self, library: &std::path::Path) {
        info!("preload library ignored");
    }

    fn add_preload(&mut self, key: &str, library: &std::path::Path) {
        unimplemented!()
    }

    fn add_cc_wrapper(&mut self, compiler: &std::path::Path, wrapper: &std::path::Path) {
        self.insert_path(KEY_INTERCEPT_CC, compiler);
        self.insert_path(KEY_CC, wrapper);
    }

    fn add_cxx_wrapper(&mut self, compiler: &std::path::Path, wrapper: &std::path::Path) {
        self.insert_path(KEY_INTERCEPT_CXX, compiler);
        self.insert_path(KEY_CXX, wrapper);
    }

    pub fn with_verbose(&mut self, verbose: bool) -> &mut EnvironmentBuilder {
        if verbose {
            self.insert_str(KEY_VERBOSE, "1");
        }
        self
    }

    pub fn with_destination(&mut self, destination: &std::path::Path) -> &mut EnvironmentBuilder {
        self.insert_path(KEY_DESTINATION, destination);
        self
    }

    fn insert_path(&mut self, key: &str, value: &std::path::Path) {
        value.to_str()
            .iter()
            .for_each(|value_str| { self.insert_str(key, value_str); });
    }

    fn insert_str(&mut self, key: &str, value: &str) {
        self.state.insert(key.to_string(), value.to_string());
    }
}
