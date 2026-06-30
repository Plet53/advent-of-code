use std::fs;
use std::env;
use regex::Regex;
use shared::grid::Grid;

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
  
  let shape_marker = Regex::new(r"\d+:").unwrap();
  let set_marker = Regex::new(r"\d+x+d+:").unwrap();
  
  let mut data_lines = rawdata.lines().peekable();
  
  let mut gifts: Vec<Grid<char>> = Vec::new();
  let mut valid_counter: u16 = 0;

  loop {
    match data_lines.peek() {
      Some(string) if shape_marker.is_match(string) => {
        data_lines.next();
        
        gifts.push(
          (0..3).map(|_index| data_lines.next().unwrap().chars().collect()).collect()
        );
        
        data_lines.next();
      },
      Some(string) if set_marker.is_match(string) => {
        for line in data_lines {
          let data: Vec<&str> = line.split(':').collect();
          let width: u16 = data[0].split('x').map(|num| num.parse().unwrap()).nth(0);
          let height: u16 = data[0].split('x').map(|num| num.parse().unwrap()).nth(1);
          let present_count: Vec<u16> = data[1].split_whitespace().map(|num| num.parse().unwrap()).collect();
          
          
        }
        break;
      },
      Some(_) => continue,
      None => break,
    }
  }
  
  println!("{valid_counter}");
}
