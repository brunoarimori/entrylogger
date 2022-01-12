use std::rc::*;

pub mod cli;
pub mod fs;

use application::*;
use domain::*;

pub fn start_entrylogger_cli_fs() {
  // let persistence = fs::FileSystemPersistence {};
  let config = fs::FileSystemConfiguration {
    file_name: "entries".to_owned(),
    file_path: "./".to_owned(),
    file_current_extension: ".log".to_owned(),
    file_backup_extension: ".bak".to_owned(),
  };

  let persistence: fs::FileSystemPersistence = fs::FileSystemLoader::load(config);
  let business = EntryBusiness {};
  let controller = EntryController::new(Rc::new(business), Rc::new(persistence));
  let cli_input = cli::CliInput::new(controller);
  match cli_input.listen() {
    Ok(_val) => {
      println!("Message written.");
    }
    Err(err) => {
      println!("Error: {}", err);
    }
  }
}
