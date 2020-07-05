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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStashExecutor;
    impl MockStashExecutor {
        fn new() -> MockStashExecutor {
            MockStashExecutor {}
        }
    }
    impl GitStashExecutable for MockStashExecutor {
        fn execute_git_stash_command(&self, _command: &str) -> Result<Vec<u8>, GitError> {
            let sample =
                b"stash@{0}: WIP on refactor: cddaae4 refactor create git_stash module".to_vec();
            Ok(sample)
        }
        fn stash_show(&self, _n: usize) -> Result<Vec<u8>, GitError> {
            let sample = br#" Cargo.lock   | 21 +++++++++++++++++++++
             Cargo.toml       |  1 +
             src/git_stash.rs |  8 ++++++++
             3 files changed, 30 insertions(+)
            "#
            .to_vec();
            Ok(sample)
        }
        fn stash_patch_show(&self, _n: usize) -> Result<Vec<u8>, GitError> {
            let sample = br#"diff --git a/Cargo.lock b/Cargo.lock
            index 295fdbb..6d9592b 100644
            --- a/Cargo.lock
            +++ b/Cargo.lock
            @@ -87,6 +87,7 @@ name = "harvest"
             version = "0.1.2"
             dependencies = [
              "clap",
            + "mocktopus",
              "pager",
             ]
            "#
            .to_vec();

            Ok(sample)
        }
    }

    #[test]
    fn test_git_stash_new() {
        let mock_executor = MockStashExecutor::new();
        let list_output =
            String::from_utf8(mock_executor.execute_git_stash_command("list").unwrap()).unwrap();
        let git_stash = GitStash::new(&mock_executor, 0, list_output).unwrap();

        assert_eq!(
            GitStash {
                number: 0,
                list_output: "stash@{0}: WIP on refactor: cddaae4 refactor create git_stash module"
                    .to_string(),
                show_output: String::from_utf8(mock_executor.stash_show(0).unwrap()).unwrap(),
                patch_show_output: String::from_utf8(mock_executor.stash_patch_show(0).unwrap())
                    .unwrap(),
            },
            git_stash
        )
    }

    #[test]
    fn test_get_stashes() {
        let mock_executor = MockStashExecutor::new();
        let v = vec![GitStash {
            number: 0,
            list_output: "stash@{0}: WIP on refactor: cddaae4 refactor create git_stash module"
                .to_string(),
            show_output: String::from_utf8(mock_executor.stash_show(0).unwrap()).unwrap(),
            patch_show_output: String::from_utf8(mock_executor.stash_patch_show(0).unwrap())
                .unwrap(),
        }];

        assert_eq!(v, get_stashes(mock_executor).unwrap())
    }
}
