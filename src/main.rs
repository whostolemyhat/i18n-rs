extern crate glob;
extern crate rustc_serialize;

// TODO use box?

use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::Read; // need this for .read_to_string to exist
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;
use rustc_serialize::json;
use rustc_serialize::json::Json;

const MESSAGE_PATTERN: &'static str = "_translations/src/**/*.json";
const TRANSLATION_PATTERN: &'static str = "_translations/lang/**/*.json";
const en_string: &'static str = "en";
// const LANG_DIR: &'static str = "_translations/lang/";

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Message {
  id: String,
  description: String,
  defaultMessage: String
}

struct TranslationFile {
  keys: BTreeMap<String, Json>
}

struct Translation<'a> {
  key: &'a String,
  value: String,
  lang: String
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

fn read_translation_file(filename: &Path, messages: &Vec<Message>) {
  // create hashmap
  let mut translations = HashMap::new();
  // for each message
  for message in messages {
    // println!("{:?}, {:?}", message.id, message.defaultMessage);
    let mut translation: HashMap<&str, &String> = HashMap::new();
    translation.insert(en_string, &message.defaultMessage.to_string());
    translations.insert(&message.id, translation);
  }

  println!("{:?}", translations);
  // use id as key

  // get name of file to use as key
  let lang_file = filename.file_name().unwrap().to_str().unwrap();
  let lang_name: Vec<&str> = lang_file.split('.').collect();
  let lang = lang_name[0];

  println!("{:?}", lang);

  let mut contents = String::new();
  let mut file = match File::open(filename) {
    Err(e) => panic!("Couldn't open {}: {}", filename.display(), e.description()),
    Ok(file) => file
  };

  let msgs = match file.read_to_string(&mut contents) {
    Err(e) => panic!("Couldn't read {}: {}", filename.display(), e.description()),
    Ok(_) => Json::from_str(&contents).unwrap()
  };

  println!("msgs: {:?}", msgs);

  let data = msgs.as_object().unwrap();

  for (key, value) in data.iter() {
    // println!("{}: {}", key, value);
    // println!("{}: {}", key, match *value {
    //   Json::String(ref v) => {
    //     format!("{} (string)", v)
    //   },
    //   Json::Object(ref o) => {
    //     // translations.push( json::decode(&o.to_string()).unwrap() );
    //     format!("{:?} object", o) // o is the object (value) where k = 'es'
    //   },
    //   _ => format!("other")
    // });

    let translation = Translation{
      key: key,
      value: value.as_string().unwrap().to_string(),
      lang: lang.to_string()
    };

    let mut translation_map: HashMap<&str, &String> = HashMap::new();
    translation_map.insert(&translation.key, &translation.value);

    // println!("key: {:?}", key);
    // println!("map: {:?}", translations.get(key));
    // println!("{:?}", translation);
    // TODO make sure key exists
    match translations.get(key) {
      Some(map) => map,
      None => {
        translations.insert(&translation.key.to_string(), translation_map);
        return;
      }
    };
    // en_key.insert(lang, &value.to_string());
    // translations.insert(key, translation);
  }
}

fn main() {
  // http://zsiciarz.github.io/24daysofrust/book/day6.html

  // get eng messages
  let mut messages: Vec<Message> = vec![];
  for entry in glob(MESSAGE_PATTERN).unwrap() {
    match entry {
      Ok(path) => { // path = PathBuf
        // println!("{:?}", path);
        read_file(path.as_path(), &mut messages);
      },
      Err(e) => println!("{:?}", e)
    }
  }
  println!("{:?}", messages);

  // get translations
  // let translations: Vec<BTreeMap<String, Json>> = vec![];
  for entry in glob(TRANSLATION_PATTERN).unwrap() {
    match entry {
      Ok(path) => {
        read_translation_file(path.as_path(), &messages);
      },
      Err(e) => println!("{:?}", e)
    }
  }
  // println!("{:?}", translations);

  // merge by keys
}
