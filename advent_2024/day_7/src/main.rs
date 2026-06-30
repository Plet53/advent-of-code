use std::fs;
use std::env;

#[derive(Clone, PartialEq, Debug)]
enum OperatorType {
  Add,
  Mult,
  Concat
}

impl OperatorType {
  fn next(&self) -> OperatorType {
    match *self {
      OperatorType::Add => OperatorType::Mult,
      OperatorType::Mult => OperatorType::Concat,
      OperatorType::Concat => OperatorType::Add
    }
  }
}

#[derive(Debug)]
struct OperatorSet {
  set: Vec<OperatorType>,
  end: bool,
}

impl OperatorSet {
  fn new(count: usize) -> OperatorSet {
    OperatorSet { set: vec![OperatorType::Add; count], end: false }
  }
}

impl Iterator for OperatorSet {
  type Item = Vec<OperatorType>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.end { return None; }
    
    for op in self.set.iter_mut() {
      *op = op.next();
      
      if *op != OperatorType::Add {
        break;
      }
    }
    
    if self.set.iter().all(|op| *op == OperatorType::Add) {
      self.end = true;
    }
    Some(self.set.clone())
  }
}

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

  let sum: u64 = rawdata.lines().map(|str| {
    let mut data = str.split(": ");
    let target: u64 = data.next().unwrap().parse::<u64>().unwrap();
    let figures: Vec<u64> = data.next().unwrap().split(' ').map(|val| val.parse::<u64>().unwrap()).collect();
    let mut ret_val = 0;

    for mask in 0u64..2u64.pow(figures.len() as u32 - 1) {
      let mut combo = figures[0];
      for index in 0..(figures.len() as u64  - 1) {
        let position = 2u64.pow(index as u32);
        if (mask & position) == position {
          combo *= figures[index as usize + 1];
        } else {
          combo += figures[index as usize + 1];
        }
      }

      if combo == target {
        ret_val = target;
        break;
      }
    }

    ret_val
  }).sum();

  println!("Sum of correct values (basic): {}", sum);

  let ex_sum: u64 = rawdata.lines().map(|str| {
    let mut data = str.split(": ");
    let target: u64 = data.next().unwrap().parse::<u64>().unwrap();
    let figures: Vec<u64> = data.next().unwrap().split(' ').map(|val| val.parse::<u64>().unwrap()).collect();
    let mut ret_val = 0;

    for mask in OperatorSet::new(figures.len() - 1) {
      let mut combo = figures[0];
      for index in 0..(figures.len() - 1) {
        match mask[index] {
          OperatorType::Add => { combo += figures[index + 1]; },
          OperatorType::Mult => { combo *= figures[index + 1]; },
          OperatorType::Concat => { 
            combo = (combo.to_string() + &figures[index + 1].to_string()).parse().unwrap();
          }
        }
      } 

      if combo == target {
        ret_val = target;
        break;
      }
    }

    ret_val
  }).sum();

  println!("Sum of correct values (expanded): {}", ex_sum);
}
