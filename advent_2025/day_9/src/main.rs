use std::fs;
use std::env;
use std::collections::HashMap;
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
  
  let red_tiles: Vec<Coordinate> = rawdata.lines().map(|line| {
    let coord_vals: Vec<isize> = line.split(',').map(|string| string.parse::<isize>().unwrap()).collect();
    Coordinate::new(coord_vals[0], coord_vals[1])
  }).collect();
  
  part_1(red_tiles.clone());
  part_2(red_tiles);
}

fn part_1(red_tiles: Vec<Coordinate>) {
  let max_val = red_tiles[..(red_tiles.len() - 1)].iter().enumerate().fold(0_u64, |acc, (index, coord)| {
    acc.max(red_tiles[(index + 1)..].iter().fold(0_u64, |in_acc, o_coord| {
      in_acc.max(((o_coord.x.abs_diff(coord.x) + 1) * (o_coord.y.abs_diff(coord.y) + 1)) as u64)
    }))
  });
  
  println!("part_1: {max_val}");
}

fn part_2(red_tiles: Vec<Coordinate>) {
  let mut direction_map: HashMap<Coordinate, Coordinate> = red_tiles.windows(2).fold(HashMap::new(), |mut map, window| {
    map.insert(window[0], (window[1] - window[0]).direction_of());
    map
  });
  
  direction_map.insert(red_tiles[red_tiles.len() - 1], (red_tiles[0] - red_tiles[red_tiles.len() - 1]).direction_of());
  
  let tile_pairs: Vec<(Coordinate, Coordinate)> = red_tiles[..(red_tiles.len() - 1)].iter().enumerate().fold(Vec::new(), |mut collection, (index, coord)| {
    for o_coord in red_tiles[(index + 1)..].iter() {
      if o_coord.x == coord.x || o_coord.y == coord.y {
        continue;
      }
      
      let (rect_1, rect_2) = (
        Coordinate::new(coord.x.min(o_coord.x), coord.y.min(o_coord.y)),
        Coordinate::new(coord.x.max(o_coord.x), coord.y.max(o_coord.y))
      );
      
      let cw: bool = rect_1 == *coord;
      
      if direction_map.iter().filter(|(point, _dir)| {
        point.x >= rect_1.x && point.y >= rect_1.y && point.x <= rect_2.x && point.y <= rect_2.y
      }).all(|(point, dir)| {
        if cw {
          match *dir {
            Coordinate::UP => point.x == rect_2.x || point.y == rect_1.y,
            Coordinate::RIGHT => point.x == rect_2.x || point.y == rect_2.y,
            Coordinate::DOWN => point.x == rect_1.x || point.y == rect_2.y,
            Coordinate::LEFT => point.x == rect_1.x || point.y == rect_1.y,
            _ => panic!("direction_map contains not cardinal"),
          }
        } else {
          match *dir {
            Coordinate::UP => point.x == rect_1.x || point.y == rect_1.y,
            Coordinate::RIGHT => point.x == rect_2.x || point.y == rect_1.y,
            Coordinate::DOWN => point.x == rect_2.x || point.y == rect_2.y,
            Coordinate::LEFT => point.x == rect_1.x || point.y == rect_2.y,
            _ => panic!("direction_map contains not cardinal"),
          }
        }
      }) {
        collection.push((*coord, *o_coord));
      }
    }
    collection
  });
  
  for pair in tile_pairs.iter() {
    println!("{pair:?}");
  }
  
  let mut values: Vec<usize> = tile_pairs.iter().map(|(rect_1, rect_2)| (rect_1.x.abs_diff(rect_2.x) + 1) * (rect_1.y.abs_diff(rect_2.y) + 1)).collect();
  values.sort();
  
  for max_val in values.iter().rev().take(10) {
    println!("part 2: {max_val}");
  }
}
