use std::fs;
use std::env;
use regex::Regex;

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
  let filter_pattern = Regex::new(r"(?s)(do\(\)|\A)(?<valid>.+?)(don't\(\)|\z){1}").unwrap();
  let mult_pattern = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();

  let outer_acc: u64 = filter_pattern.captures_iter(&rawdata).map(|caps| {
    let valid_data = caps.name("valid");
    let inner_acc: Option<u64> = match valid_data {
      Some(data) => {
        println!("{}", data.as_str());
        mult_pattern.captures_iter(&data.as_str()).map(|prod| {
          let num1: u64 = prod.name("num1").unwrap().as_str().parse().expect("bad number");
          let num2: u64 = prod.name("num2").unwrap().as_str().parse().expect("bad number");

          num1 * num2
        }).reduce(|acc, elem| { acc + elem })
      },
      None => None
    };
    println!();
    inner_acc.unwrap_or(0)
  }).reduce(|acc, elem| { acc + elem }).unwrap();
  
  println!("{}", outer_acc);
}
