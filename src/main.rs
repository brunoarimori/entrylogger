extern crate chrono;
extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;
mod application;
mod domain;
mod infrastructure;

fn main() {
  infrastructure::start_entrylogger_cli_fs();
}
