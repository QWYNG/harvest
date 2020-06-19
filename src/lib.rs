use clap::Clap;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::process::{Command, Output};
use std::str;

#[derive(Clap)]
#[clap(version = "1.0")]
pub struct Arg {
    /// regexp to grep
    regexp: String,
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
    let oldest_stash = str::from_utf8(&execute_git_command("stash list")?.stdout)?
        .lines()
        .last()
        .ok_or(NoStashError)?
        .to_string();
    let oldest_stash_number = Regex::new(r"\d+")?
        .captures(&oldest_stash)
        .ok_or(NoStashError)?[0]
        .parse::<i32>()?;

    let map = create_stashes_diff_map(oldest_stash_number)?;

    Ok(println!("{:#?}", map))
}

fn execute_git_command(command: &str) -> io::Result<Output> {
    let git_command = format!("git {}", command);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(git_command).output()
    } else {
        Command::new("sh").arg("-c").arg(git_command).output()
    }
}

fn create_stashes_diff_map(max: i32) -> Result<HashMap<i32, String>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for i in 0..(max + 1) {
        let diff_command = format!("diff stash@{{{}}}", i);
        let result = str::from_utf8(&execute_git_command(&diff_command)?.stdout)?.to_string();
        map.insert(i, result);
    }

    Ok(map)
}
