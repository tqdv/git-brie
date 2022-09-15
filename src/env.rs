pub use std::env::*;
use std::ffi::OsString;

const ENV_PREFIX: &str = "GIT_BRIE_";

pub fn ours(name: &str) -> String {
    String::from(ENV_PREFIX) + name
}

pub fn our_var_os(name: &str) -> Option<OsString> {
    var_os(ours(name))
}

pub fn program_args_os() -> impl Iterator<Item = OsString> {
    // NB args[0] is the program name
    args_os().skip(1)
}

pub mod ext {
    use std::ffi::OsString;
    
    pub trait OptionOsStringExtension {
        /// Consider that empty vars and unset vars are the same
        fn but_non_empty(self) -> Self;
    }

    impl OptionOsStringExtension for Option<OsString> {
        fn but_non_empty(self) -> Self {
            self.filter(|x| !x.is_empty())
        }
    }
}
