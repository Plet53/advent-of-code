use std::fs;
use std::env;
use std::collections::HashMap;
use std::ops::Index;

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

  let mut list2map: HashMap<u64, u64> = HashMap::new(); 

  for val in list2 {
    if !list2map.contains_key(&val) {
      list2map.insert(val, 1);
    } else {
      list2map.insert(val, list2map.index(&val) + 1);
    }
  }

  let mut acc: u64 = 0;

  for val in list1 {
    if list2map.contains_key(&val) {
      acc = acc + val * list2map.index(&val);
    }
  }

  println!("{}", acc);
}
