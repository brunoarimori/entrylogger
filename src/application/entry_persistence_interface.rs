use domain;

pub trait EntryPersistenceInterface {
  fn read_entries(&self) -> Result<Vec<domain::EntryObject>, String>;
  fn write_entry(&self, entry: domain::EntryObject) -> Result<domain::EntryObject, String>;
  // fn upsert_entry_file_meta(&self, file_meta: String) -> Result<String, String>;
}
