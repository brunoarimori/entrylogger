use chrono::prelude::*;
use regex::Regex;
use std::{clone::Clone, usize};
use std::cmp::Ordering;
use std::fmt::Debug;
// use self::DomainError;

static TIME_VEC: &'static [&str] = &["latenight", "morning", "afternoon", "night", "n/a"];
const INS_LENGTH: usize = 13;
const MAX_TAG_LENGTH: usize = 12;
const MAX_MSG_LENGTH: usize = 32;
lazy_static! {
  static ref ALPHA_REGEX: Regex = Regex::new(r"^[a-z0-9]+$").unwrap();
  static ref MSG_REGEX: Regex = Regex::new(r"^[A-z \+-=.,:_\\/\(\)<> \$]+$").unwrap();
}
// const TAG_VALIDATION_ERROR: &str = "error";

#[derive(Clone)]
pub enum DomainErrorCode {
  InvalidFormat,
  MaxLengthExceeded,
  MissingIns,
}

pub struct DomainError {
  code: DomainErrorCode,
  message: String,
}

impl DomainError {
  pub fn new(code: DomainErrorCode, message: String) -> DomainError {
    return DomainError {
      code,
      message,
    }
  }
  pub fn code(&self) -> DomainErrorCode {
    return self.code.to_owned();
  }
  pub fn message(&self) -> String {
    return self.message.to_owned();
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryMetadata {
  pub ins: Option<String>,
  pub date: String,
  pub time: String,
  pub tag: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryObject {
  pub metadata: EntryMetadata,
  pub message: String,
}

impl Ord for EntryMetadata {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.date != other.date {
      let dt_self = NaiveDate::parse_from_str(self.date.as_str(), "%d-%b-%y").unwrap();
      let dt_other = NaiveDate::parse_from_str(other.date.as_str(), "%d-%b-%y").unwrap();
      return dt_self.cmp(&dt_other);
    }
    if self.time != other.time {
      let self_time_index = TIME_VEC
        .iter()
        .position(|&time_str| -> bool { time_str == self.time });
      let other_time_index = TIME_VEC
        .iter()
        .position(|&time_str| -> bool { time_str == other.time });
      return self_time_index.cmp(&other_time_index);
    }
    if self.ins.is_none() {
      return Ordering::Greater;
    }
    if other.ins.is_none() {
      return Ordering::Less;
    }
    if self.ins != other.ins {
      let ins_self = self.ins.as_ref().unwrap().parse::<i64>().unwrap();
      let ins_other = other.ins.as_ref().unwrap().parse::<i64>().unwrap();
      return ins_self.cmp(&ins_other);
    }
    return self.tag.cmp(&other.tag);
  }
}
impl PartialOrd for EntryMetadata {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}
impl Ord for EntryObject {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.metadata == other.metadata {
      return self.message.cmp(&other.message);
    } else {
      return self.metadata.cmp(&other.metadata);
    }
  }
}
impl PartialOrd for EntryObject {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}

pub trait EntryValidator {
  fn validate_tag(&self, tag: &str) -> Result<String, DomainError>;
  fn validate_date(&self, date: &str) -> Result<String, DomainError>;
  fn validate_time(&self, time: &str) -> Result<String, DomainError>;
  fn validate_ins(&self, ins: &str) -> Result<String, DomainError>;
  fn validate_message(&self, message: &str) -> Result<String, DomainError>;
}

pub trait EntryHandler: EntryValidator {
  fn validate(&self, entry: &EntryObject) -> Result<String, DomainError>;
}

pub struct EntryBusiness {}
impl EntryValidator for EntryBusiness {
  fn validate_tag(&self, tag: &str) -> Result<String, DomainError> {
    if !ALPHA_REGEX.is_match(tag) {
      // return Err("Only lowercase alphanumerical characters allowed in tag".to_string());
      // return Err(DomainError::InvalidFormat);
      return Err(DomainError::new(DomainErrorCode::InvalidFormat, "Only lowercase alphanumerical characters allowed in tag".to_string()));
    };
    if tag.len() > MAX_TAG_LENGTH {
      // return Err("Maximum length allowed for tag: ".to_string() + &MAX_TAG_LENGTH.to_string());
      // return Err(DomainError::MaxLengthExceeded);
      return Err(DomainError::new(DomainErrorCode::MaxLengthExceeded, "Maximum length for tag is ".to_string() + &MAX_TAG_LENGTH.to_string()));
    };
    return Ok(tag.to_string());
  }
  fn validate_date(&self, date: &str) -> Result<String, DomainError> {
    match chrono::NaiveDate::parse_from_str(date, "%d-%b-%y") {
      Ok(value) => return Ok(value.format("%d-%b-%y").to_string().to_lowercase()),
      Err(_err) => {
        // return Err("Expected one of the following: today, yesterday or <dd-mon-yy> format".to_string());
        // return Err(DomainError::InvalidFormat)
        return Err(DomainError::new(DomainErrorCode::InvalidFormat, "Expected <dd-mon-yy>".to_string()));
      }
    };
  }
  fn validate_time(&self, time: &str) -> Result<String, DomainError> {
    if TIME_VEC.contains(&time) {
      return Ok(time.to_string());
    } else {
      // return Err("Expected one of the following: morning, afternoon, night, latenight, n/a or now".to_string());
      return Err(DomainError::new(DomainErrorCode::InvalidFormat, "Expected one of the following: morning, afternoon, night, latenight, n/a".to_string()));
    }
  }
  fn validate_ins(&self, ins: &str) -> Result<String, DomainError> {
    if ins.len() != INS_LENGTH {
      return Err(DomainError::new(DomainErrorCode::MaxLengthExceeded, "Maximum length for ins is ".to_string() + &INS_LENGTH.to_string()));
    }
    let check = ins.parse::<u64>();
    match check {
      Ok(_ok) => return Ok(ins.to_string()),
      Err(_err) => return Err(DomainError::new(DomainErrorCode::InvalidFormat, "Ins must be a number".to_string())),
    };
  }
  fn validate_message(&self, message: &str) -> Result<String, DomainError> {
    if !MSG_REGEX.is_match(message) {
      return Err(DomainError::new(DomainErrorCode::InvalidFormat, "Invalid characters found in message".to_string()));
    };
    if message.len() > MAX_MSG_LENGTH {
      return Err(DomainError::new(DomainErrorCode::MaxLengthExceeded, "Maximum length for message is ".to_string() + &MAX_MSG_LENGTH.to_string()));
    };
    return Ok(message.to_string());
  }
}
impl EntryHandler for EntryBusiness {
  fn validate(&self, entry: &EntryObject) -> Result<String, DomainError> {
    // check if ins exists in metadata
    if entry.metadata.ins.is_none() {
      return Err(DomainError::new(DomainErrorCode::MissingIns, "Missing ins".to_string()));
    };
    let ins = entry.metadata.ins.as_ref().unwrap();
    self.validate_tag(entry.metadata.tag.as_str())?;
    self.validate_date(entry.metadata.date.as_str())?;
    self.validate_time(entry.metadata.time.as_str())?;
    self.validate_ins(ins.as_str())?;
    self.validate_message(entry.message.as_str())?;
    return Ok("Entry validated: ".to_string() + ins.as_str());
  }
}

/* -----------------------------------TESTS------------------------------------------ */
/*
# Domain
## EntryObject validation
### metadata
- four properties allowed in metadata:
  - ins: (13 characters) epoch format, date of insertion
  - date: (9 characters) dd-mon-yy format, date of occurence
  - time: (9 characters max) fixed values:
    - latenight: 00-05
    - morning: 06-11
    - afternoon: 12-17
    - night: 18-23
    - no: not applicable
  - tag: (12 characters max) alphanumerical only, entry subject, only one per entry
### message
- 32 characters max, characters allowed: alphanumerical, + - = . , : _ \ / ( ) < > $"
## EntryObject sorting
- date -> time -> ins -> tag -> message
*/
#[cfg(test)]
mod tests {
  use super::*;
  // use chrono::prelude::*;
  use std::convert::TryInto;

