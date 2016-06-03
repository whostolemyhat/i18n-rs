extern crate glob;
extern crate rustc_serialize;

use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::Read; // need this for .read_to_string to exist
use std::path::Path;
use rustc_serialize::json;

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
    Ok(_) => println!("{}", json::decode(&contents).unwrap())
  };
}

fn main() {
  // open src files
  // read in eng translation
  // convert to struct
  // http://zsiciarz.github.io/24daysofrust/book/day6.html

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
