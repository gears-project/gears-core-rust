extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use self::serde::de::DeserializeOwned;

#[allow(dead_code)]
pub fn load_doc<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let json_string = read_json_file(path);
    serde_json::from_str::<T>(&json_string).unwrap()
}

#[allow(dead_code)]
pub fn read_json_file(filename: &str) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, Error::description(&why));
    };

    s
}
