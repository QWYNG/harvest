use clap::Clap;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::process::{Command, Output};
use std::str;
mod bm;

#[derive(Clap)]
#[clap(version = "1.0")]
pub struct Arg {
    /// pattern to search
    pattern: String,
}

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {
    let stashes_diff_map = create_stashes_diff_map()?;

    let mut matched_stashes: Vec<&str> = Vec::new();

    for (stash, diff) in stashes_diff_map.iter() {
        match bm::search(diff, &arg.pattern) {
            Some(_i) => matched_stashes.push(stash),
            None => continue,
        }
    }

    Ok(print_stashes(matched_stashes)?)
}

fn execute_git_command(command: &str) -> io::Result<Output> {
    let git_command = format!("git {}", command);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(git_command).output()
    } else {
        Command::new("sh").arg("-c").arg(git_command).output()
    }
}

#[derive(Debug, Clone)]
struct NoStashError;

impl fmt::Display for NoStashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is no stash")
    }
}

impl Error for NoStashError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

fn create_stashes_diff_map() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let stash_lists = String::from_utf8(execute_git_command("stash list")?.stdout)?;

    if stash_lists.len() == 0 {
        return Err(Box::new(NoStashError));
    }

    let mut map = HashMap::new();

    for stash in stash_lists.lines() {
        let stash_number = capture_first_number(stash).unwrap();
        let diff_command = format!("diff stash@{{{}}}", stash_number);
        let result = String::from_utf8(execute_git_command(&diff_command)?.stdout)?;
        map.insert(stash.to_string(), result);
    }

    Ok(map)
}

fn capture_first_number(str: &str) -> Option<usize> {
    let regex = Regex::new(r"\d+").unwrap();

    match regex.captures(&str) {
        Some(captures) => Some(captures[0].parse::<usize>().unwrap()),
        None => None,
    }
}

fn print_stashes(mut stashes: Vec<&str>) -> Result<(), Box<dyn Error>> {
    stashes.sort();
    for stash in stashes {
        let command = format!(
            "stash show stash@{{{}}}",
            capture_first_number(stash).unwrap()
        );
        let show_stash_output = execute_git_command(&command)?;

        println!("{}", stash);
        println!("{}", String::from_utf8(show_stash_output.stdout)?)
    }

    Ok(())
}
