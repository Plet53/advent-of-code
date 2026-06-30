use std::fs;
use std::env;

extern crate nalgebra as na;
use na::{DMatrix};
type ButtonMatrix = DMatrix<f32>;

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
    let mut joltage_ratings: Option<Vec<u16>> = None;
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
    // Take every combination, starting from the smallest
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
    // Set up a matrix of the buttons. Each Button as a column (0 for not present, 1 for present)
    // Use Gaussian Elimination
    // Solve as system of equations
    // Sum column of results (Analyze for correctness)
    match joltage_ratings {
      Some(pattern) => {
        let needed_count: usize;
        
        let mut button_matrix: ButtonMatrix = ButtonMatrix::from_fn(
          pattern.len(),
          control_sequences.len() + 1,
          |row, col| {
            if col < control_sequences.len() {
              match control_sequences[col].contains(&row) {
                true => 1.0_f32,
                false => 0.0_f32
              }
            } else {
              pattern[row].try_into().unwrap()
            }
          }
        );
        
        // Tuple indexing is Row then Column
        println!("normal: {button_matrix}");
        reduced_row_echelon_form_in_place(&mut button_matrix);
        println!("echelon: {button_matrix}");
        
        let target: ButtonMatrix = ButtonMatrix::from_iterator(
          pattern.len(),
          1,
          button_matrix.column(control_sequences.len()).iter().map(|r| *r)
        );
        
        button_matrix = button_matrix.remove_column(control_sequences.len());
        
        needed_count = match button_matrix.solve_upper_triangular(&target) {
          Some(vector) => {
            println!("solution: {vector}");
            vector.iter().fold(0.0_f32, |acc, e| acc + *e).round() as usize
          },
          None => 0,
        };
        
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


// Algorithm copied from wikipedia, worked into position.
fn reduced_row_echelon_form_in_place(matrix: &mut ButtonMatrix) {
  let mut pivot = 0;
  let (row_count, column_count) = (
    matrix.row_iter().count(),
    matrix.column_iter().count()
  );
  
  'outer: for r in 0..row_count {
    if column_count <= pivot {
      break;
    }
    let mut i = r;
    while matrix[(i, pivot)] == 0.0 {
      i = i+1;
      if i == row_count {
        i = r;
        pivot += 1;
        if column_count == pivot {
          pivot -= 1;
          break 'outer;
        }
      }
    }
    for k in 0..column_count {
      let temp = matrix[(r, k)];
      matrix[(r, k)] = matrix[(i, k)];
      matrix[(i, k)] = temp;
    }
    let divisor = matrix[(r, pivot)];
    if divisor != 0.0 {
      for k in 0..column_count {
        matrix[(r, k)] /= divisor;
      }
    }
    for j in 0..row_count {
      if j != r {
        let hold = matrix[(j, pivot)];
        for k in 0..column_count {
            matrix[(j, k)] -= hold * matrix[(r, k)];
        }
      }
    }
    pivot = pivot + 1;
  }
}

// Ultimately this did not work, upper echelon form did not land in the ways I needed it to to use the solver library.
// It was ambitious of me to try and learn a lot of linear algebra in a day, but I got to a point where I could build them on paper.
