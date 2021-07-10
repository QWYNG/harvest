mod git_stash;
mod bm;
use clap::Clap;
use std::error::Error;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Arg {
    /// string to search
    string: String,
}

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {
    let executor = git_stash::GitStashExecutor::new();
    let mut stashes = git_stash::get_stashes(executor)?;

    stashes.sort();
    for stash in stashes {
        match bm::search(&stash.patch_show_output, &arg.string) {
            Some(_) => println!("{}", stash),
            None => continue,
        }
    }

    Ok(())
}
