extern crate chrono;
extern crate clap;
use clap::{App, Arg};
mod application;
mod domain;
mod infrastructure;

fn main() {
  println!("hello");
  let cli_input = infrastructure::CliInput::new();
  let matches = App::new("entrylogger")
    .arg(
      Arg::with_name("post")
        .long("post")
        .short("p")
        .help("posting"),
    )
    .get_matches();
  if matches.is_present("post") {
    println!("Posting new entry...");
    match cli_input.read_date() {
      Ok(value) => println!("{}", value),
      Err(error) => {
        println!("Invalid date: {}", error);
        std::process::exit(0);
      }
    }
    match cli_input.read_time() {
      Ok(value) => println!("{}", value),
      Err(error) => {
        println!("Invalid time: {}", error);
        std::process::exit(0);
      }
    }
  }
}
