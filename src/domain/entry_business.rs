use super::error::*;
use super::entry_object::*;

use regex::Regex;

const INS_LENGTH: usize = 13;
const MAX_TAG_LENGTH: usize = 12;
const MAX_MSG_LENGTH: usize = 32;
lazy_static! {
  static ref ALPHA_REGEX: Regex = Regex::new(r"^[a-z0-9]+$").unwrap();
  static ref MSG_REGEX: Regex = Regex::new(r"^[A-z \+-=.,:_\\/\(\)<> \$]+$").unwrap();
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
      return Err(DomainError::new(
        DomainErrorCode::InvalidFormat,
        "Only lowercase alphanumerical characters allowed in tag".to_string(),
      ));
    };
    if tag.len() > MAX_TAG_LENGTH {
      // return Err("Maximum length allowed for tag: ".to_string() + &MAX_TAG_LENGTH.to_string());
      // return Err(DomainError::MaxLengthExceeded);
      return Err(DomainError::new(
        DomainErrorCode::MaxLengthExceeded,
        "Maximum length for tag is ".to_string() + &MAX_TAG_LENGTH.to_string(),
      ));
    };
    return Ok(tag.to_string());
  }
  fn validate_date(&self, date: &str) -> Result<String, DomainError> {
    match chrono::NaiveDate::parse_from_str(date, "%d-%b-%y") {
      Ok(value) => return Ok(value.format("%d-%b-%y").to_string().to_lowercase()),
      Err(_err) => {
        // return Err("Expected one of the following: today, yesterday or <dd-mon-yy> format".to_string());
        // return Err(DomainError::InvalidFormat)
        return Err(DomainError::new(
          DomainErrorCode::InvalidFormat,
          "Expected <dd-mon-yy>".to_string(),
        ));
      }
    };
  }
  fn validate_time(&self, time: &str) -> Result<String, DomainError> {
    if TIME_VEC.contains(&time) {
      return Ok(time.to_string());
    } else {
      // return Err("Expected one of the following: morning, afternoon, night, latenight, n/a or now".to_string());
      return Err(DomainError::new(
        DomainErrorCode::InvalidFormat,
        "Expected one of the following: morning, afternoon, night, latenight, n/a".to_string(),
      ));
    }
  }
  fn validate_ins(&self, ins: &str) -> Result<String, DomainError> {
    if ins.len() != INS_LENGTH {
      return Err(DomainError::new(
        DomainErrorCode::MaxLengthExceeded,
        "Maximum length for ins is ".to_string() + &INS_LENGTH.to_string(),
      ));
    }
    let check = ins.parse::<u64>();
    match check {
      Ok(_ok) => return Ok(ins.to_string()),
      Err(_err) => {
        return Err(DomainError::new(
          DomainErrorCode::InvalidFormat,
          "Ins must be a number".to_string(),
        ))
      }
    };
  }
  fn validate_message(&self, message: &str) -> Result<String, DomainError> {
    if !MSG_REGEX.is_match(message) {
      return Err(DomainError::new(
        DomainErrorCode::InvalidFormat,
        "Invalid characters found in message".to_string(),
      ));
    };
    if message.len() > MAX_MSG_LENGTH {
      return Err(DomainError::new(
        DomainErrorCode::MaxLengthExceeded,
        "Maximum length for message is ".to_string() + &MAX_MSG_LENGTH.to_string(),
      ));
    };
    return Ok(message.to_string());
  }
}
impl EntryHandler for EntryBusiness {
  fn validate(&self, entry: &EntryObject) -> Result<String, DomainError> {
    // check if ins exists in metadata
    if entry.metadata.ins.is_none() {
      return Err(DomainError::new(
        DomainErrorCode::MissingIns,
        "Missing ins".to_string(),
      ));
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

