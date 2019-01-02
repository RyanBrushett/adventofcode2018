use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("file not found");
    let mut start: i32 = 0;

    for line in BufReader::new(&f).lines() {
        let l = line.unwrap();
        let my_int: i32 = l.parse()
            .expect("error parsing line as integer");
        start += my_int;
    }

    println!("{}", start)
}
