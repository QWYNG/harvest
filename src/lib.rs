mod git_stash;
use clap::Clap;
use std::error::Error;
mod bm;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Arg {
    /// pattern to search
    pattern: String,
}

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {
    let mut stashes = git_stash::get_stashes()?;

    stashes.sort();
    for stash in stashes {
        match bm::search(&stash.patch_show_output, &arg.pattern) {
            Some(_) => println!("{}", stash),
            None => continue,
        }
    }

    Ok(())
}
