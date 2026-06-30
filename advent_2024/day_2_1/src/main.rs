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

  let mut safecount: u32 = 0;
  let mut extendedsafecount: u32 = 0;

  for dataset in rawdata.lines() {
    let data: Vec<i64> = dataset.split_whitespace().map(|string| {
      string.parse().unwrap()
    }).collect();

    safecount = safecount + safety_test(&data);

    let mut countextended = 0;
    for index in 0..data.len() {
      let mut skip_data = data.clone();
      skip_data.remove(index);
      countextended = countextended.max(safety_test(&skip_data));
    }
    extendedsafecount = extendedsafecount + countextended;
  }

  println!("{}", safecount);
  println!("{}", extendedsafecount);
}

fn safety_test(data: &Vec<i64>) -> u32 {
  let mut lasttest: i64 = data[0] - data[1];

  if lasttest > 3 || lasttest < -3 || lasttest == 0 {
    return 0;
  }

  for index in 2..data.len() {
    let test: i64 = data[index - 1] - data[index];

    if test > 3 || test < -3 || test == 0 {
      return 0;
    }
    if test.signum() != lasttest.signum() {
      return 0;
    }

    lasttest = test;
  }

  return 1;
}