  /* VALIDATION */
  #[test]
  fn validate_tag_test() {
    let entry_validator = EntryBusiness {};
    let invalid_characters = entry_validator.validate_tag("aa..");
    assert_eq!(invalid_characters.is_err(), true);
    let too_long = entry_validator.validate_tag("aaaaaaaaaaaaa");
    assert_eq!(too_long.is_err(), true);
    let uppercase = entry_validator.validate_tag("aaAa");
    assert_eq!(uppercase.is_err(), true);
    let ok = entry_validator.validate_tag("aaa");
    assert_eq!(ok.is_ok(), true);
  }
  #[test]
  fn validate_date_test() {
    let entry_validator = EntryBusiness {};
    let wrong_format = entry_validator.validate_date("....");
    assert_eq!(wrong_format.is_err(), true);
    let ok = entry_validator.validate_date("10-dec-20");
    assert_eq!(ok.is_ok(), true);
  }
  #[test]
  fn validate_time_test() {
    let entry_validator = EntryBusiness {};
    let wrong_format = entry_validator.validate_date("....");
    assert_eq!(wrong_format.is_err(), true);
    let morning = entry_validator.validate_time("morning");
    assert_eq!(morning.is_ok(), true);
    let afternoon = entry_validator.validate_time("afternoon");
    assert_eq!(afternoon.is_ok(), true);
    let night = entry_validator.validate_time("night");
    assert_eq!(night.is_ok(), true);
    let latenight = entry_validator.validate_time("latenight");
    assert_eq!(latenight.is_ok(), true);
    let notapplicable = entry_validator.validate_time("n/a");
    assert_eq!(notapplicable.is_ok(), true);
  }
  #[test]
  fn validate_ins_test() {
    let entry_validator = EntryBusiness {};
    let wrong_length = entry_validator.validate_ins("111111111111");
    let wrong_format = entry_validator.validate_ins("111111111111a");
    assert_eq!(wrong_length.is_err(), true);
    assert_eq!(wrong_format.is_err(), true);
    let allowed = entry_validator.validate_ins("1111111111111");
    assert_eq!(allowed.is_ok(), true);
  }
  #[test]
  fn validate_message_test() {
    let entry_validator = EntryBusiness {};
    let too_long = entry_validator.validate_message("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(too_long.is_err(), true);
    let wrong_format = entry_validator.validate_message("[]ç");
    assert_eq!(wrong_format.is_err(), true);
    let allowed = entry_validator.validate_message("A-z0-9 .,:$()<>-=\\/_");
    assert_eq!(allowed.is_ok(), true);
    let allowed2 = entry_validator.validate_message("d");
    assert_eq!(allowed2.is_ok(), true);
  }
  /* SORTING */
  #[test]
  fn sort_entry_test() {
    // sort by date -> time -> ins

    /* DATE */
    let date1 = Local
      .ymd(2020, 12, 2.try_into().unwrap())
      .format("%d-%b-%y")
      .to_string()
      .to_lowercase();
    let date2 = Local
      .ymd(2020, 12, 3.try_into().unwrap())
      .format("%d-%b-%y")
      .to_string()
      .to_lowercase();
    let date3 = Local
      .ymd(2020, 12, 5.try_into().unwrap())
      .format("%d-%b-%y")
      .to_string()
      .to_lowercase();
    let date_vec_ordered = vec![date1.clone(), date2.clone(), date3.clone()];
    let date_vec_unordered = vec![date2.clone(), date1.clone(), date3.clone()];
    let mut unordered_date_entries: Vec<EntryObject> = vec![];
    let mut ordered_date_entries: Vec<EntryObject> = vec![];
    fn gen_date_entry(date: String) -> EntryObject {
      let ins = "111111111111".to_string();
      let time = "night".to_string();
      let tag = "datetest".to_string();
      let metadata = EntryMetadata {
        ins: Some(ins),
        date,
        time,
        tag,
      };
      let obj = EntryObject {
        metadata,
        message: "hello".to_string(),
      };
      return obj;
    }
    for date in date_vec_unordered {
      let obj = gen_date_entry(date);
      unordered_date_entries.push(obj);
    }
    for date in date_vec_ordered {
      let obj = gen_date_entry(date);
      ordered_date_entries.push(obj);
    }
    unordered_date_entries.sort();
    assert_eq!(unordered_date_entries, ordered_date_entries);

    /* TIME */

    let time1 = "latenight".to_string();
    let time2 = "morning".to_string();
    let time3 = "night".to_string();
    let time4 = "n/a".to_string();
    let time_vec_ordered = vec![time1.clone(), time2.clone(), time3.clone(), time4.clone()];
    let time_vec_unordered = vec![time2.clone(), time1.clone(), time3.clone(), time4.clone()];
    let mut unordered_time_entries: Vec<EntryObject> = vec![];
    let mut ordered_time_entries: Vec<EntryObject> = vec![];
    fn gen_time_entry(time: String) -> EntryObject {
      let ins = "111111111111".to_string();
      let tag = "datetest".to_string();
      let date = "20-dec-2".to_string();
      let metadata = EntryMetadata {
        ins: Some(ins),
        tag,
        time,
        date,
      };
      let obj = EntryObject {
        metadata,
        message: "hello".to_string(),
      };
      return obj;
    }
    for time in time_vec_unordered {
      let obj = gen_time_entry(time);
      unordered_time_entries.push(obj);
    }
    for time in time_vec_ordered {
      let obj = gen_time_entry(time);
      ordered_time_entries.push(obj);
    }
    unordered_time_entries.sort();
    assert_eq!(unordered_time_entries, ordered_time_entries);

    /* INS */

    let ins1 = "1111111111112".to_string();
    let ins2 = "1111111111113".to_string();
    let ins3 = "1111111111114".to_string();
    let ins_vec_ordered = vec![ins1.clone(), ins2.clone(), ins3.clone()];
    let ins_vec_unordered = vec![ins2.clone(), ins3.clone(), ins1.clone()];
    let mut unordered_ins_entries: Vec<EntryObject> = vec![];
    let mut ordered_ins_entries: Vec<EntryObject> = vec![];
    fn gen_ins_entry(ins: String) -> EntryObject {
      let tag = "datetest".to_string();
      let date = "20-dec-2".to_string();
      let time = "morning".to_string();
      let metadata = EntryMetadata {
        ins: Some(ins),
        tag,
        time,
        date,
      };
      let obj = EntryObject {
        metadata,
        message: "hello".to_string(),
      };
      return obj;
    }
    for ins in ins_vec_ordered {
      let obj = gen_ins_entry(ins);
      ordered_ins_entries.push(obj);
    }
    for ins in ins_vec_unordered {
      let obj = gen_ins_entry(ins);
      unordered_ins_entries.push(obj);
    }
    unordered_ins_entries.sort();
    assert_eq!(unordered_ins_entries, ordered_ins_entries);

    /* MIX */
    // different date
    // same date, different time
    // same date, same time, different ins
    // same date, same time, same ins
    // same date, same time, null ins
    let mix_metadata1 = EntryMetadata {
      ins: Some(ins1.clone()),
      date: date1.clone(),
      time: time1.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object1 = EntryObject {
      metadata: mix_metadata1,
      message: "check date".to_string(),
    };
    let mix_metadata2 = EntryMetadata {
      ins: Some(ins1.clone()),
      date: date2.clone(),
      time: time1.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object2 = EntryObject {
      metadata: mix_metadata2,
      message: "check date and time".to_string(),
    };
    let mix_metadata3 = EntryMetadata {
      ins: Some(ins1.clone()),
      date: date2.clone(),
      time: time2.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object3 = EntryObject {
      metadata: mix_metadata3,
      message: "check time".to_string(),
    };
    let mix_metadata4 = EntryMetadata {
      ins: Some(ins1.clone()),
      date: date2.clone(),
      time: time3.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object4 = EntryObject {
      metadata: mix_metadata4,
      message: "check time and ins".to_string(),
    };
    let mix_metadata5 = EntryMetadata {
      ins: Some(ins2.clone()),
      date: date2.clone(),
      time: time3.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object5 = EntryObject {
      metadata: mix_metadata5,
      message: "check ins".to_string(),
    };
    let mix_metadata6 = EntryMetadata {
      ins: Some(ins3.clone()),
      date: date2.clone(),
      time: time3.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object6 = EntryObject {
      metadata: mix_metadata6,
      message: "check null ins".to_string(),
    };
    let mix_metadata7 = EntryMetadata {
      ins: None,
      date: date2.clone(),
      time: time3.clone(),
      tag: "mixtest".to_string(),
    };
    let mix_object7 = EntryObject {
      metadata: mix_metadata7,
      message: "check null ins".to_string(),
    };
    let mix_metadata8 = EntryMetadata {
      ins: Some(ins3.clone()),
      date: date2.clone(),
      time: time4.clone(),
      tag: "a".to_string(),
    };
    let mix_object8 = EntryObject {
      metadata: mix_metadata8,
      message: "check tag".to_string(),
    };
    let mix_metadata9 = EntryMetadata {
      ins: Some(ins3.clone()),
      date: date2.clone(),
      time: time4.clone(),
      tag: "b".to_string(),
    };
    let mix_object9 = EntryObject {
      metadata: mix_metadata9,
      message: "check tag".to_string(),
    };
    let mix_metadata10 = EntryMetadata {
      ins: Some(ins3.clone()),
      date: date2.clone(),
      time: time4.clone(),
      tag: "b".to_string(),
    };
    let mix_object10 = EntryObject {
      metadata: mix_metadata10,
      message: "check tag and message".to_string(),
    };
    let obj_vec_ordered = vec![
      mix_object1.clone(),
      mix_object2.clone(),
      mix_object3.clone(),
      mix_object4.clone(),
      mix_object5.clone(),
      mix_object6.clone(),
      mix_object7.clone(),
      mix_object8.clone(),
      mix_object9.clone(),
      mix_object10.clone(),
    ];
    let mut obj_vec_unordered = vec![
      mix_object10.clone(),
      mix_object8.clone(),
      mix_object9.clone(),
      mix_object3.clone(),
      mix_object2.clone(),
      mix_object4.clone(),
      mix_object1.clone(),
      mix_object7.clone(),
      mix_object6.clone(),
      mix_object5.clone(),
    ];
    obj_vec_unordered.sort();
    /*
    println!("unordered sort -------------------------");
    for obj in obj_vec_unordered {
      println!("{:?}", obj);
    }
    println!("ordered -------------------------");
    for obj in obj_vec_ordered {
      println!("{:?}", obj);
    }
    */
    assert_eq!(obj_vec_ordered, obj_vec_unordered);
  }
}
