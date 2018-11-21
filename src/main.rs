extern crate pathdiff;

fn run_git_command(args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .expect("Failed to run git command");

    if !output.status.success() {
        return None;
    };

    let stdout = std::str::from_utf8(&output.stdout)
        .expect("Failed to decode string")
        .trim();

    if stdout.is_empty() {
        return None;
    };

    return Some(String::from(stdout));
}

fn new_file_from_rename(line: &str) -> String {
    let files: Vec<&str> = line.split(" -> ").collect();
    assert!(files.len() == 2, "Had file with '->' in the name");
    return files.last().unwrap().to_string();
}

#[test]
fn test_simple_rename() {
    assert_eq!(new_file_from_rename("foo.txt -> bar.txt"), "bar.txt");
}

#[test]
#[should_panic(expected = "Had file with '->' in the name")]
fn test_complex_rename() {
    new_file_from_rename("\"foo -> bar.txt\" -> baz.txt");
}

fn file_path_for_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let idx = trimmed
        .find(char::is_whitespace)
        .expect("No space in line, unexpected output from git status");
    let (file_status, file) = trimmed.split_at(idx);

    match file_status {
        "R" | "C" => Some(new_file_from_rename(&file.trim())),
        _ => Some(file.trim().to_string()),
    }
    .map(|x| x.trim_matches('"').to_string())
}

#[test]
fn test_path_for_modified_line() {
    assert_eq!(
        file_path_for_line(" M foo.txt"),
        Some("foo.txt".to_string())
    );
}

#[test]
fn test_path_for_renamed_line() {
    assert_eq!(
        file_path_for_line(" R foo.txt -> bar.txt"),
        Some("bar.txt".to_string())
    );
}

#[test]
fn test_path_for_changed_line() {
    assert_eq!(
        file_path_for_line(" C foo.txt -> bar.txt"),
        Some("bar.txt".to_string())
    );
}

#[test]
fn test_path_for_deleted_line() {
    assert_eq!(
        file_path_for_line(" D foo.txt"),
        Some("foo.txt".to_string())
    );
}

fn paths_for_lines<'a, I>(lines: I) -> Vec<std::path::PathBuf>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .into_iter()
        .filter_map(file_path_for_line)
        .map(std::path::PathBuf::from)
        .collect()
}

#[test]
fn test_getting_paths_from_lines() {
    let lines = vec![
        " M foo.txt",
        " M \"bar baz.txt\"",
        " R qux.txt -> quxx.txt",
        " D deleted.txt",
    ];

    let expected: Vec<std::path::PathBuf> =
        ["foo.txt", "bar baz.txt", "quxx.txt", "deleted.txt"]
            .iter()
            .map(std::path::PathBuf::from)
            .collect();

    assert_eq!(paths_for_lines(lines.into_iter()), expected)
}

fn main() {
    let git_dir = match run_git_command(&["rev-parse", "--show-toplevel"]) {
        Some(output) => std::path::PathBuf::from(&output),
        None => {
            eprintln!("Not in git repo");
            std::process::exit(1);
        }
    };

    let status = match run_git_command(&["status", "--porcelain"]) {
        Some(output) => output,
        None => std::process::exit(0),
    };

    let pwd = std::env::current_dir().expect("Couldn't fetch pwd");
    let path_from_root = pathdiff::diff_paths(&pwd, &git_dir)
        .expect("Both paths should be absolute");

    for file_path in paths_for_lines(status.lines()) {
        let relative_path = pathdiff::diff_paths(&file_path, &path_from_root)
            .expect("File must be in git repo");
        println!("\"{}\"", relative_path.display());
    }
}
