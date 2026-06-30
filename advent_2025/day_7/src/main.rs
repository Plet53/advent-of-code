use std::fs;
use std::env;
use std::ops::Add;
use std::collections::HashMap;
use shared::grid::{Grid, CoordIter};
use shared::coordinates::Coordinate;

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

  let mut grid: Grid<char> = rawdata.lines().map(|line| line.chars().collect()).collect();
  
  // map data away from presentation.
  let mut timeline: HashMap<Coordinate, u64> = HashMap::new();
  
  let start_pos = match grid.find('S') {
    Some(coord) => coord,
    None => panic!("input file does not contain start point"),
  };
  
  match grid.get_mut(&(start_pos + Coordinate::DOWN)) {
    None => panic!("input file is 1 row long"),
    Some(val) => {
      timeline.insert(start_pos + Coordinate::DOWN, 1);
      *val = '|';
    }
  };
  
  let coord_iter = CoordIter::new(grid.height() as isize, grid.width() as isize);
  
  // Split down the visual grid at '^'.
  // first row is guaranteed not to be helpful to our cause.
  for coord in coord_iter.skip(grid.width()) {
    match grid.get(&coord) {
      None => (),
      Some('|') => {
        let quantum_val = timeline[&coord];
        let down = coord + Coordinate::DOWN;
        
        match grid.get_mut(&down) {
          None => (),
          // Continue data beam, sum number of branches that hit this point.
          Some(val) if *val != '^' => {
            *timeline.entry(down).or_insert(0) += quantum_val;
            *val = '|';
          },
          // Split the data beam at Splitters
          Some('^') => {
            for side_coord in [Coordinate::LEFT, Coordinate::RIGHT] {
              match grid.get_mut(&(down + side_coord)) {
                None => (),
                Some(val) => {
                  *timeline.entry(down + side_coord).or_insert(0) += quantum_val;
                  *val = '|';
                },
              };
            }
          },
          Some(_) => (),
        }
      },
      Some(_) => (),
    };
  };

  // Get the sum of the final values at the end of the multi split process.
  let quantum_value = timeline.iter().filter(|(key, _value)| key.y == (grid.height() - 1) as isize).map(|(key, value)| *value).reduce(u64::add).unwrap();
  
  // Count the number of times the Beam Splits.
  let split_count = grid.coord_iter().filter(|coord| match grid.get(&coord) {
    Some('^') => true,
    _ => false,
  }).filter(|coord| match grid.get(&(*coord + Coordinate::UP)) {
    Some('|') => true,
    _ => false
  }).count();
  
  println!("{}", grid);
  println!("{}", split_count);
  println!("{}", quantum_value);
}
