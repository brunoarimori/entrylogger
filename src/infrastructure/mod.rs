use std::rc::*;

pub mod cli;
pub mod fs;

use application::*;
use domain::*;

pub fn test() {
  // let test_persistence = cli::TestFileSystemPersistence {};
  let test_persistence = fs::FileSystemPersistence {};
  let business = EntryBusiness {};
  let test = EntryController::new(Rc::new(business), Rc::new(test_persistence));
  // let entries = test.get_entries().unwrap();
  // println!("{:?}", entries.len());
  let cli_input = cli::CliInput::new(test);
  match cli_input.listen() {
    Ok(value) => {
      println!("{:?}", value);
    }
    Err(error) => {
      println!("Error while reading entry: {:?}", error);
    }
  }
}
