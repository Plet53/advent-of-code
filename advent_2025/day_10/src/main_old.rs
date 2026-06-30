use std::fs;
use std::env;

extern crate nalgebra as na;
use na::{Matrix, Dynamic};

struct CombinerIter<'a, T> {
  curr: bool,
  state: Vec<usize>,
  data: &'a Vec<T>
}

impl<'a, T> CombinerIter<'a, T> {
  pub fn new(data: &'a Vec<T>, size: usize) -> CombinerIter<'a, T> {
    CombinerIter {curr: false, state: vec![0_usize; size], data}
  }
}

impl<'a, T> Iterator for CombinerIter<'a, T> {
  type Item = Vec<&'a T>;
  fn next(&mut self) -> Option<Self::Item> {
    self.curr = match self.curr {
      false => {
        true
      },
      true => {
        for i in 0..self.state.len() {
          self.state[i] += 1;
          if self.state[i] < self.data.len() {
            break;
          }
          self.state[i] = 0;
        }
        
        if self.state.iter().all(|index| *index == 0) {
          false
        } else {
          true
        }
      }
    };
    
    if self.curr {
      Some(self.state.iter().map(|index| &self.data[*index]).collect())
    } else {
      None
    }
  }
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
  
  let (toggle_counts, power_counts): (Vec<usize>, Vec<usize>) = rawdata.lines().map(|line| {
    let substrings = line.split(' ');
    let mut light_pattern: Option<Vec<u8>> = None;
    let mut joltage_ratings: Option<Vec<usize>> = None;
    let mut control_sequences: Vec<Vec<usize>> = Vec::new();
    for substring in substrings {
      match substring.chars().nth(0) {
        Some('[') => {
          light_pattern = Some(substring[1..(substring.len() - 1)].chars().map(|glyph| (glyph == '#') as u8).collect());
        },
        Some('(') => {
          control_sequences.push(substring[1..(substring.len() - 1)].split(',').map(|num| num.parse().unwrap()).collect());
        },
        Some('{') => {
          joltage_ratings = Some(substring[1..(substring.len() - 1)].split(',').map(|num| num.parse().unwrap()).collect());
        },
        None => (),
        _ => panic!("malformed data"),
      }
    }
    
    // Part 1
    // Take every combination, starting from the smallestt
    // The lowest combo length that matches is the result
    (match light_pattern {
      Some(pattern) => {
        let needed_count = (1..=control_sequences.len()).find(|count| {
            for combo in CombinerIter::new(&control_sequences, *count) {
              let index_count: Vec<u8> = combo.iter().fold(vec![0_u8; pattern.len()], |mut acc, values| {
                for value in values.iter() {
                  acc[*value] += 1;
                }
                
                acc
              });
              
              if index_count.iter().zip(pattern.iter()).all(|(num_switches, final_state)| {
                (num_switches % 2) == *final_state
              }) {
                // println!("{combo:?}");
                return true;
              }
            }
            
            false
          }
        );
        
        match needed_count {
          Some(count) => count,
          None => panic!("light pattern cannot be matched with simple pattern"),
        }
      },
      None => 0,
    },
    
    // Part 2
    // Set up a matrix of the buttons. Each Button as a row (0 for not present, 1 for present)
    // Find inverse.
    // Multiply by column matrix of Joltage Ratings
    // Sum column of results (Analyze for correctness)
    match joltage_ratings {
      Some(pattern) => {
        let mut needed_count = 0;
        let mut current_pattern = vec![0_usize; pattern.len()];
        let mut bad_start_vec_indices: Vec<usize> = Vec::new();
        let mut vec_start_index: Option<usize> = None;
        while current_pattern.iter().zip(pattern.iter()).any(|(current, target)| {
                  *current != *target
        }) {
          loop {
            let curr_index = (0..pattern.len()).filter(|index| {
              pattern[*index] != current_pattern[*index]
            }).min_by(|a, b| {
              (pattern[*a] - current_pattern[*a]).cmp(&(pattern[*b] - current_pattern[*b]))
            }).unwrap();
            
            let count = pattern[curr_index] - current_pattern[curr_index];
            
            let satisfied: Vec<usize> = (0..pattern.len()).filter(|index| {
              pattern[*index] == current_pattern[*index]
            }).collect();
            
            let sequence_index = (0..control_sequences.len()).filter(|vec_index| {
              if vec_start_index.is_none() && bad_start_vec_indices.contains(vec_index) {
                return false;
              }
              control_sequences[*vec_index].contains(&curr_index) && control_sequences[*vec_index].iter().all(|index| {
                !satisfied.contains(index) 
              })
            }).max_by(|index_a, index_b| {
              control_sequences[*index_a].len().cmp(&(control_sequences[*index_b].len()))
            });
            
            match sequence_index {
              Some(index) => {
                if vec_start_index.is_none() {
                  vec_start_index = Some(index)
                }
                for seq_index in control_sequences[index].iter() {
                  current_pattern[*seq_index] += count;
                }
                
                println!("{current_pattern:?}");
                needed_count += count;
                
                if current_pattern == pattern {
                  break;          
                }
              },
              None => {
                println!();
                bad_start_vec_indices.push(vec_start_index.unwrap());
                vec_start_index = None;
                current_pattern = vec![0_usize; pattern.len()];
                needed_count = 0;
              }
            }
          }
        }
        
        println!("{needed_count}");
        needed_count
      },
      None => 0,
    })
  }).unzip();
  
  let (toggle_sol, power_sol) = (
    toggle_counts.iter().fold(0_usize, |acc, e| acc + *e),
    power_counts.iter().fold(0_usize, |acc, e| acc + *e)
  );
  
  println!("part_1: {toggle_sol}\npart_2: {power_sol}");
  
}
