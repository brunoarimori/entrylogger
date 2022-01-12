use std::rc::*;

pub mod interface_cli;
pub mod persistence_textfile;

use application::*;
use domain::*;

pub fn start_entrylogger_cli_fs() {
  let config = persistence_textfile::FileSystemConfiguration {
    file_name: "entries".to_owned(),
    file_path: "./".to_owned(),
    file_current_extension: ".log".to_owned(),
    file_backup_extension: ".bak".to_owned(),
  };

  let persistence: persistence_textfile::FileSystemPersistence =
    persistence_textfile::FileSystemLoader::load(config);
  let business = EntryBusiness {};
  let controller = EntryController::new(Rc::new(business), Rc::new(persistence));
  let cli_input = interface_cli::CliInput::new(controller);
  match cli_input.listen() {
    Ok(_val) => {
      println!("Message written.");
    }
    Err(err) => {
      println!("Error: {}", err);
    }
  }
}
