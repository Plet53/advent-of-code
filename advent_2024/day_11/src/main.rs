use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = match args.next() {
    Some(arg) => arg,
    None => {
      panic!("remember the filename")
    },
  };

  let rawdata = fs::read_to_string(filename).unwrap();

  let starting_values: Vec<u128> = rawdata.split_whitespace().map(|num| num.parse().unwrap()).collect();
  let mut blink_map = HashMap::new();

  let mut count = |value, count| { count_stones(value, count, &mut blink_map) };

  let blinks_25 = starting_values.iter().fold(0, |sum, &stone| sum + count(stone, 25));
  println!("{}", blinks_25);

  let blinks_75 = starting_values.iter().fold(0, |sum, &stone| sum + count(stone, 75));
  println!("{}", blinks_75);
}


// Recursively iterate blinking stones, which have an odd formula. Keep a cached table of results.
fn count_stones(value: u128, blinks: u32, result_lookup: &mut HashMap<(u128, u32), u128>) -> u128 {
  if blinks == 0 {
    1
  } else {
    match result_lookup.get(&(value, blinks)) {
      Some(&count) => count,
      None => {
        let result = match value {
          0 => count_stones(1, blinks - 1, result_lookup),
          // Number of digits is Odd.
          value if value.ilog10() % 2 == 1 => {
            let split_value = 10u128.pow((value.ilog10() + 1) / 2);
            count_stones(value / split_value, blinks - 1, result_lookup) +
            count_stones(value % split_value, blinks - 1, result_lookup)
          },
          // Number of digits is Even.
          value => count_stones(value * 2024, blinks - 1, result_lookup),
        };
        result_lookup.insert((value, blinks), result);
        result
      }
    }
  }
}