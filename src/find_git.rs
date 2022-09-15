//! To avoid calling ourselves in an infinite loop,
//! we find the actual git binary.

use std::path::PathBuf;
use std::ffi::OsString;
use std::fs;
use std::process::Command;
use crate::env;
use env::ext::*;

pub struct GitFinder {
    path: OsString,
}

impl GitFinder {
    pub fn new() -> Self {
        Self { path: env::var_os("PATH").unwrap() }
    }

    /// Finds paths to potential git binaries that exist and aren't us 
    pub fn get_potential_binaries(&'_ self) -> impl Iterator<Item = PathBuf> + '_ {
        let git_binary = "git";
        let append_git = move |mut x: PathBuf| -> PathBuf { x.push(git_binary); x };
        let path_exists = |x: &PathBuf| -> bool { x.exists() };
        let binary_is_not_us = |x: &PathBuf| -> bool {
            match (fs::canonicalize(x), env::current_exe().and_then(fs::canonicalize)) {
                (Ok(path), Ok(us)) => path != us,
                (Err(_), _) => false,
                _ => true
            }
        };

        env::split_paths(&self.path)
            .chain(["/usr/bin".into()])
            .map(append_git)
            .filter(path_exists)
            .filter(binary_is_not_us)
    }
}

/// Mark the child process that it is run by us 
pub fn mark_command(command: &mut Command) -> &mut Command {
    command.env(env::ours("RUNNING"), "1")
}

/// The current process is marked as being run by us: infinite loop
pub fn is_marked() -> bool {
    env::our_var_os("RUNNING").but_non_empty().is_some()
}
