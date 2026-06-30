use std::fs;
use std::env;
use std::ops::{Add, Mul};
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
  
  let straight_sheet: Grid<&str> = rawdata.lines().map(|row| row.split_ascii_whitespace().collect()).collect();
  
  let p1_total: u64 = straight_sheet.column_iter().map(|mut column| {
    let operator = match *(column.pop().unwrap()) {
      "+" => u64::add,
      "*" => u64::mul,
      &_ => panic!("wrong operator symbol"),
    };
    
    column.iter().map(|&&num| num.parse::<u64>().unwrap()).reduce(operator).unwrap()
  }).sum();
  
  println!("part_1: {}", p1_total);
  
  let mut char_lines: Vec<Vec<char>> = rawdata.lines().map(|row| row.chars().collect()).collect();
  
  let op_line: Vec<char> = char_lines.pop().unwrap();
  
  let mut op_positions: Vec<(usize, char)> = op_line.iter().enumerate().filter(|(_index, character)| {
    **character != ' '
  }).map(|(index, character)| (index, *character)).collect();
  op_positions.push((char_lines[0].len(), '_'));
  
  let char_sheet: Grid<char> = char_lines.iter().map(|line| line.clone()).collect();
  
  // Commentary from the future: I don't precisely remember what the problem statement is. This map seems to
  // Combine both columns by the operator from the second column.
  let p2_total: u64 = op_positions.windows(2).map(|window| {
    let number_columns = char_sheet.column_iter();
    let column_count = window[1].0 - window[0].0;
    let operator = match window[0].1 {
      '+' => u64::add,
      '*' => u64::mul,
      _ => panic!("wrong operator symbol"),
    };
    
    number_columns.skip(window[0].0).take(column_count).map(|column| {
      let mut string = column.iter().fold(String::new(), |mut acc, char| {
        acc.push(**char);
        acc
      });
      string.retain(|c| c != ' ');
      
      match string.parse::<u64>() {
        Ok(num) => {
          num
        },
        Err(_) => match window[0].1 {
          '*' => 1,
          _ => 0
        },
      }
    }).reduce(operator).unwrap()
  }).sum();
  
  println!("part_2: {}", p2_total);
}
