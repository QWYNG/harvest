extern crate harvest;
use clap::derive::Clap;
use harvest::Arg;

fn main() {
    let arg = Arg::parse();

    if let Err(e) = harvest::run(arg) {
        eprintln!("{}", e)
    }
}
