use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
};

use crate::{DVec3, FVec3, IVec3};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: u32,
  y: u32,
  z: u32,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self { x: 0, y: 0, z: 0 }
  }
}

impl From<u32> for Arg {
  fn from(value: u32) -> Self {
    Self {
      x: value,
      y: value,
      z: value,
    }
  }
}

impl From<(u32, u32, u32)> for Arg {
  fn from((x, y, z): (u32, u32, u32)) -> Self {
    Self { x, y, z }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UVec3 {
  x: u32,
  y: u32,
  z: u32,
}

impl UVec3 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z } = arg.into();
    Self { x, y, z }
  }

  pub const fn get_x(&self) -> u32 {
    self.x
  }

  pub fn set_x(&mut self, value: u32) {
    self.x = value;
  }

  pub fn with_x(&self, value: u32) -> Self {
    Self::new((value, self.y, self.z))
  }

  pub const fn get_y(&self) -> u32 {
    self.y
  }

  pub fn set_y(&mut self, value: u32) {
    self.y = value;
  }

  pub fn with_y(&self, value: u32) -> Self {
    Self::new((self.x, value, self.z))
  }

  pub const fn get_z(&self) -> u32 {
    self.z
  }

  pub fn set_z(&mut self, value: u32) {
    self.z = value;
  }

  pub fn with_z(&self, value: u32) -> Self {
    Self::new((self.x, self.y, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
  }
}

impl Add for UVec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y, self.z + other.z))
  }
}

impl AddAssign for UVec3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Sub for UVec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y, self.z - other.z))
  }
}

impl SubAssign for UVec3 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl Mul<u32> for UVec3 {
  type Output = Self;

  fn mul(self, value: u32) -> Self::Output {
    Self::new((self.x * value, self.y * value, self.z * value))
  }
}

impl MulAssign<u32> for UVec3 {
  fn mul_assign(&mut self, value: u32) {
    self.x *= value;
    self.y *= value;
    self.z *= value;
  }
}

impl Mul for UVec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self::new((self.x * other.x, self.y * other.y, self.z * other.z))
  }
}

impl MulAssign for UVec3 {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }
}

impl Div<u32> for UVec3 {
  type Output = Self;

  fn div(self, value: u32) -> Self::Output {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value, self.z / value))
  }
}

impl DivAssign<u32> for UVec3 {
  fn div_assign(&mut self, value: u32) {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= value;
    self.y /= value;
    self.z /= value;
  }
}

impl Div for UVec3 {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    debug_assert_ne!(
      other.x, 0,
      "other.x must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0,
      "other.y must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.z, 0,
      "other.z must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / other.x, self.y / other.y, self.z / other.z))
  }
}

impl DivAssign for UVec3 {
  fn div_assign(&mut self, other: Self) {
    debug_assert_ne!(
      other.x, 0,
      "other.x must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0,
      "other.y must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.z, 0,
      "other.z must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }
}

impl BitAnd<u32> for UVec3 {
  type Output = UVec3;

  fn bitand(self, value: u32) -> Self::Output {
    Self::new((self.x & value, self.y & value, self.z & value))
  }
}

impl BitAndAssign<u32> for UVec3 {
  fn bitand_assign(&mut self, value: u32) {
    self.x &= value;
    self.y &= value;
    self.z &= value;
  }
}

impl BitAnd for UVec3 {
  type Output = UVec3;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((self.x & other.x, self.y & other.y, self.z & other.z))
  }
}

impl BitAndAssign for UVec3 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
    self.z &= other.z;
  }
}

impl Shl<u32> for UVec3 {
  type Output = UVec3;

  fn shl(self, value: u32) -> Self::Output {
    Self::new((self.x << value, self.y << value, self.z << value))
  }
}

impl ShlAssign<u32> for UVec3 {
  fn shl_assign(&mut self, value: u32) {
    self.x <<= value;
    self.y <<= value;
    self.z <<= value;
  }
}

impl Shl for UVec3 {
  type Output = UVec3;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((self.x << other.x, self.y << other.y, self.z << other.z))
  }
}

impl ShlAssign for UVec3 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
    self.z <<= other.z;
  }
}

impl Shr<u32> for UVec3 {
  type Output = UVec3;

  fn shr(self, value: u32) -> Self::Output {
    Self::new((self.x >> value, self.y >> value, self.z >> value))
  }
}

impl ShrAssign<u32> for UVec3 {
  fn shr_assign(&mut self, value: u32) {
    self.x >>= value;
    self.y >>= value;
    self.z >>= value;
  }
}

impl Shr for UVec3 {
  type Output = UVec3;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((self.x >> other.x, self.y >> other.y, self.z >> other.z))
  }
}

impl ShrAssign for UVec3 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
    self.z >>= other.z;
  }
}

impl Display for UVec3 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl From<DVec3> for UVec3 {
  fn from(vec: DVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<FVec3> for UVec3 {
  fn from(vec: FVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<IVec3> for UVec3 {
  fn from(vec: IVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}
