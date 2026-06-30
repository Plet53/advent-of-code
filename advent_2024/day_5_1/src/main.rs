use std::collections::HashMap;
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
  let data: Vec<&str> = rawdata.split("\n\n").collect();

  let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
  for rule in data[0].lines().map(|rule| rule.split('|').collect::<Vec<&str>>()) {
    let lead = rule[0];
    let follow = rule[1];
    match rules.get_mut(lead) {
      Some(list) => { list.push(follow); },
      None => { rules.insert(lead, Vec::from([follow])); }
    }
  }

  let order_by_rules = |lhv: &&str, rhv: &&str| { order_to_rules(*lhv, *rhv, &rules) };

  let mut correct_counter = 0;
  let mut incorrect_counter = 0;

  for sequence in data[1].lines() {
    let mut values = sequence.split(',').collect::<Vec<&str>>();
    let mut sorted = true;
    if !values.is_sorted_by(|lhv, rhv| { order_by_rules(lhv, rhv) == Ordering::Less }) {
      values.sort_by(order_by_rules);
      sorted = false;
    }

    let val: u64 = values[values.len() / 2].parse().unwrap_or(0);

    if sorted {
      correct_counter += val;
    } else {
      incorrect_counter += val;
    }
  }

  println!("Correct Values: {}", correct_counter);
  println!("Incorrect Values: {}", incorrect_counter);
}

fn order_to_rules(lhv: &str, rhv: &str, map: &HashMap<&str, Vec<&str>>) -> Ordering {
  match map.get(lhv) {
    Some(rule) => {
      if rule.contains(&rhv) {
        return Ordering::Less
      }
    },
    None => {
      match map.get(rhv) {
        Some(rule) => {
          if rule.contains(&lhv) {
            return Ordering::Greater;
          }
        },
        None => ()
      }
    }
  }
  Ordering::Equal
}