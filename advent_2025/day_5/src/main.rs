use std::fs;
use std::env;
use std::cmp::{Ordering, max};

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
  
  // The first database segment is a pile of ranges of ids, which might be bad.
  let mut database_parts = rawdata.split("\n\n");
  
  let fresh_ranges = database_parts.next().unwrap().lines().map(|line| {
    let mut parts = line.split('-');
    
    parts.next().unwrap().parse::<u64>().unwrap()..=parts.next().unwrap().parse::<u64>().unwrap()
  });
  
  let mut sorted_ranges: Vec<_> = fresh_ranges.collect();
  sorted_ranges.sort_by(|range_a, range_b| {
    match range_a.start().cmp(&range_b.start()) {
      Ordering::Equal => {
        range_a.end().cmp(&range_b.end())
      },
      ord => ord,
    }
  });

  // Create a set of non-overlapping ranges out of a set of otherwise arbitrarily made ranges
  let mut cleaned_ranges: Vec<_> = Vec::new();
  
  let mut current_range = sorted_ranges[0].clone();
  for index in 1..sorted_ranges.len() {
    match current_range.end().cmp(sorted_ranges[index].start()) {
      Ordering::Greater | Ordering::Equal => {
        current_range = *current_range.start()..=max(*sorted_ranges[index].end(), *current_range.end());
      },
      Ordering::Less => {
        cleaned_ranges.push(current_range.clone());
        current_range = sorted_ranges[index].clone();
      }
    }
  }
  
  cleaned_ranges.push(current_range);
  
  // The second segment of the database is a set of known bad IDs
  // Check for residency within the cleaned ranges of specific IDs
  println!("\n{}, {}", sorted_ranges.len(), cleaned_ranges.len());
  let id_count = database_parts.next().unwrap().lines().filter(|line| {
    let id = line.parse::<u64>().unwrap();
    
    for range in cleaned_ranges.clone() {
      if range.contains(&id) {
        return true;
      }
    }
    
    false
  }).count();
  
  println!("\npart 1: {}", id_count);
  
  let total_id_count: u64 = cleaned_ranges.iter().map(|range| {
    range.end() - range.start() + 1
  }).sum();
  
  println!("part2: {}", total_id_count);
}
