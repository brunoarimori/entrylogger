extern crate entrylogger;
extern crate chrono;
use entrylogger::domain;
use entrylogger::application;
use application::EntryControllerInterface;

struct EntryControllerTest {

}

impl EntryControllerTest {
  fn get_metadata_example(&self, iterator: i16) -> domain::EntryMetadata {
    let tag_start = String::from("it");
    let result = domain::EntryMetadata {
      ins: String::from("0000000000000"),
      date: String::from("20-aug-2020"),
      time: String::from("morning"),
      tag: [tag_start, iterator.to_string()].concat(),
    };
    return result;
  }
  fn get_entry_example(&self, iterator: i16) -> domain::EntryObject {
    let metadata = self.get_metadata_example(iterator);
    let result = domain::EntryObject {
      metadata: metadata,
      message: String::from("hello"),
    };
    return result;
  }
  fn get_multiple_entries(&self, qty: i16) -> Vec<domain::EntryObject> {
    let mut vec: Vec<domain::EntryObject> = Vec::new();
    for i in 0..qty {
      let obj = self.get_entry_example(i);
      vec.push(obj);
    }
    return vec;
  }
}

impl application::EntryControllerInterface for EntryControllerTest {
  fn get_entries(&self) -> Vec<domain::EntryObject> {
    let ret = self.get_multiple_entries(10);
    return ret;
  }
  fn get_digest(&self) -> domain::EntryDigest {
    let entry_test = self.get_entry_example(10);
    let mut tags: Vec<String> = Vec::new();
    tags.push(String::from("test"));
    let digest = domain::EntryDigest {
      qty: 10,
      tags: tags,
      entries_today: 10,
      last_entry_date: String::from("20-aug-2020"),
      last_entry: entry_test
    };
    return digest;
  }
  fn post_entry(&self, entry: domain::EntryObject) -> bool {
    return true;
  }
}

#[test]
fn test() {
  let a = EntryControllerTest{};
  println!("{:?}", a.get_entries());
}