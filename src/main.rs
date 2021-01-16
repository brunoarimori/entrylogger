extern crate chrono;
extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;
mod application;
mod domain;
mod infrastructure;

fn main() {
  println!("hello");
  // let cli_input = infrastructure::cli::CliInput::new();
  // match cli_input.listen() {
  //   Ok(value) => println!("{:?}", value),
  //   Err(error) => {
  //     println!("Error while reading entry: {:?}", error);
  //   }
  // }
  infrastructure::test();
}
