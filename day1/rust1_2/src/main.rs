use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
  let mut f = File::open("input").expect("Failed to open file");
  let mut input = String::new();
  f.read_to_string(&mut input).expect("Failed to read file");

  let mut value = 0;
  let mut values = HashSet::new();
  values.insert(value);

  loop {
    for line in input.lines() {
      let my_int: i32 = line.parse().expect("Could not parse line");
      value += my_int;

      if values.contains(&value) {
        println!("The matching value is {}", value);
        return;
      }
      values.insert(value);
    }
  }
}
