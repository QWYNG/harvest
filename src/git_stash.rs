use std::error::Error;
use std::fmt;
use std::process::Command;
use std::str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitStash {
    pub number: usize,
    pub list_output: String,
    pub show_output: String,
    pub patch_show_output: String,
}

impl GitStash {
    fn new(number: usize, list_output: String) -> Result<GitStash, GitError> {
        let show_output = String::from_utf8(stash_show(number)?).unwrap();
        let patch_show_output = String::from_utf8(stash_patch_show(number)?).unwrap();
        Ok(GitStash {
            number,
            list_output,
            show_output,
            patch_show_output,
        })
    }
}

impl fmt::Display for GitStash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.list_output, self.show_output)
    }
}

#[derive(Debug)]
pub struct GitError {
    msg: String,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

pub fn get_stashes() -> Result<Vec<GitStash>, Box<dyn Error>> {
    let stash_lists = String::from_utf8(execute_git_command("stash list")?).unwrap();

    let stashes = stash_lists
        .lines()
        .enumerate()
        .map(|(number, list_output)| GitStash::new(number, String::from(list_output)).unwrap())
        .collect();
    Ok(stashes)
}

fn execute_git_command(command: &str) -> Result<Vec<u8>, GitError> {
    let git_command = format!("git {}", command);
    let result = Command::new("sh")
        .arg("-c")
        .arg(git_command)
        .output()
        .unwrap();
    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(GitError {
            msg: String::from_utf8(result.stderr).unwrap(),
        })
    }
}

fn stash_show(n: usize) -> Result<Vec<u8>, GitError> {
    let show_command = format!("stash show stash@{{{}}}", n);
    execute_git_command(&show_command)
}

fn stash_patch_show(n: usize) -> Result<Vec<u8>, GitError> {
    let patch_show_command = format!("stash show -p stash@{{{}}}", n);
    execute_git_command(&patch_show_command)
}
