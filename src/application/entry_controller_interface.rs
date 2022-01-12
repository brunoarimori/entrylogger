use super::entry_persistence_interface::EntryPersistenceInterface;
use domain;
use std::rc::Rc;

pub trait EntryControllerInterface {
  fn new(
    entry_business: Rc<dyn domain::EntryHandler>,
    entry_persistence: Rc<dyn EntryPersistenceInterface>,
  ) -> Self;
  fn get_entries(&self) -> Result<Vec<domain::EntryObject>, String>;
  fn post_entry(&self, entry: domain::EntryObject) -> Result<domain::EntryObject, String>;
}
