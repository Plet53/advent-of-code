use std::fs;
use std::env;
use shared::{grid::Grid, coordinates};

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
  
  let mut grid: Grid<char> = rawdata.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
  
  part_1(&grid);
  part_2(grid);
}
  
fn part_1(grid: &Grid::<char>) {
  let num_accessible_rolls = grid.coord_iter().filter(|&coord| {
    // Match adjacent symbols. If occupied (== '@') >  a given '@' is inaccessible
    grid.get(&coord) == Some(&'@') && coordinates::DIRECTIONS.iter().filter(|&&dir| {
      match grid.get(&(coord + dir)) {
        Some(&symbol) => {
          symbol == '@'
        },
        None => false
      }
    }).count() < 4
  }).map(|coord| {
    println!("{}", coord);
    coord
  }).count();
  
  println!("part_1: {}", num_accessible_rolls);
}

fn part_2(mut grid: Grid::<char>) {
  let mut total_accessible_rolls: u64 = 0;

  // Repeat the same process, but this time, removing accessible rolls. Some will simply remain inaccessible.
  loop {
    let accessible_rolls: Vec<coordinates::Coordinate> = grid.coord_iter().filter(|&coord| {
    grid.get(&coord) == Some(&'@') && coordinates::DIRECTIONS.iter().filter(|&&dir| {
      match grid.get(&(coord + dir)) {
        Some(&symbol) => {
          symbol == '@'
        },
        None => false
      }
    }).count() < 4
    }).collect();
    
    let num_accessible_rolls = accessible_rolls.iter().map(|coord| {
      match grid.get_mut(&coord) {
        None => (),
        Some(pointer) => {
          *pointer = 'x'
        }
      }
    }).count();
    
    total_accessible_rolls += num_accessible_rolls as u64;
    if num_accessible_rolls == 0 {
      break
    }
  }
  
  println!("part_2: {}", total_accessible_rolls);
}
