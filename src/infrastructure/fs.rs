use std::fs::{OpenOptions, rename};
use std::io::prelude::*;

use regex::Regex;

use application::*;
use domain::*;

static FILENAME: &'static str = "entries";

pub struct FileSystemPersistence {}

impl EntryPersistenceInterface for FileSystemPersistence {
  fn read_entries(&self) -> Result<Vec<EntryObject>, String> {
    let mut res: Vec<EntryObject> = vec![];
    // let display = path.display();
    // let mut file = File::open(&path).unwrap();
    let mut file = OpenOptions::new()
      .write(true)
      .read(true)
      .create(true)
      .open(FILENAME)
      .unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec: Vec<&str> = content.split("\n").collect();
    for split_string in string_vec {
      if split_string.len() > 0 {
        let obj = self.parse_string(split_string.to_string()).unwrap();
        res.push(obj);
      }
    }
    return Ok(res);
  }
  fn write_entry(&self, entry: EntryObject) -> Result<EntryObject, String> {
    let mut entries = self.read_entries()?;
    entries.push(entry.clone());
    entries.sort();
    rename(FILENAME, FILENAME.to_string() + ".bak").unwrap();
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(FILENAME)
      .unwrap();
    let mut writer = std::io::LineWriter::new(file);
    for item in entries {
      let entry_string = self.serialize_entry(item.clone())? + "\n";
      writer.write(entry_string.as_bytes()).unwrap();
    }
    return Ok(entry);
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
      return Err("Couldn't parse string to Entry: ".to_string() + entry_string.as_str());
    }
    let metadata_string = REGEX.captures(entry_string.as_str()).unwrap().get(1).unwrap().as_str();
    let metadata_split: Vec<&str> = metadata_string.split(' ').collect();
    let message = REGEX.split(entry_string.as_str()).collect::<Vec<&str>>()[1].trim_start();
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

