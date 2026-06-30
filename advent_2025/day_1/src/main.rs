use std::fs;
use std::env;
use std::cmp::Ordering;

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
  
  let mut pos: i32 = 50;
  let mut v1_count: u32 = 0;
  let mut v2_count: u32 = 0;
  
  for row in rawdata.lines() {
    let dist = row[1..].parse::<i32>().unwrap();
    let dir = row.chars().nth(0).unwrap();
    let last_pos = pos.clone();
    
    pos = match dir {
      'R' => {
        pos + dist
      },
      'L' => {
        pos - dist
      },
      _ => {
        println!("unexpected direction");
        continue;
      }
    } % 100;
    
    v2_count += dist as u32 / 100;
    
    match pos.cmp(&0) {
      Ordering::Equal => {
        v1_count += 1;
        v2_count += 1;
      },
      Ordering::Less => {
        pos += 100;
      },
      Ordering::Greater => (),
    };
    
    if last_pos != 0 && pos != 0 {
      match (dir, pos.cmp(&last_pos)) {
        ('L', Ordering::Greater) => {
          v2_count += 1;
        },
        ('R', Ordering::Less) => {
          v2_count += 1;
        },
        _ => (),
      }
    }
    
    println!("{}, {}, {}, {}, {}, {}", last_pos, row, pos, dist / 100, v1_count, v2_count);
  }
  
  println!("part_1 count: {}", v1_count);
  println!("part_2 count: {}", v2_count);
}
