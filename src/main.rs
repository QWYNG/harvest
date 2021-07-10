use clap::Clap;
use harvest::Arg;
use pager::Pager;

fn main() {
    Pager::new().setup();

    let arg = Arg::parse();

    if let Err(e) = harvest::run(arg) {
        eprintln!("{}", e)
    }
}
