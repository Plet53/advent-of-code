// Count all paths through a network graph
// At it's simplest, an input node that connects to 2 intermediares and then an output node has 2 paths.
// Each possible branch holds a new set of paths.

use std::fs;
use std::env;
use std::collections::HashMap;

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

  // Pile of comparison to do, so let's simplify the data. As the complexity is exponential, microseconds count.
  let relevant_indices: [u32; 4] = [
    u32::from_str_radix("svr", 36).unwrap(),
    u32::from_str_radix("fft", 36).unwrap(),
    u32::from_str_radix("dac", 36).unwrap(),
    u32::from_str_radix("out", 36).unwrap()
  ];
  
  println!("{relevant_indices:?}");

  // input: text lines of 3 alpha characters, separated by a colon. before, the node in question. after, the nodes that it is connected to
  // output: hashmap of that relationship as u32s.
  // 'out' leads to nowhere. 
  let data_graph: HashMap<u32, Vec<u32>> = rawdata.lines().fold(HashMap::from([(relevant_indices[3], Vec::new())]), |mut map, line| {
    let mut line_indices: Vec<u32> = line.split_whitespace().map(|s| {
      u32::from_str_radix(&s[..3], 36).unwrap()
    }).rev().collect();

    let index = line_indices.pop().expect("empty line in mapping");
    
    map.insert(index, line_indices);
    
    map
  });
  
  // The total pathing is to go from "svr", through both "dac" and "fft", and to "out". 
  // Map each segment individually, and then add the results at the end.
  let (mut svr_to_fft, mut svr_to_dac): (HashMap<u32, u128>, HashMap<u32, u128>) = (
    HashMap::from([(relevant_indices[0], 1), (relevant_indices[1], 0)]),
    HashMap::from([(relevant_indices[0], 1), (relevant_indices[2], 0)])
  );
  
  recurse_search(&data_graph, &mut svr_to_fft, vec![relevant_indices[0]], &relevant_indices[1]);
  recurse_search(&data_graph, &mut svr_to_dac, vec![relevant_indices[0]], &relevant_indices[2]);
  
  println!("{}, {}",
    svr_to_fft[&relevant_indices[1]], svr_to_dac[&relevant_indices[2]]
  );

  let (mut fft_to_dac, mut dac_to_fft): (HashMap<u32, u128>, HashMap<u32, u128>) = (
    HashMap::from([(relevant_indices[1], svr_to_fft[&relevant_indices[1]]), (relevant_indices[2], 0)]),
    HashMap::from([(relevant_indices[2], svr_to_dac[&relevant_indices[2]]), (relevant_indices[1], 0)])
  );
  
  recurse_search(&data_graph, &mut fft_to_dac, vec![relevant_indices[1]], &relevant_indices[2]);
  recurse_search(&data_graph, &mut dac_to_fft, vec![relevant_indices[2]], &relevant_indices[1]);
  
  println!("{}, {}",
    fft_to_dac[&relevant_indices[2]], dac_to_fft[&relevant_indices[1]]
  );
  
  let (mut fft_to_out, mut dac_to_out): (HashMap<u32, u128>, HashMap<u32, u128>) = (
    HashMap::from([(relevant_indices[1], dac_to_fft[&relevant_indices[1]]), (relevant_indices[3], 0)]),
    HashMap::from([(relevant_indices[2], fft_to_dac[&relevant_indices[2]]), (relevant_indices[3], 0)])
  );
  
  recurse_search(&data_graph, &mut fft_to_out, vec![relevant_indices[1]], &relevant_indices[3]);
  recurse_search(&data_graph, &mut dac_to_out, vec![relevant_indices[2]], &relevant_indices[3]);
  
  println!("{}, {}",
    fft_to_out[&relevant_indices[3]], dac_to_out[&relevant_indices[3]]
  );

  let final_count: u128 = fft_to_out[&relevant_indices[3]] + dac_to_out[&relevant_indices[3]];
  
  println!("final_count: {final_count}");
}

// Search through `map`, marking the distance from point to point in `path_count_map`. Search the current `layer` towards `target`.
fn recurse_search(map: &HashMap<u32, Vec<u32>>, path_count_map: &mut HashMap<u32, u128>, layer: Vec<u32>, target: &u32) {
  // The number of paths for each element in `layer`
  let current_layer: HashMap<u32, u128> = layer.iter().fold(HashMap::new(), |mut map, curr| {
    map.insert(*curr, path_count_map[curr]);
    
    map
  });
  
  let mut next_layer: Vec<u32> = Vec::new();
  for (curr, count) in current_layer { // rust decomposition my beloved
    for next in map[&curr].
      // the number of paths from a point to a new point is the sum of the numbers of paths up to this point.
      // see Entry API on hashmaps for more detail
      *(path_count_map.entry(*next).or_insert(0)) += count;
      // don't continue past the target, through the target, and don't re-add the next to the next layer.
      if !map[target].contains(next) && *next != *target && !next_layer.contains(next) {
        next_layer.push(*next);
      }
    }
  }

  // recurse through next layer until we've hit the target
  if next_layer.len() > 0 {
    recurse_search(map, path_count_map, next_layer, target);
  }
}
