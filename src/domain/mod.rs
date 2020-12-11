#[derive(std::fmt::Debug)]
pub struct EntryMetadata {
  pub ins: String,
  pub date: String,
  pub time: String,
  pub tag: String,
}
#[derive(std::fmt::Debug)]
pub struct EntryObject {
  pub metadata: EntryMetadata,
  pub message: String,
}

#[derive(std::fmt::Debug)]
pub struct EntryDigest {
  pub qty: i16,
  pub tags: Vec<String>,
  pub entries_today: i16,
  pub last_entry_date: String,
  pub last_entry: EntryObject,
}
