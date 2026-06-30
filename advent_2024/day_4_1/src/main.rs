use std::fs;
use std::env;
// Word Searcher, but only for the word XMAS.

// Written pre-coordinate and grid library
const DIRECTIONS: [(i8, i8); 8] = [
  (1, 0),
  (0, 1),
  (-1, 0),
  (0, -1),
  (1, 1),
  (-1, 1),
  (-1, -1),
  (1, -1)
];

const SEQUENCE: [char; 3] = ['M', 'A', 'S'];

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

  let search_area: Vec<Vec<char>> = rawdata.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

  let height = search_area.len() as isize;
  let width = search_area[0].len() as isize;

  let mut line_counter = 0;
  let mut cross_counter = 0;

  
  for i in 0..(search_area.len() as isize * search_area[0].len() as isize) {
    let c = search_area[(i / width) as usize][(i % width) as usize];
    if c == 'X' {
      line_counter += scan_area_line(i, &(width, height), &search_area);
    }
    if c == 'A' {
      cross_counter += scan_area_cross(i, &(width, height), &search_area);
    }
  }

  println!("Match Count: {}", line_counter);
  println!("Cross Count: {}", cross_counter);
}

pub fn scan_area_line(index: isize, size: &(isize, isize), area: &Vec<Vec<char>>) -> u64 {
  DIRECTIONS.iter().map(|direction| {
    let mut coord = index_to_coord(index, &size.0);
    if !bounds_check(&coord, &size, direction, 3) { return 0; };

    for letter in SEQUENCE.iter() {
      coord = (
        coord.0 + (direction.0 as isize),
        coord.1 + (direction.1 as isize)
      );
      if area[coord.1 as usize][coord.0 as usize] != *letter {
        return 0;
      }
    };

    1
  }).reduce(|acc, val| acc + val).unwrap()
}

// MXM
// XAX
// SAS
// or any of its rotations
pub fn scan_area_cross(index: isize, size: &(isize, isize), area: &Vec<Vec<char>>) -> u64 {
  let coord = index_to_coord(index, &size.0);
  if !(
    bounds_check(&coord, &size, &(-1, -1), 1) && 
    bounds_check(&coord, &size, &(1, 1), 1)
  ) { return 0; };
  match (area[(coord.1 - 1) as usize][(coord.0 - 1) as usize], area[(coord.1 - 1) as usize][(coord.0 + 1) as usize]) {
    ('M', 'M') => {
      if (area[(coord.1 + 1) as usize][(coord.0 - 1) as usize], area[(coord.1 + 1) as usize][(coord.0 + 1) as usize]) == ('S', 'S') {
        1
      } else {
        0
      }
    },
    ('S', 'S') => {
      if (area[(coord.1 + 1) as usize][(coord.0 - 1) as usize], area[(coord.1 + 1) as usize][(coord.0 + 1) as usize]) == ('M', 'M') {
        1
      } else {
        0
      }
    },
    ('M', 'S') => {
      if (area[(coord.1 + 1) as usize][(coord.0 - 1) as usize], area[(coord.1 + 1) as usize][(coord.0 + 1) as usize]) == ('M', 'S') {
        1
      } else {
        0
      }
    },
    ('S', 'M') => {
      if (area[(coord.1 + 1) as usize][(coord.0 - 1) as usize], area[(coord.1 + 1) as usize][(coord.0 + 1) as usize]) == ('S', 'M') {
        1
      } else {
        0
      }
    },
    _ => { 0 }
  }
}

pub fn bounds_check(coord: &(isize, isize), size: &(isize, isize), direction: &(i8, i8), length: i8) -> bool {
  let end: (isize,  isize) = (coord.0 + (direction.0 * length) as isize, coord.1 + (direction.1 * length) as isize);
  if end.0 > -1 && end.1 > -1 {
    if end.0 < size.0  && end.1 < size.1 {
      return true;
    }
  }
  false
}

pub fn index_to_coord(index: isize, width: &isize) -> (isize, isize) {
  (index as isize % width, index as isize / width)
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_DATA: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

  #[test]
  fn index_to_coord_test() {
    let coord = index_to_coord(25, &5);
    assert_eq!(coord.0, 0);
    assert_eq!(coord.1, 5);

    let coord = index_to_coord(28, &5);
    assert_eq!(coord.0, 3);
    assert_eq!(coord.1, 5);

    let coord = index_to_coord(4, &5);
    assert_eq!(coord.0, 4);
    assert_eq!(coord.1, 0);
  }

  #[test]
  fn bounds_test() {
    let coord: (isize, isize) = (0, 0);

    assert!(!bounds_check(&coord, &(0, 1), &(-1, 0), 3));
    assert!(bounds_check(&coord, &(4, 1), &(1, 0), 3));
  }

  #[test]
  fn scan_area_line_test() {
    let search_area: Vec<Vec<char>> = TEST_DATA.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let height = search_area.len() as isize;
    let width = search_area[0].len() as isize;

    let mut counter = 0;
    for i in 0..(search_area.len() as isize * search_area[0].len() as isize) {
      if search_area[(i / width) as usize][(i % width) as usize] == 'X' {
        counter += scan_area_line(i, &(width, height), &search_area);
      }
    }

    assert_eq!(counter, 18);
  }

  #[test]
  fn scan_area_cross_test() {
    let search_area: Vec<Vec<char>> = TEST_DATA.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let height = search_area.len() as isize;
    let width = search_area[0].len() as isize;

    let mut counter = 0;
    for i in 0..(search_area.len() as isize * search_area[0].len() as isize) {
      if search_area[(i / width) as usize][(i % width) as usize] == 'A' {
        counter += scan_area_cross(i, &(width, height), &search_area);
      }
    }

    assert_eq!(counter, 9);
  }
}