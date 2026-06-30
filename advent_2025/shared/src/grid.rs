use core::fmt;
use std::fmt::Display;
use crate::coordinates::Coordinate;

pub struct Grid<T> {
  data: Vec<Vec<T>>,
  height: usize,
  width: usize
}

pub struct CoordIter {
  height: isize,
  width: isize,
  curr: Option<Coordinate>
}

impl CoordIter {
  pub fn new(height: isize, width: isize) -> CoordIter {
    CoordIter { height, width, curr: None }
  }
}

impl Iterator for CoordIter {
  type Item = Coordinate;
  fn next(&mut self) -> Option<Self::Item> {
    self.curr = match self.curr {
      None => Some(Coordinate::new(0, 0)),
      Some(coord) => {
        if coord.x + 1 == self.width {
          if coord.y + 1 == self.height {
            None
          } else {
            Some(Coordinate::new(0, coord.y + 1))
          }
        } else {
          Some(coord + Coordinate::RIGHT)
        }
      }
    };
    self.curr
  }
}

pub struct ColumnIter<'a, T> {
  grid: &'a Grid<T>,
  curr_index: isize,
  curr: Option<Vec<&'a T>>,
}

impl<'a, T> ColumnIter<'a, T> {
  fn new(grid: &'a Grid<T>) -> ColumnIter<'a, T> {
    ColumnIter { grid, curr_index: 0, curr: None }
  }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
  type Item = Vec<&'a T>;
  fn next(&mut self) -> Option<Self::Item> {
    self.curr = match self.curr {
      None => Some((0..self.grid.height as isize).map(|index| self.grid.get(&Coordinate::new(self.curr_index, index)).unwrap()).collect()),
      Some(_) => {
        self.curr_index += 1;
        if self.curr_index >= self.grid.width as isize {
          None
        } else {
          Some((0..self.grid.height as isize).map(|index| self.grid.get(&Coordinate::new(self.curr_index, index)).unwrap()).collect())
        }
      },
    };
    
    self.curr.clone()
  }
}

impl<T> Grid<T> {
  pub fn new(height: usize, width: usize, fill: T) -> Self
  where T: Clone {
    let data = vec![vec![fill; width]; height];
    Self {data, height, width}
  }
  
  pub fn data(&self) -> &Vec<Vec<T>> {
    &self.data
  }
  
  pub fn height(&self) -> usize {
    self.height
  }
  
  pub fn width(&self) -> usize {
    self.width
  }
  
  pub fn in_bounds(&self, coordinate: &Coordinate) -> bool {
    coordinate.x.clamp(0, self.width as isize - 1) == coordinate.x
    && coordinate.y.clamp(0, self.height as isize - 1) == coordinate.y
  }
  
  pub fn get(&self, coordinate: &Coordinate) -> Option<&T> {
    if self.in_bounds(coordinate) {
      Some(&self.data[coordinate.y as usize][coordinate.x as usize])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut T> {
    if self.in_bounds(coordinate) {
      Some(&mut self.data[coordinate.y as usize][coordinate.x as usize])
    } else {
      None
    }
  }

  pub fn set(&mut self, coordinate: &Coordinate, value: T) -> bool {
    if self.in_bounds(coordinate) {
      self.data[coordinate.y as usize][coordinate.x as usize] = value;
    }
    self.in_bounds(coordinate)
  }

  pub fn index_to_coord(&self, index: isize) -> Coordinate {
    Coordinate::index_to_coord(index, self.width as isize)
  }

  pub fn coord_iter(&self) -> CoordIter {
    CoordIter::new(self.height as isize, self.width as isize)
  }
  
  pub fn column_iter(&self) -> ColumnIter<'_, T> {
    ColumnIter::new(self)
  }
  
  pub fn occurance_locations(&self, target: T) -> Vec<Coordinate> 
  where T: Eq {
    self.coord_iter().filter(|coord| {
      match self.get(coord) {
        Some(val) => *val == target,
        None => false,
      }
    }).collect()
  }

  pub fn count_occurances(&self, target: T) -> u64
  where T: Eq {
    self.data.iter().fold(0, |acc, val| {
      acc + (val.iter().filter(|e| **e == target).count() as u64)
    })
  }
  
  pub fn find(&self, target: T) -> Option<Coordinate>
  where T: Eq {
    self.coord_iter().find(|coord| {
      *(self.get(&coord).unwrap()) == target
    })
  }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
  fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
    let mut data = Vec::new();
    for item in iter {
      data.push(item);
    }
    let height = data.len();
    let width = data[0].len();
    Self {data, height, width}
  }
}

impl<T> fmt::Display for Grid<T>
where T: Display {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.data.iter().map(|line| {
      let result = line.iter().map(|item| {
        write!(f, "{}", item)
      }).reduce(|acc, result| {
        match (acc, result) {
          (Err(e), _) => Err(e),
          (_, Ok(_)) => Ok(()),
          (_, Err(e)) => Err(e),
        }
      }).unwrap();
      match result {
        Ok(_) => write!(f, "\n"),
        Err(e) => Err(e)
      }
    }).reduce(|acc, result| {
      match (acc, result) {
        (Err(e), _) => Err(e),
        (_, Ok(_)) => Ok(()),
        (_, Err(e)) => Err(e),
      }
    }).unwrap()
  }
}
