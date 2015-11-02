#[cfg(test)]
#[allow(dead_code)]

    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn read_json_file(filename:&str) -> String {
        // Create a path to the desired file
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!(
                "couldn't open {}: {}",
                display,
                Error::description(&why)
                ),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_)    => {}, // print!("{} contains:\n{}", display, s),
        };

        s
    }

