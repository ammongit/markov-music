extern crate clap;
extern crate mpv;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termcolor;
extern crate walkdir;

use args::{Args, parse_args};
use error::Error;
use selector::Selector;
use std::io::Write;
use std::process::exit;

mod args;
mod config;
mod error;
mod markov;
mod player;
mod selector;
mod song;

#[inline]
fn print_error(err: Error) {
    writeln!(std::io::stderr(), "Error: {}", err).unwrap();
}

fn main() {
    let args: Args;
    match parse_args() {
        Ok(x) => {
            args = x;
        }
        Err(e) => {
            print_error(e);
            exit(1);
        }
    }

    let mut selector = Selector::new(&args.config);

}
