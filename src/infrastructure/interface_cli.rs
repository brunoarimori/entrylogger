use std::io::prelude::*;

use chrono::{prelude::*, Duration};
use clap::{App, Arg};

use application::*;
use domain::*;

struct InputParsing {}
impl InputParsing {
  fn parse_date(&self, date: String) -> String {
    match date.as_str() {
      "today" => {
        let today = Local::today().format("%d-%b-%y").to_string().to_lowercase();
        return today;
      }
      "yesterday" => {
        let yesterday = (Local::today() - Duration::days(1))
          .format("%d-%b-%y")
          .to_string()
          .to_lowercase();
        return yesterday;
      }
      _ => {
        return date.to_string();
      }
    }
  }
  fn parse_time(&self, time: String) -> String {
    let now = Local::now().hour();
    let midnight = 0;
    let six = 6;
    let midday = 12;
    let afternoon = 18;
    match time.as_str() {
      "now" => {
        if (now >= midnight) && (now < six) {
          return "latenight".to_string();
        } else if (now >= six) && (now < midday) {
          return "morning".to_string();
        } else if (now >= midday) && (now < afternoon) {
          return "afternoon".to_string();
        } else {
          return "night".to_string();
        }
      }
      _ => return time.to_string(),
    }
  }
}

pub struct CliInput {
  input_parsing: InputParsing,
  entry_business: EntryBusiness,
  entry_controller: EntryController,
}
impl CliInput {
  fn parse_new_line(str: String) -> String {
    return str
      .trim_end_matches("\n")
      .trim_end_matches("\r")
      .to_string();
  }
  pub fn new(entry_controller: EntryController) -> CliInput {
    let input_parsing = InputParsing {};
    let entry_business = EntryBusiness {};
    return CliInput {
      input_parsing,
      entry_business,
      entry_controller,
    };
  }
  fn read_date(&self) -> Result<String, String> {
    print!("date > ");
    std::io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        let date = self.input_parsing.parse_date(Self::parse_new_line(buf));
        return Ok(date.to_string());
      }
      Err(_error) => return Err("couldn't read date".to_string()),
    }
  }
  fn read_time(&self) -> Result<String, String> {
    print!("time > ");
    std::io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        let time = self.input_parsing.parse_time(Self::parse_new_line(buf));
        return Ok(time.to_string());
      }
      Err(_error) => return Err("couldn't read time".to_string()),
    }
  }
  fn read_tag(&self) -> Result<String, String> {
    print!("tag > ");
    std::io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        return Ok(Self::parse_new_line(buf));
      }
      Err(_error) => return Err("couldn't read tag".to_string()),
    }
  }
  fn read_message(&self) -> Result<String, String> {
    print!("message > ");
    std::io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        return Ok(Self::parse_new_line(buf));
      }
      Err(_error) => return Err("couldn't read message".to_string()),
    }
  }
  pub fn listen(&self) -> Result<EntryObject, String> {
    let matches = App::new("entrylogger")
      .arg(
        Arg::with_name("post")
          .long("post")
          .short("p")
          .help("posting"),
      )
      .get_matches();
    if matches.is_present("post") {
      let date = self
        .entry_business
        .validate_date(self.read_date().unwrap().as_str())
        .or_else(|err| match err.code() {
          DomainErrorCode::InvalidFormat => {
            return Err("Expected one of the following: today, yesterday or <dd-mon-yy> format")
          }
          _ => return Err("Couldn't parse date."),
        })?;
      let time = self
        .entry_business
        .validate_time(self.read_time().unwrap().as_str())
        .or_else(|err| match err.code() {
          DomainErrorCode::InvalidFormat => {
            return Err(
              "Expected one of the following: morning, afternoon, night, latenight, n/a or now",
            )
          }
          _ => return Err("Couldn't parse time."),
        })?;
      let tag = self
        .entry_business
        .validate_tag(self.read_tag().unwrap().as_str())
        .or_else(|err| {
          match err.code() {
            DomainErrorCode::InvalidFormat => {
              return Err("Only lowercase alphanumerical characters allowed in tag")
            }
            DomainErrorCode::MaxLengthExceeded => return Err("Maximum length allowed for tag: 12"), // TODO: better error handling
            _ => return Err("Couldn't parse tag."),
          }
        })?;
      let message = self
        .entry_business
        .validate_message(self.read_message().unwrap().as_str())
        .or_else(|err| {
          match err.code() {
            DomainErrorCode::InvalidFormat => return Err("Invalid characters found in message"),
            DomainErrorCode::MaxLengthExceeded => {
              return Err("Maximum length allowed for message: 32")
            } // TODO: better error handling
            _ => return Err("Couldn't parse message."),
          }
        })?;
      let entry_metadata = EntryMetadata {
        date,
        time,
        ins: None,
        tag,
      };
      let entry_object = EntryObject {
        metadata: entry_metadata,
        message,
      };
      return self.entry_controller.post_entry(entry_object);
    } else {
      return Err("No args".to_string());
    }
  }
}

/* -----------------------------------TESTS------------------------------------------ */
pub struct TestFileSystemPersistence {}
impl EntryPersistenceInterface for TestFileSystemPersistence {
  fn read_entries(&self) -> Result<Vec<EntryObject>, String> {
    let mut mock_vec = vec![];
    for _i in 0..36500 {
      let mock_metadata = EntryMetadata {
        date: "10-jan-20".to_string(),
        time: "morning".to_string(),
        ins: Some("0000000000000".to_string()),
        tag: "mock metadata".to_string(),
      };
      let mock_object = EntryObject {
        metadata: mock_metadata,
        message: "mock message".to_string(),
      };
      mock_vec.push(mock_object);
    }
    return Ok(mock_vec);
  }
  fn write_entry(&self, entry: EntryObject) -> Result<EntryObject, String> {
    return Ok(entry);
  }
}
