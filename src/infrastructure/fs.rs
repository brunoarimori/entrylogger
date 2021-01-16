use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;

use application::*;
use domain::*;

pub struct FileSystemPersistence {}

impl EntryPersistenceInterface for FileSystemPersistence {
  fn read_entries(&self) -> Result<Vec<EntryObject>, String> {
    let path = Path::new("file.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
      Err(err) => {
        println!("{:?} file opening error {:?}", display, err);
        return Err("couldn't open file".to_string());
      }
      Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
      Err(err) => panic!("couldn't read {}: {}", display, err),
      Ok(_) => print!("{} contains:\n{}", display, content),
    };
    return Err("not implemented".to_string());
  }
  fn write_entry(&self, entry: EntryObject) -> Result<EntryObject, String> {
    // read all entries
    // insert entry
    // sort entries
    // backup file
    // write file
    // delete backup
    // if any errors, delete file and restore backup

    let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("file.txt")
      .unwrap();
    let entry_string = self.serialize_entry(entry.clone())? + "\n";
    let mut writer = std::io::LineWriter::new(file);
    writer.write(entry_string.as_bytes()).unwrap();
    return Ok(entry);

    /*
    match writeln!() {
      Err(why) => panic!("couldn't write {}", why),
      Ok(_) => {
        print!("ok")
      }
    }
    */
  }
}

trait EntryStringConverter {
  fn parse_string(&self, entry_string: String) -> Result<EntryObject, String>;
  fn serialize_entry(&self, entry: EntryObject) -> Result<String, String>;
}

impl EntryStringConverter for FileSystemPersistence {
  fn parse_string(&self, entry_string: String) -> Result<EntryObject, String> {
    lazy_static! {
      static ref REGEX: Regex = Regex::new(r"\[(.*?)\]").unwrap();
    };
    let match_metadata = REGEX.is_match(entry_string.as_str());
    if !match_metadata {
      return Err("Couldn't parse string to Entry".to_string());
    }
    let metadata_string = REGEX.captures(entry_string.as_str()).unwrap().get(1).unwrap().as_str();
    let metadata_split: Vec<&str> = metadata_string.split(' ').collect();
    let message = REGEX.split(entry_string.as_str()).collect::<Vec<&str>>()[1].trim_start();
    println!("string {:?}", message);
    let mut metadata = EntryMetadata {
      ins: None,
      date: "".to_string(),
      time: "".to_string(),
      tag: "".to_string(),
    };
    for meta in metadata_split {
      let metadata_split_value: Vec<&str> = meta.split(':').collect();
      match metadata_split_value[0] {
        "ins" => metadata.ins = Some(metadata_split_value[1].to_string()),
        "date" => metadata.date = metadata_split_value[1].to_string(),
        "time" => metadata.time = metadata_split_value[1].to_string(),
        "tag" => metadata.tag = metadata_split_value[1].to_string(),
        _=> return Err("Invalid value detected in metadata".to_string()),
      }
    }
    println!("{:?}", metadata);
    let result = EntryObject {
      metadata,
      message: message.to_string(),
    };
    return Ok(result);
  }
  fn serialize_entry(&self, entry: EntryObject) -> Result<String, String> {
    let entry_string = format!("[ins:{} date:{} time:{} tag:{}] {}", entry.metadata.ins.unwrap(), entry.metadata.date, entry.metadata.time, entry.metadata.tag, entry.message);
    return Ok(entry_string);
  }
}

/* -----------------------------------TESTS------------------------------------------ */
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn parse_string_test() {
    let string = "[ins:0000000000000 date:13-oct-20 time:morning tag:fit] aerobic (5/5)".to_string();
    let persistence = FileSystemPersistence {};
    let object = persistence.parse_string(string).unwrap();
    println!("{:?}", object);
    let compare_metadata = EntryMetadata {
      ins: Some("0000000000000".to_string()),
      date: "13-oct-20".to_string(),
      time: "morning".to_string(),
      tag: "fit".to_string(),
    };
    let compare_object = EntryObject {
      metadata: compare_metadata,
      message: "aerobic (5/5)".to_string(),
    };
    assert_eq!(object, compare_object);
  }
  #[test]
  fn serialize_entry() {
    let persistence = FileSystemPersistence {};
    let input_metadata = EntryMetadata {
      ins: Some("0000000000000".to_string()),
      date: "13-oct-20".to_string(),
      time: "morning".to_string(),
      tag: "fit".to_string(),
    };
    let input_object = EntryObject {
      metadata: input_metadata,
      message: "aerobic (5/5)".to_string(),
    };
    let compare_string = "[ins:0000000000000 date:13-oct-20 time:morning tag:fit] aerobic (5/5)".to_string();
    let string = persistence.serialize_entry(input_object).unwrap();
    assert_eq!(string, compare_string);
  }
}

