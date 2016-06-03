extern crate glob;

use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::Read; // need this for .read_to_string to exist
use std::path::Path;

const MESSAGE_PATTERN: &'static str = "_translations/**/*.json";
const LANG_DIR: &'static str = "_translations/lang/";

fn read_file(filename: &Path) {
  let mut contents = String::new();
  let mut file = match File::open(filename) {
    Err(e) => panic!("Couldn't open {}: {}", filename.display(), e.description()),
    Ok(file) => file
  };

  match file.read_to_string(&mut contents) {
    Err(e) => panic!("Couldn't read {}: {}", filename.display(), e.description()),
    Ok(_) => println!("{}", contents)
  };
  // println!("{}", contents.len());
}

fn main() {
    println!("Hello, world!");

    // let default_messages = glob(MESSAGE_PATTERN);
    // println!(default_messages);

    for entry in glob(MESSAGE_PATTERN).unwrap() {
      match entry {
        Ok(path) => { // path = PathBuf
          println!("{:?}", path);
          read_file(path.as_path());
        },
        Err(e) => println!("{:?}", e)
      }
    }
}
