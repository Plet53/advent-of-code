use std::fs;
use std::env;
use std::collections::HashMap;
use shared::grid::Grid;
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

  let mut map: Grid<char> = Grid::from_iter(rawdata.lines().map(|line| line.chars().collect()));

  let stations: HashMap<char, Vec<Coordinate>> = rawdata.match_indices(|c| match c {
    '.' | '\n' => false,
    _ => true
  }).fold(HashMap::new(), |mut stations, (index, s)| {
    let c = s.chars().next().unwrap();
    if stations.contains_key(&c) {
      stations.get_mut(&c).unwrap().push(Coordinate::index_to_coord(index as isize, map.width() as isize + 1));
    } else {
      stations.insert(c, vec![Coordinate::index_to_coord(index as isize, map.width() as isize + 1)]);
    }
    stations
  }); 

  // an Antinode is a position whose distance is the same as the distance to the opposite station, and each antinode.
  // This comes out to being all points equidistant on the line drawn between both stations
  for coords in stations.values() {
    for index in 0..coords.len() - 1 {
      let coord_1 = coords[index];
      for &coord_2 in &coords[(index + 1)..] {
        let diff = coord_1 - coord_2;

        let mut diff_1 = coord_1 + diff;
        let mut diff_2 = coord_2 - diff;

        while map.in_bounds(&diff_1) {
          map.set(&diff_1, '#');
          diff_1 += diff;
        }

        while map.in_bounds(&diff_2) {
          map.set(&diff_2, '#');
          diff_2 -= diff;
        }
      }
    }
  }

  let antinodes = stations.keys().fold(map.count_occurances('#'), |acc, c| {
    acc + map.count_occurances(*c)
  });

  print!("{}", map);
  println!("Unique Antinodes: {}", antinodes);
}
