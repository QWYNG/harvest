use std::process::{Command, Output};
use std::str;
use std::io;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
   /// The path to grep
   path: std::path::PathBuf,
}

fn main() {
   let args = Opts::parse();

   let output = execute_git_command("stash show stash@{0} ").expect("failed");

   let str = str::from_utf8(&output.stdout).expect("cant parse");
   println!("{}", str)
}


fn execute_git_command(command: &str) -> io::Result<Output> {
   let git_command = format!("git {}", command);
   if cfg!(target_os = "windows") {
      Command::new("cmd").arg("/C").arg(git_command).output()
   } else {
      Command::new("sh").arg("-c").arg(git_command).output()
   }
}
