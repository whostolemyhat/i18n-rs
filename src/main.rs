extern crate glob;
extern crate rustc_serialize;
extern crate regex;

use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::Read; // need this for .read_to_string to exist
use std::path::Path;
use std::collections::HashMap;
use std::collections::BTreeMap;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use regex::Regex;

const MESSAGE_PATTERN: &'static str = "_translations/src/**/*.json";
const TRANSLATION_PATTERN: &'static str = "_translations/lang/**/*.json";
const EN_STRING: &'static str = "en";

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Message {
  id: String,
  description: String,
  defaultMessage: String
}

fn read_file(filename: &Path, messages: &mut Vec<Message>) {
  let mut contents = String::new();
  let mut file = match File::open(filename) {
    Err(e) => panic!("Couldn't open {}: {}", filename.display(), e.description()),
    Ok(file) => file
  };

  let msgs = match file.read_to_string(&mut contents) {
    Err(e) => panic!("Couldn't read {}: {}", filename.display(), e.description()),
    Ok(_) => Json::from_str(&contents).unwrap()
  };

  let data = msgs.as_array().unwrap();
  for msg in data {
    let message: Message = json::decode(&msg.to_string()).unwrap();
    messages.push(message);
  }

}

fn read_translation_file(filename: &Path, translations: &mut BTreeMap<String, BTreeMap<String, String>>) {
  // get name of file to use as key
  let lang_file = filename.file_name().unwrap().to_str().unwrap();
  let lang_name: Vec<&str> = lang_file.split('.').collect();
  let lang = lang_name[0];

  let mut contents = String::new();
  let mut file = match File::open(filename) {
    Err(e) => panic!("Couldn't open {}: {}", filename.display(), e.description()),
    Ok(file) => file
  };

  let msgs = match file.read_to_string(&mut contents) {
    Err(e) => panic!("Couldn't read {}: {}", filename.display(), e.description()),
    Ok(_) => Json::from_str(&contents).unwrap()
  };

  let data = msgs.as_object().unwrap();

  // to_string everywhere: https://users.rust-lang.org/t/variable-does-not-live-long-enough/936/2
  // converts to String which are on the heap so live long enough
  for translation in data {
    let translation_map: BTreeMap<String, String> = BTreeMap::new();

    // update existing key or add an empty hashmap under that key
    // http://stackoverflow.com/questions/28512394/how-to-lookup-from-and-insert-into-a-hashmap-efficiently
    // http://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
    let existing = translations.entry(translation.0.to_string()).or_insert(translation_map);
    existing.insert(lang.to_string(), translation.1.as_string().unwrap().to_string());
  }
}

fn main() {
  // http://zsiciarz.github.io/24daysofrust/book/day6.html

  // get eng messages
  let mut messages: Vec<Message> = vec![];
  for entry in glob(MESSAGE_PATTERN).unwrap() {
    match entry {
      Ok(path) => { // path = PathBuf
        read_file(path.as_path(), &mut messages);
      },
      Err(e) => println!("{:?}", e)
    }
  }

  let mut translations = BTreeMap::new();

  // add en to map
  for message in messages {
    let mut translation: BTreeMap<String, String> = BTreeMap::new();
    translation.insert(EN_STRING.to_string(), message.defaultMessage.to_string());
    translations.insert(message.id.to_string(), translation);
  }

  // convert translation files and add to map
  for entry in glob(TRANSLATION_PATTERN).unwrap() {
    match entry {
      Ok(path) => {
        read_translation_file(path.as_path(), &mut translations);
      },
      Err(e) => println!("{:?}", e)
    }
  }

  println!("{:?}", translations);
  // let encoded = json::encode(&translations).unwrap();
  let encoded = format!("{:?}", translations);

  let re = Regex::new(r"(\\u\{)([\w]+)(\})+").unwrap();
  let result = re.replace_all(&encoded, "\\u00$2");
  println!("{:?}", result);
  // replace \u{e0} with \u00e0
  // /(\\u\{[\w]+\})+/g
}
