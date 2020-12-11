use application;
use chrono::{prelude::*, Duration};
use domain;
use std::io;
use std::io::prelude::*;

struct InputValidation {}
impl InputValidation {
  fn validate_date(&self, date: String) -> Result<String, String> {
    match date.as_str() {
      "today" => {
        let today = Local::today().format("%d-%b-%y").to_string().to_lowercase();
        return Ok(today);
      }
      "yesterday" => {
        let yesterday = (Local::today() - Duration::days(1))
          .format("%d-%b-%y")
          .to_string()
          .to_lowercase();
        return Ok(yesterday);
      }
      _ => {
        match NaiveDate::parse_from_str(date.as_str(), "%d-%b-%y") {
          Ok(value) => return Ok(value.format("%d-%b-%y").to_string().to_lowercase()),
          Err(_err) => return Err(String::from("Expected one of the following: today, yesterday or <dd-mon-yy> format")),
        };
      }
    }
  }
  fn validate_time(&self, time: String) -> Result<String, String> {
    match time.as_str() {
      "morning" | "afternoon" | "night" | "latenight" | "n/a" => {
        return Ok(time);
      }
      "now" => return Err(String::from("TBI")),
      _ => {
        return Err(String::from(
          "Expected one of the following: morning, afternoon, night, latenight, n/a or now",
        ))
      }
    }
  }
  // fn validate_tag(&self, tag: String) -> Result<String, String> {}
}

pub struct CliInput {
  input_validation: InputValidation,
}
impl CliInput {
  fn parse_new_line(str: String) -> String {
    return str
      .trim_end_matches("\n")
      .trim_end_matches("\r")
      .to_string();
  }
  pub fn new() -> CliInput {
    let input_validation = InputValidation {};
    return CliInput { input_validation };
  }
  pub fn read_date(&self) -> Result<String, String> {
    print!("date > ");
    io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        return self
          .input_validation
          .validate_date(Self::parse_new_line(buf))
      }
      Err(_error) => return Err(String::from("couldn't read date")),
    }
  }
  pub fn read_time(&self) -> Result<String, String> {
    print!("time > ");
    io::stdout().flush().expect("flush failed");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
      Ok(_n) => {
        return self
          .input_validation
          .validate_time(Self::parse_new_line(buf))
      }
      Err(_error) => return Err(String::from("couldn't read time")),
    }
  }
}
