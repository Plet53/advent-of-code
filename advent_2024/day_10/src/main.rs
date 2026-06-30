use std::fs;
use std::env;
use std::collections::HashSet;
use shared::grid::Grid;
use shared::coordinates::Coordinate;
use shared::coordinates::CARDINALS;

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

  let map: Grid<u8> = Grid::from_iter(rawdata.lines().map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect()));

  let trailhead_score = rawdata.match_indices('0').fold(0, |acc, (index, _)| {
    let mut coords = vec![Coordinate::index_to_coord(index as isize, map.width() as isize + 1)];
    for value in 1u8..=9 {
      let mut new_coords: HashSet<Coordinate> = HashSet::new();
      for coord in coords.iter() {
        for direction in CARDINALS.iter() {
          let next = *coord + direction;
          match map.get(&next) {
            None => continue,
            Some(x) if *x == value => {
              let _ = new_coords.insert(next);
            },
            Some(_) => continue,
          }
        }
      }
      coords = new_coords.drain().collect();
    }
    acc + coords.len()
  });

  let trailhead_rating = rawdata.match_indices('0').fold(0, |acc, (index, _)| {
    let mut coords = vec![Coordinate::index_to_coord(index as isize, map.width() as isize + 1)];
    for value in 1u8..=9 {
      let mut new_coords: Vec<Coordinate> = Vec::new();
      for coord in coords.iter() {
        for direction in CARDINALS.iter() {
          let next = *coord + direction;
          match map.get(&next) {
            None => continue,
            Some(x) if *x == value => {
              new_coords.push(next);
            },
            Some(_) => continue,
          }
        }
      }
      coords = new_coords.clone();
    }
    acc + coords.len()
  });

  println!("{}", trailhead_score);
  println!("{}", trailhead_rating);
}
