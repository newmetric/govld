use std::env;
use clap::Parser;
use govld::run::Args;

fn main() {
    govld::run::do_run(
        env::current_dir().unwrap().to_str().unwrap(),
        Args::parse(),
    );
}
