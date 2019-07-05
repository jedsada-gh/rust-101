use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let _file = File::open("../files/hello-world.txt");
    let _file = match _file {
        Ok(file) => file,
        Err(err) => {
            panic!("Problem opening the file: {:?}", err);
        },
    };

    let f = BufReader::new(_file);
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("Line: {}", line);
    }
}