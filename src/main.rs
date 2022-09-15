use std::process::Command;
use std::process;
use std::ffi::OsString;
use env::ext::*;

mod find_git;
mod env;

const OUR_NAME: &str = "git-brie";

// TODO create an output framework that lets us hide debug messages easily

fn main() {
    // 0. Gather information about the environment

    // Break out of infinite loops
    if find_git::is_marked() {
        // TODO display an error message and suggest uninstalling
        //      or running it again in debug mode 
        return;
    }

    let is_meta = env::our_var_os("META").but_non_empty();
    let is_debug = env::our_var_os("DEBUG").but_non_empty();
    let program_args: Vec<_> = env::program_args_os().collect();

    // Print received arguments
    if is_debug.is_some() {
        // TODO quote it correctly
        println!("[{OUR_NAME}] program args: {}", program_args.iter().map(|x| x.to_string_lossy()).collect::<Vec<_>>().join(" "));
    }

    // 1. Process the command-line arguments

    // TODO Probably turn this into function in another module

    let dont_run_git = false;
    let mut git_arguments: Option<Vec<OsString>> = None;

    // Handle `US_META=1 git --version`
    if is_meta.is_some() && program_args.len() == 1 && program_args[0] == "--version" {
        println!("{OUR_NAME} version {}", env!("CARGO_PKG_VERSION"));
    }

    // Handle `git branch --rename ...`
    if program_args.len() > 2 && program_args[0] == "branch" && program_args[1] == "--rename" {
        let mut v: Vec<OsString> = vec!["branch".into(), "--move".into()];
        v.extend_from_slice(&program_args[2..]);

        git_arguments = Some(v);
    }

    // If we decided not to run git, exit now
    if dont_run_git {
        return;
    }

    // 2. Find git

    let git_finder = find_git::GitFinder::new();
    let mut git_binaries = git_finder.get_potential_binaries();

    if is_debug.is_some() {
        for bin in git_finder.get_potential_binaries() {
            println!("[{OUR_NAME}] potential git binaries: {:?}", bin);
        }
    }

    // Display the modified git command
    if let Some(git_args) = git_arguments.as_ref() {
        // TODO quote it correctly
        println!("[{OUR_NAME}] $ git {}", git_args.iter().map(|x| x.to_string_lossy()).collect::<Vec<_>>().join(" "));
    }

    // TODO maybe somehow turn this into something less indented and more linear
    let exit_status = loop {
        match git_binaries.next() {
            Some(git_binary) => {
                if is_debug.is_some() {
                    println!("[{OUR_NAME}] trying git binary: {:?}", git_binary);
                }

                // 3. Prepare the git command

                let mut command = Command::new(git_binary);
                let command_args = git_arguments.as_ref().unwrap_or(&program_args);
                command.args(command_args.as_slice());
                find_git::mark_command(&mut command);
        
                // 4. Execute the git command
                
                let child = command.spawn();
                if let Ok(mut child) = child {
                    // TODO handle IO error
                    break child.wait().expect("failed to wait git")
                }
            },
            // TODO display error message
            None => panic!(),
        }
    };


    // 5. Cleanup and exit

    // Exit with the same code as the inner command
    if let Some(code) = exit_status.code() {
        process::exit(code)
    }
}
