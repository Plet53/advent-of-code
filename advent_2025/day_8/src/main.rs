use std::fs;
use std::env;
use std::ops::Mul;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vector3(u64, u64, u64);

impl Vector3 {
  fn distance_to_squared(&self, other: Self) -> u64 {
    (self.0.abs_diff(other.0).pow(2)) +
    (self.1.abs_diff(other.1).pow(2)) +
    (self.2.abs_diff(other.2).pow(2))
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
  
  let points: Vec<Vector3> = rawdata.lines().map(|line| {
    let vec: Vec<&str> = line.split(',').collect();
    Vector3(vec[0].parse::<u64>().unwrap(), vec[1].parse::<u64>().unwrap(), vec[2].parse::<u64>().unwrap())
  }).collect();
  
  let mut dist_map: HashMap<u64, (Vector3, Vector3)> = HashMap::new();
  
  for i in 0..points.len() {
    let point = points[i];
    for other_point in &points[(i + 1)..] {
      let dist_squared = point.distance_to_squared(*other_point);
      dist_map.insert(dist_squared, (point, *other_point));
    }
  }
  
  let mut sorted_keys: Vec<&u64> = dist_map.keys().collect();
  sorted_keys.sort();
  
  let mut circuits: Vec<HashSet<Vector3>> = points.iter().map(|point|
    HashSet::from([*point])
  ).collect();
  
  // Create a grand union circuit from the 1000 shortest connections
  for key in sorted_keys[0..1000].iter() {
    let (point_1, point_2) = dist_map[key];
    
    let circuits_to_union: Vec<HashSet<Vector3>> = circuits.extract_if(.., |circuit| {
      circuit.contains(&point_1) || circuit.contains(&point_2)
    }).collect();
    
    let union_circuit: HashSet<Vector3> = circuits_to_union.into_iter().reduce(|acc, e| acc.union(&e).into_iter().map(|&val| val).collect()).unwrap();
    
    circuits.push(union_circuit);
  }

  let mut circuit_lengths: Vec<usize> = circuits.iter().map(|circuit| {
    circuit.len()
  }).collect();
  circuit_lengths.sort_by(|a, b| b.cmp(&a));
  
  let top_3_mult = circuit_lengths.iter().take(3).fold(1_usize, usize::mul);
  
  println!("top 3 after 1k: {}", top_3_mult);
  
  let mut last_connection: Option<(Vector3, Vector3)> = None;
  for key in sorted_keys[1000..].iter() {
    let (point_1, point_2) = dist_map[key];
    last_connection = Some((point_1, point_2));
    
    let circuits_to_union: Vec<HashSet<Vector3>> = circuits.extract_if(.., |circuit| {
      circuit.contains(&point_1) || circuit.contains(&point_2)
    }).collect();
    
    let union_circuit: HashSet<Vector3> = circuits_to_union.into_iter().reduce(|acc, e| acc.union(&e).into_iter().map(|&val| val).collect()).unwrap();
    
    circuits.push(union_circuit);
    if circuits.len() == 1 {
      break;
    }
  }
  
  let connection = last_connection.unwrap();
  let cable_length = connection.0.0 * connection.1.0;
  
  println!("cable length from wall: {}", cable_length);
}


