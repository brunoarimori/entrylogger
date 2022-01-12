use chrono::prelude::*;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::clone::Clone;

pub static TIME_VEC: &'static [&str] = &["latenight", "morning", "afternoon", "night", "n/a"];

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
