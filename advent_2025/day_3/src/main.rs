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
  
  println!("2 batteries score: {}", activate_batteries(rawdata.clone(), 2));
  println!("12 batteries score: {}", activate_batteries(rawdata, 12));
}

// Given a list of batteries, find the sum of power rankings. Each battery is a string of digits
fn activate_batteries(data: String, num_batteries: u32) -> u64 {
  data.lines().fold(0_u64, |acc, row| {    
    let mut current_high_pos: usize = 0;
    
    let value: u64 = (0..num_batteries).rev().fold(0_u64, |acc, remainder| {
      let mut current_high: char = '0';
      
      for (index, digit) in row[..(row.len() - remainder as usize)].chars().enumerate().skip(current_high_pos) {
        if digit > current_high {
          current_high = digit;
          current_high_pos = index + 1;
          
          if digit == '9' {
            break;
          }
        }
      }
      
      acc + (current_high.to_digit(10).unwrap() as u64 * 10_u64.pow(remainder))
    });
    
    println!("{}", value);
    acc + value
  })
}
