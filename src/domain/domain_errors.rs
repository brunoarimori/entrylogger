use std::{clone::Clone};

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
