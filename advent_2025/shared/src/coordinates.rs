// Cartesian Library. Positive Directions are Right and Down.
use std::{cmp::Ordering, ops::{Add, AddAssign, Mul, Rem, Sub, SubAssign}, fmt::Display};

pub const CARDINALS: [Coordinate; 4] = [
  Coordinate::UP,
  Coordinate::RIGHT,
  Coordinate::DOWN,
  Coordinate::LEFT
];

pub const DIRECTIONS: [Coordinate; 8] = [
  Coordinate::UP,
  Coordinate::UP_RIGHT,
  Coordinate::RIGHT,
  Coordinate::DOWN_RIGHT,
  Coordinate::DOWN,
  Coordinate::DOWN_LEFT,
  Coordinate::LEFT,
  Coordinate::UP_LEFT
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
  pub x: isize,
  pub y: isize
}

impl Coordinate {
  pub const UP: Coordinate = Coordinate::new(0, -1);
  pub const RIGHT: Coordinate = Coordinate::new(1, 0);
  pub const DOWN: Coordinate = Coordinate::new(0, 1);
  pub const LEFT: Coordinate = Coordinate::new(-1, 0);
  pub const UP_LEFT: Coordinate = Coordinate::new(-1, -1);
  pub const UP_RIGHT: Coordinate = Coordinate::new(1, -1);
  pub const DOWN_RIGHT: Coordinate = Coordinate::new(1, 1);
  pub const DOWN_LEFT: Coordinate = Coordinate::new(-1, 1);

  pub const fn new(x: isize, y: isize) -> Coordinate {
    Coordinate {x, y}
  }

  pub fn index_to_coord(index: isize, width: isize) -> Coordinate {
    Coordinate {x: index % width, y: index / width}
  }

  pub fn direction_of(self) -> Coordinate {
    Coordinate::new(self.x.signum(), self.y.signum())
  }
}

impl Add<Self> for Coordinate {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::Output {x: self.x + other.x, y: self.y + other.y}
  }
}

impl Add<&Self> for Coordinate {
  type Output = Self;
  fn add(self, other: &Self) -> Self::Output {
    Self::Output {x: self.x + other.x, y: self.y + other.y}
  }
}

impl AddAssign<Self> for Coordinate {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl AddAssign<&Self> for Coordinate {
  fn add_assign(&mut self, other: &Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl Sub<Self> for Coordinate {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::Output {x: self.x - other.x, y: self.y - other.y}
  }
}

impl Sub<&Self> for Coordinate {
  type Output = Self;
  fn sub(self, other: &Self) -> Self::Output {
    Self::Output {x: self.x - other.x, y: self.y - other.y}
  }
}

impl SubAssign<Self> for Coordinate {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl SubAssign<&Self> for Coordinate {
  fn sub_assign(&mut self, other: &Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl Mul<isize> for Coordinate {
  type Output = Self;
  fn mul(self, other: isize) -> Self::Output {
    Self::Output {x: self.x * other, y: self.y * other}
  }
}

impl Rem<Self> for Coordinate {
  type Output = Self;
  fn rem(self, rhs: Self) -> Self::Output {
    Self::Output {x: self.x % rhs.x, y: self.y % rhs.y}
  }
}

impl Rem<&Self> for Coordinate {
  type Output = Self;
  fn rem(self, rhs: &Self) -> Self::Output {
    Self::Output {x: self.x % rhs.x, y: self.y % rhs.y}
  }
}

impl Ord for Coordinate {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.x.cmp(&other.x) {
      Ordering::Equal => {
        self.y.cmp(&other.y)
      },
      ord => ord,
    }
  }
}

impl PartialOrd for Coordinate {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Display for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
