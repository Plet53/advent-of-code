use std::fs;
use std::env;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String;
  match args.next() {
    Some(arg) => filename = arg,
    None => {
      panic!("remember the filename")
    },
  };
  
  let rawdata = fs::read_to_string(filename).unwrap();

  let mut list1: Vec<u64> = Vec::new();
  let mut list2: Vec<u64> = Vec::new();

  for row in rawdata.lines() {
    let values: Vec<u64> = row.split_whitespace().map(|data| {
      data.parse().unwrap()
    }).collect();

    let val1 = values.get(0).unwrap();
    let val2 = values.get(1).unwrap();

    list1.push(*val1);
    list2.push(*val2);
  }

  list1.sort();
  list2.sort();

  let mut contents = String::new();
  for index in 0..list1.len() {
    contents.push_str(&list1.get(index).unwrap().to_string()[..]);
    contents.push(' ');
    contents.push_str(&list2.get(index).unwrap().to_string()[..]);
    contents.push('\n');
  }
  fs::write("day1sorted.txt", contents).unwrap();

  let mut acc: u64 = 0;

  for index in 0..list1.len() {
    acc = acc + list1.get(index).unwrap().abs_diff(*list2.get(index).unwrap());
  }

  println!("{}", acc);
}
