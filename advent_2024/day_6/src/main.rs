use std::fs;
use std::env;
use std::isize;
use std::collections::HashSet;

const CARDINALS: [(isize, isize); 4] = [
  (0, -1),
  (1, 0),
  (0, 1),
  (-1, 0)
];

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

  let start_index = match rawdata.find('^') {
    Some(val) => val,
    None => { panic!("Broken Input?"); }
  };
  let mut map: Vec<Vec<char>> = rawdata.lines().map(|str| str.chars().collect()).collect();

  let height = map.len() as isize;
  let width = map[0].len() as isize;

  // Checking rawdata means width is +1, due to \n
  let mut guard_coord = index_to_coord(start_index, &((width + 1) as usize));
  let obstacles: Vec<(isize, isize)> = rawdata.match_indices('#').map(|(index, _)| {
    index_to_coord(index, &((width + 1) as usize))
  }).collect();

  let mut path_steps: Vec<((isize, isize), (isize, isize), usize)> = Vec::new();
  
  let walk_map = |pos: &(isize, isize), direction| { walk(pos.clone(), direction, (width, height), &obstacles) };

  // Track the guard's movement in the building.
  while (guard_coord.0.clamp(1, width - 2) == guard_coord.0) && (guard_coord.1.clamp(1, height - 2 as isize) == guard_coord.1) {
    for index in 0..CARDINALS.len() {
      let direction = &CARDINALS[index];
      let end = walk_map(&guard_coord, direction);
      for pos in path_between(&guard_coord, &end).iter() {
        map[pos.1 as usize][pos.0 as usize] = 'X';
      }
      path_steps.push((guard_coord, end.clone(), index));
      guard_coord = end;
      if !(
        (guard_coord.0.clamp(1, width - 2) == guard_coord.0) &&
        (guard_coord.1.clamp(1, height - 2 as isize) == guard_coord.1)
      ) { break; }
    }
  }

  let cover_count = map.iter().map(|row| {
    for c in row.iter() {
      print!("{c}");
    }
    println!();
    row.iter().filter(|c| **c == 'X').count()
  }).reduce(|acc, elem| acc + elem).unwrap();

  println!("Covered Spaces: {}", cover_count);

  // By moving 1 obstacle, how many loops can we create for the guard?
  for (start, end, dir_index) in path_steps.iter() {
    let check_dir = &CARDINALS[(*dir_index + 1) % CARDINALS.len()];
    for position in path_between(start, &(end.0 - CARDINALS[*dir_index].0, end.1 - CARDINALS[*dir_index].1)).iter() {
      match walk_map(position, check_dir) {
        (x, _) if x == width => { continue; },
        (_, y) if y == height => { continue; },
        (0, _) => { continue; },
        (_, 0) => { continue; },
        new_position => {
          let mut potential_coord = new_position;
          let mut index = (*dir_index + 2) % CARDINALS.len();
          let mut turns: HashSet<((isize, isize), usize)> = HashSet::new();
          while (potential_coord.0.clamp(1, width - 2) == potential_coord.0) && (potential_coord.1.clamp(1, height - 2 as isize) == potential_coord.1) {
            let direction = &CARDINALS[index];
            let next = walk_map(&potential_coord, direction);

            if index == *dir_index {
              let path = path_between(&potential_coord, &next);
              if path.contains(&position) {
                map[(position.1 + CARDINALS[*dir_index].1) as usize][(position.0 + CARDINALS[*dir_index].0) as usize] = 'O';
                break;
              }
            }

            if !turns.insert((next, index)) {
              map[(position.1 + CARDINALS[*dir_index].1) as usize][(position.0 + CARDINALS[*dir_index].0) as usize] = 'O';
              break;
            }

            index = (index + 1) % CARDINALS.len();
            potential_coord = next;
          }
        }
      }
    };
  };

  let loop_count = map.iter().map(|row| {
    for c in row.iter() {
      print!("{c}");
    }
    println!();
    row.iter().filter(|c| **c == 'O').count()
  }).reduce(|acc, elem| acc + elem).unwrap();

  println!("Loops: {}", loop_count);
}

fn index_to_coord(index: usize, width: &usize) -> (isize, isize) {
  ((index % width).try_into().unwrap(), (index / width).try_into().unwrap())
}

// All points between 2 points. Must be axis aligned.
fn path_between(start: &(isize, isize), end: &(isize, isize)) -> Vec<(isize, isize)> {
  if start.0 == end.0 {
    (start.1.min(end.1)..(start.1 + 1).max(end.1)).map(|pos| (start.0, pos)).collect()
  } else if start.1 == end.1 {
    (start.0.min(end.0)..(start.0 + 1).max(end.0)).map(|pos| (pos, start.1)).collect()
  } else {
    panic!("Attempting to go between non-aligned directions")
  }
}

// Raycast on the map, checking for any entries that exist in coord_list
fn scan_map<'a>(pos: &(isize, isize), dir: &(isize, isize), coord_list: &'a Vec<(isize, isize)>) -> Vec<&'a (isize, isize)> {
  coord_list.iter().filter(|item| {
    (item.1 == pos.1 && (item.0 - pos.0).signum() == dir.0.signum()) ||
    (item.0 == pos.0 && (item.1 - pos.1).signum() == dir.1.signum())
  }).collect()
}

// Travel from starting position until hitting an obstacle, or the edge of the map
fn walk(position: (isize, isize), direction: &(isize, isize), size: (isize, isize), obstacles: &Vec<(isize, isize)>) -> (isize, isize) {
  let obstacles_in_path = scan_map(&position, direction, obstacles);
  let hit_pos = obstacles_in_path.iter().min_by(|a, b| 
    if a.0 == b.0 {
      (a.1 * direction.1).cmp(&(b.1 * direction.1))
    } else {
      (a.0 * direction.0).cmp(&(b.0 * direction.0))
  });
  match hit_pos {
    Some(coord) => {
      (coord.0 - direction.0, coord.1 - direction.1)
    }
    None => {
      let (width, height) = size;
      match direction {
        (x, 0) => ((width / 2) + (width * x / 2), position.1 as isize),
        (0, y) => (position.0 as isize, (height / 2) + (height * y / 2)),
        _ => panic!("Malformed Direction")
      }
    }
  }
}
