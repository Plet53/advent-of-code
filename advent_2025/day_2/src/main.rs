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
  
  let mut rawdata = fs::read_to_string(filename).unwrap();
  
  let _ = rawdata.pop(); // Peel off the endline

  part_1(rawdata.clone());
  part_2(rawdata);
}


fn part_1(rawdata: String) {
  let mut score: u128 = 0;
  
  for range_str in rawdata.split(',') {
    let [range_begin_str, range_end_str]: [&str; 2] = {
      range_str.split('-').collect::<Vec<&str>>().try_into().unwrap()
    };
    
    if range_begin_str.len() == range_end_str.len() && range_begin_str.len() % 2 == 1 {
      continue;
    }
    
    let range_end: u64 = match range_end_str.len() % 2 {
      1 => 10_u64.pow((range_end_str.len() - 1).try_into().unwrap()) - 1,
      0 => range_end_str.parse::<u64>().unwrap(),
      _ => { continue; }
    };
    
    let range_begin: u64 = match range_begin_str.len() % 2 {
      1 => {
        let target_len: u32 = range_begin_str.len().try_into().unwrap();
        10_u64.pow(target_len) + 10_u64.pow(target_len / 2)
      },
      0 => range_begin_str.parse::<u64>().unwrap(),
      _ => { continue; }
    };
    
    if range_begin > range_end {
      continue; // Ranges not real, continue
    }
      
    let mut range_begin_lower = range_begin / 10_u64.pow(range_begin.ilog10() / 2 + 1);
    if range_begin_lower * 10_u64.pow(range_begin.ilog10() / 2 + 1) + range_begin_lower < range_begin {
      range_begin_lower += 1;
    }
    let mut range_end_lower = range_end / 10_u64.pow(range_end.ilog10() / 2 + 1);
    if range_end_lower * 10_u64.pow(range_end.ilog10() / 2 + 1) + range_end_lower > range_end {
      range_end_lower -= 1;
    }
    
    if range_begin_lower > range_end_lower {
      continue; // Ranges Still Not Real
    }
    
    println!("range: {}, {}", range_begin, range_end);
    println!("range_lowers: {}, {}", range_begin_lower, range_end_lower);
    
    score += (range_begin_lower..(range_end_lower + 1)).map(|value| 
      value * 10_u64.pow(value.ilog10() + 1) + value
    ).sum::<u64>() as u128;
  }
  
  println!("part_1: {}", score);
}


fn part_2(rawdata: String) {
  let mut score: u128 = 0;

  for range_str in rawdata.split(',') {
    println!("{}", range_str);
    let [range_begin, range_end]: [u64; 2] = {
      range_str.split('-').map(|string| string.parse::<u64>().unwrap()).collect::<Vec<u64>>().try_into().unwrap()
    };
    
    for num in range_begin..(range_end + 1) {
      let num_string = num.to_string();
      let string_len = num_string.len();
      for slice_size in 1..((string_len / 2) + 1) {
        if string_len % slice_size != 0 {
          continue;
        }
        
        let mut is_all_repeats = true;
        for index in 0..((string_len / slice_size) - 1) {
          let charset = num_string.get((index * slice_size)..((index + 1) * slice_size)).unwrap();
          match num_string.get(((index + 1) * slice_size)..((index + 2) * slice_size)) {
            Some(other_charset) => {
              if charset != other_charset {
                is_all_repeats = false;
                break;
              }
            },
            None => (),
          }
        }
        
        if is_all_repeats {
          println!("{}", num);
          score += num as u128;
          break;
        }
      }
    }
  }
  
  println!("part_2 {}", score);
}
