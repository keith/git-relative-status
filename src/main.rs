extern crate pathdiff;

use pathdiff::diff_paths;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

fn run_git_command(args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to run git command");

    if !output.status.success() {
        panic!("Command failed");
    };

    let stdout = from_utf8(&output.stdout)
        .expect("Failed to decode string")
        .trim();

    if stdout.is_empty() {
        return None;
    };

    return Some(String::from(stdout));
}

fn main() {
    let git_dir = match run_git_command(&["rev-parse", "--show-toplevel"]) {
        Some(output) => PathBuf::from(&output),
        None => panic!("Not in git directory"),
    };

    let status = match run_git_command(&["status", "--porcelain"]) {
        Some(output) => output,
        None => panic!("No status"),
    };

    let pwd = current_dir().expect("Couldn't fetch pwd");
    let path_from_root =
        diff_paths(&pwd, &git_dir).expect("Both paths should be absolute");

    for line in status.lines().map(|x| x.trim()) {
        let idx = line
            .find(" ")
            .expect("No space in line, unexpected output from git status");
        let (file_status, file) = line.split_at(idx);
        if file_status != "D" {
            let file_path = Path::new(file.trim());
            let relative_path = diff_paths(file_path, &path_from_root)
                .expect("File must be in git repo");
            println!("{}", relative_path.display());
        }
    }
}
