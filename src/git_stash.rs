use std::error::Error;
use std::fmt;
use std::process::Command;
use std::str;

pub trait GitStashExecutable {
    fn execute_git_stash_command(&self, command: &str) -> Result<Vec<u8>, GitError>;
    fn stash_show(&self, n: usize) -> Result<Vec<u8>, GitError>;
    fn stash_patch_show(&self, n: usize) -> Result<Vec<u8>, GitError>;
}

pub struct GitStashExecutor;

impl GitStashExecutor {
    pub fn new() -> Self {
        GitStashExecutor
    }
}

impl GitStashExecutable for GitStashExecutor {
    fn execute_git_stash_command(&self, command: &str) -> Result<Vec<u8>, GitError> {
        let git_command = format!("git stash {}", command);
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

    fn stash_show(&self, n: usize) -> Result<Vec<u8>, GitError> {
        let show_command = format!("show stash@{{{}}}", n);
        self.execute_git_stash_command(&show_command)
    }

    fn stash_patch_show(&self, n: usize) -> Result<Vec<u8>, GitError> {
        let patch_show_command = format!("show -p stash@{{{}}}", n);
        self.execute_git_stash_command(&patch_show_command)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitStash {
    pub number: usize,
    pub list_output: String,
    pub show_output: String,
    pub patch_show_output: String,
}

impl GitStash {
    fn new(
        executor: &impl GitStashExecutable,
        number: usize,
        list_output: String,
    ) -> Result<GitStash, GitError> {
        let show_output = String::from_utf8(executor.stash_show(number)?).unwrap();
        let patch_show_output = String::from_utf8(executor.stash_patch_show(number)?).unwrap();
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

pub fn get_stashes(executor: impl GitStashExecutable) -> Result<Vec<GitStash>, Box<dyn Error>> {
    let stash_lists = String::from_utf8(executor.execute_git_stash_command("list")?).unwrap();

    let stashes = stash_lists
        .lines()
        .enumerate()
        .map(|(number, list_output)| {
            GitStash::new(&executor, number, String::from(list_output)).unwrap()
        })
        .collect();
    Ok(stashes)
}
