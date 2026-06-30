use std::fs;
use std::env;
use std::collections::HashMap;
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

  let map: Grid<char> = Grid::from_iter(rawdata.lines().map(|line| line.chars().collect()));
  let mut known_map: HashMap<Coordinate, (char, u64, usize)> = HashMap::new();
  let mut region_id = 0;

  let total = map.coord_iter().fold(0, |acc, coord| {
    if known_map.contains_key(&coord) { return acc; }
    let &match_char = map.get(&coord).unwrap();
    let mut outer_id = if let Some((_, _, id)) = known_map.get(&(coord + Coordinate::UP)) {
      Some(*id)
    } else {
      None
    };
    let mut current_coords = vec![coord];
    let mut next_coords: Vec<Coordinate> = Vec::new();
    let mut region: HashSet<Coordinate> = HashSet::from([coord]);

    loop {
      for coordinate in current_coords {
        for dir in CARDINALS {
          let next = coordinate + dir;
          match map.get(&next) {
            None => outer_id = None,
            Some(&c) if c == match_char => {
              if region.insert(next) {
                next_coords.push(next);
              }
            },
            Some(_) if outer_id.is_some() => {
              outer_id = if let Some((_, _, id)) = known_map.get(&next) {
                if *id != outer_id.unwrap() {
                  None
                } else {
                  outer_id
                }
              } else {
                None
              };
            }
            Some(_) => continue,
          }
        }
      }
      if next_coords.is_empty() { break; }
      current_coords = next_coords.drain(..).collect();
    }

    let perimeter = get_perimeter(&region);
    let area = region.len() as u64;
    for coordinate in region.iter() {
      let _ = known_map.insert(*coordinate, (match_char, area, region_id));
    }
    region_id += 1;
    println!("{}: {} X {} = {}", match_char, area, perimeter, perimeter * area);
    acc + (area * perimeter) + if outer_id.is_some() {
      println!("Subarea Detected {}: {} X {} = {}", known_map.get(&(coord + Coordinate::UP)).unwrap().0, known_map.get(&(coord + Coordinate::UP)).unwrap().1, perimeter, perimeter * known_map.get(&(coord + Coordinate::UP)).unwrap().1);
      known_map.get(&(coord + Coordinate::UP)).unwrap().1 * perimeter
    } else {
      0
    }
  });

  println!("Total Cost: {}", total);
}

fn get_perimeter(coordinates: &HashSet<Coordinate>) -> u64 {
  let &top_corner = coordinates.iter().min().unwrap();
  let mut dir_index = 1;
  let mut pos = top_corner;
  let mut r_val = 0;
  // Attempt to walk the borders of the region clockwise
  loop {
    // Try to turn left, then try to go forward, otherwise, turn right
    if coordinates.contains(&(pos + CARDINALS[(dir_index + 3) % 4])) {
      dir_index = (dir_index + 3) % 4;
      pos += CARDINALS[dir_index];
      r_val -= 1;
    } else if coordinates.contains(&(pos + CARDINALS[dir_index])) {
      pos += CARDINALS[dir_index];
    } else {
      dir_index = (dir_index + 1) % 4;
    }
    r_val += 1;
    if dir_index == 1 && pos == top_corner {
      break;
    }
  }
  r_val
}