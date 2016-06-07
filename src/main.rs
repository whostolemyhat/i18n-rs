extern crate glob;
extern crate rustc_serialize;

use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::Read; // need this for .read_to_string to exist
use std::path::Path;
use rustc_serialize::json;
use rustc_serialize::json::Json;

const MESSAGE_PATTERN: &'static str = "_translations/src/**/*.json";
// const LANG_DIR: &'static str = "_translations/lang/";

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Message {
  id: String,
  description: String,
  defaultMessage: String
}

// #[derive(Debug, RustcDecodable, RustcEncodable)]
// struct Messages {
//   messages: Vec<Message>
// }

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
    // println!("{:?}", message);
    messages.push(message);
  }
}

fn main() {
  // open src files
  // read in eng translation
  // convert to struct
  // http://zsiciarz.github.io/24daysofrust/book/day6.html
  // let raw = "{
  //   \"id\": \"alarms.heading\",
  //   \"description\": \"Alarm heading text\",
  //   \"defaultMessage\": \"Alarms\"
  // }";

  // let arr = "[{
  //   \"id\": \"alarms.heading\",
  //   \"description\": \"Alarm heading text\",
  //   \"defaultMessage\": \"Alarms\"
  // },{
  //   \"id\": \"alarms.heading2\",
  //   \"description\": \"Alarm heading text\",
  //   \"defaultMessage\": \"Alarms 2\"
  // }]";

  // let message: Message = json::decode(raw).unwrap();
  // println!("{:?}", message);

  // let msgs = Json::from_str(arr).unwrap();
  // println!("{:?}", msgs);

  // let data = msgs.as_array().unwrap();
  // println!("{:?}", data);

  // for msg in data {
  //   let message = msg.as_object().unwrap();
  //   println!("{:?}", message);

  //   let decoded: Message = json::decode(&msg.to_string()).unwrap();
  //   println!("{:?}", decoded);
  // }

  // for(key, value) in data.iter() {
  //   println!("{:?}: {}", key, match *value {
  //     Json::String(ref v) => format!("{} (string)", v),
  //     _ => format!("other")
  //   });
  // }

  let mut messages = vec![];

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
}
