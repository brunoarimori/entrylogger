use super::entry_controller_interface::EntryControllerInterface;
use super::entry_persistence_interface::EntryPersistenceInterface;
use domain;
use std::rc::Rc;

pub struct EntryController {
  entry_business: Rc<dyn domain::EntryHandler>,
  entry_persistence: Rc<dyn EntryPersistenceInterface>,
}

impl EntryControllerInterface for EntryController {
  fn new(
    entry_business: Rc<dyn domain::EntryHandler>,
    entry_persistence: Rc<dyn EntryPersistenceInterface>,
  ) -> Self {
    return EntryController {
      entry_business,
      entry_persistence,
    };
  }
  fn get_entries(&self) -> Result<Vec<domain::EntryObject>, String> {
    let mut entries = self.entry_persistence.read_entries().unwrap();
    entries.sort();
    return Ok(entries);
  }
  fn post_entry(&self, mut entry: domain::EntryObject) -> Result<domain::EntryObject, String> {
    let ins = chrono::Local::now().timestamp_millis().to_string();
    entry.metadata.ins = Some(ins);
    let _validate = self.entry_business.validate(&entry).or_else(|err| {
      return Err(err.message());
    });
    return self.entry_persistence.write_entry(entry);
  }
}
