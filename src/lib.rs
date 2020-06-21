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

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {
    let oldest_stash = String::from_utf8(execute_git_command("stash list")?.stdout)?
        .lines()
        .last()
        .ok_or(NoStashError)?
        .to_string();
    let oldest_stash_number = Regex::new(r"\d+")
        .unwrap()
        .captures(&oldest_stash)
        .ok_or(NoStashError)?[0]
        .parse::<usize>()?;

    let map = create_stashes_diff_map(oldest_stash_number)?;

    let mut matched_stash_numbers: Vec<usize> = Vec::new();

    for (k, v) in map.iter() {
        match bm::search(v, &arg.pattern) {
            Some(_i) => matched_stash_numbers.push(*k),
            None => continue,
        }
    }

    Ok(print_stashes(matched_stash_numbers)?)
}

fn execute_git_command(command: &str) -> io::Result<Output> {
    let git_command = format!("git {}", command);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(git_command).output()
    } else {
        Command::new("sh").arg("-c").arg(git_command).output()
    }
}

fn create_stashes_diff_map(max: usize) -> Result<HashMap<usize, String>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for i in 0..(max + 1) {
        let diff_command = format!("diff stash@{{{}}}", i);
        let result = String::from_utf8(execute_git_command(&diff_command)?.stdout)?;
        map.insert(i, result);
    }

    Ok(map)
}

fn print_stashes(stashes_numbers: Vec<usize>) -> Result<(), Box<dyn Error>> {
    for stash_number in stashes_numbers {
        let command = format!("stash show stash@{{{}}}", stash_number);
        let show_stash_output = execute_git_command(&command)?;

        println!("stash@{{{}}}", stash_number);
        println!("{}", String::from_utf8(show_stash_output.stdout)?)
    }

    Ok(())
}
