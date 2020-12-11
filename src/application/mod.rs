use domain;
pub trait EntryControllerInterface {
  fn get_entries(&self) -> Vec<domain::EntryObject>;
  fn post_entry(&self, entry: domain::EntryObject) -> bool;
  fn get_digest(&self) -> domain::EntryDigest;
}