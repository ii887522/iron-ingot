use iron_ingot::{DVec4, FVec4, IVec4, UVec4};

#[test]
fn test_from_i32() {
  assert_eq!(IVec4::new(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new(1), IVec4::new((1, 1, 1, 1)));
  assert_eq!(IVec4::new(2), IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(IVec4::new((0, 0, 0, 0)).with_x(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)).with_x(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)).with_x(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)).with_x(0), IVec4::new((0, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_x(0), IVec4::new((0, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_x(1), IVec4::new((1, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_x(2), IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)).with_x(2), IVec4::new((2, 2, 1, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)).with_x(2), IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)).with_x(2), IVec4::new((2, 2, 2, 1)));
  assert_eq!(IVec4::new((2, 2, 2, 2)).with_x(2), IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(IVec4::new((0, 0, 0, 0)).with_y(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)).with_y(0), IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)).with_y(0), IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)).with_y(0), IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_y(0), IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_y(1), IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_y(2), IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)).with_y(2), IVec4::new((2, 2, 1, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)).with_y(2), IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)).with_y(2), IVec4::new((2, 2, 2, 1)));
  assert_eq!(IVec4::new((2, 2, 2, 2)).with_y(2), IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_z() {
  assert_eq!(IVec4::new((0, 0, 0, 0)).with_z(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)).with_z(0), IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)).with_z(0), IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)).with_z(0), IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_z(0), IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_z(1), IVec4::new((2, 2, 1, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_z(2), IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)).with_z(2), IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)).with_z(2), IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)).with_z(2), IVec4::new((2, 2, 2, 1)));
  assert_eq!(IVec4::new((2, 2, 2, 2)).with_z(2), IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_w() {
  assert_eq!(IVec4::new((0, 0, 0, 0)).with_w(0), IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)).with_w(0), IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)).with_w(0), IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)).with_w(0), IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_w(0), IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_w(1), IVec4::new((2, 2, 0, 1)));
  assert_eq!(IVec4::new((2, 2, 0, 0)).with_w(2), IVec4::new((2, 2, 0, 2)));
  assert_eq!(IVec4::new((2, 2, 1, 0)).with_w(2), IVec4::new((2, 2, 1, 2)));
  assert_eq!(IVec4::new((2, 2, 2, 0)).with_w(2), IVec4::new((2, 2, 2, 2)));
  assert_eq!(IVec4::new((2, 2, 2, 1)).with_w(2), IVec4::new((2, 2, 2, 2)));
  assert_eq!(IVec4::new((2, 2, 2, 2)).with_w(2), IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) + IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) + IVec4::new((0, 0, 0, 0)),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) + IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) + IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) + IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) + IVec4::new((1, 0, 0, 0)),
    IVec4::new((3, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) + IVec4::new((2, 0, 0, 0)),
    IVec4::new((4, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) + IVec4::new((2, 1, 0, 0)),
    IVec4::new((4, 3, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) + IVec4::new((2, 2, 0, 0)),
    IVec4::new((4, 4, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) + IVec4::new((2, 2, 0, 0)),
    IVec4::new((4, 4, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) + IVec4::new((2, 2, 0, 0)),
    IVec4::new((4, 4, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) + IVec4::new((2, 2, 1, 0)),
    IVec4::new((4, 4, 3, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) + IVec4::new((2, 2, 2, 0)),
    IVec4::new((4, 4, 4, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) + IVec4::new((2, 2, 2, 0)),
    IVec4::new((4, 4, 4, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) + IVec4::new((2, 2, 2, 0)),
    IVec4::new((4, 4, 4, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) + IVec4::new((2, 2, 2, 1)),
    IVec4::new((4, 4, 4, 3))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) + IVec4::new((2, 2, 2, 2)),
    IVec4::new((4, 4, 4, 4))
  );
}

#[test]
fn test_add_assign() {
  let mut vec = IVec4::new(());
  vec += IVec4::new((0, 0, 0, 0));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
  vec += IVec4::new((1, 0, 0, 0));
  assert_eq!(vec, IVec4::new((1, 0, 0, 0)));
  vec += IVec4::new((2, 0, 0, 0));
  assert_eq!(vec, IVec4::new((3, 0, 0, 0)));
  vec += IVec4::new((2, 1, 0, 0));
  assert_eq!(vec, IVec4::new((5, 1, 0, 0)));
  vec += IVec4::new((2, 2, 0, 0));
  assert_eq!(vec, IVec4::new((7, 3, 0, 0)));
  vec += IVec4::new((2, 2, 1, 0));
  assert_eq!(vec, IVec4::new((9, 5, 1, 0)));
  vec += IVec4::new((2, 2, 2, 0));
  assert_eq!(vec, IVec4::new((11, 7, 3, 0)));
  vec += IVec4::new((2, 2, 2, 1));
  assert_eq!(vec, IVec4::new((13, 9, 5, 1)));
  vec += IVec4::new((2, 2, 2, 2));
  assert_eq!(vec, IVec4::new((15, 11, 7, 3)));
}

#[test]
fn test_sub() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) - IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) - IVec4::new((0, 0, 0, 0)),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) - IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) - IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) - IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) - IVec4::new((1, 0, 0, 0)),
    IVec4::new((1, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) - IVec4::new((2, 0, 0, 0)),
    IVec4::new((0, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) - IVec4::new((2, 1, 0, 0)),
    IVec4::new((0, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) - IVec4::new((2, 2, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) - IVec4::new((2, 2, 0, 0)),
    IVec4::new((0, 0, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) - IVec4::new((2, 2, 0, 0)),
    IVec4::new((0, 0, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) - IVec4::new((2, 2, 1, 0)),
    IVec4::new((0, 0, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) - IVec4::new((2, 2, 2, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) - IVec4::new((2, 2, 2, 0)),
    IVec4::new((0, 0, 0, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) - IVec4::new((2, 2, 2, 0)),
    IVec4::new((0, 0, 0, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) - IVec4::new((2, 2, 2, 1)),
    IVec4::new((0, 0, 0, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) - IVec4::new((2, 2, 2, 2)),
    IVec4::new((0, 0, 0, 0))
  );
}

#[test]
fn test_sub_assign() {
  let mut vec = IVec4::new(());
  vec -= IVec4::new((0, 0, 0, 0));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
  vec -= IVec4::new((1, 0, 0, 0));
  assert_eq!(vec, IVec4::new((-1, 0, 0, 0)));
  vec -= IVec4::new((2, 0, 0, 0));
  assert_eq!(vec, IVec4::new((-3, 0, 0, 0)));
  vec -= IVec4::new((2, 1, 0, 0));
  assert_eq!(vec, IVec4::new((-5, -1, 0, 0)));
  vec -= IVec4::new((2, 2, 0, 0));
  assert_eq!(vec, IVec4::new((-7, -3, 0, 0)));
  vec -= IVec4::new((2, 2, 1, 0));
  assert_eq!(vec, IVec4::new((-9, -5, -1, 0)));
  vec -= IVec4::new((2, 2, 2, 0));
  assert_eq!(vec, IVec4::new((-11, -7, -3, 0)));
  vec -= IVec4::new((2, 2, 2, 1));
  assert_eq!(vec, IVec4::new((-13, -9, -5, -1)));
  vec -= IVec4::new((2, 2, 2, 2));
  assert_eq!(vec, IVec4::new((-15, -11, -7, -3)));
}

#[test]
fn test_mul_i32() {
  assert_eq!(IVec4::new((0, 0, 0, 0)) * 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)) * 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)) * 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)) * 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) * 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) * 1, IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) * 2, IVec4::new((4, 4, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)) * 2, IVec4::new((4, 4, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)) * 2, IVec4::new((4, 4, 4, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)) * 2, IVec4::new((4, 4, 4, 2)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) * 2, IVec4::new((4, 4, 4, 4)));
}

#[test]
fn test_mul_i32_assign() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec *= 1;
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec *= 2;
  assert_eq!(vec, IVec4::new((2, 2, 2, 2)));
  vec *= 3;
  assert_eq!(vec, IVec4::new((6, 6, 6, 6)));
}

#[test]
fn test_mul() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((1, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 0, 0, 0)),
    IVec4::new((4, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 1, 0, 0)),
    IVec4::new((4, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 2, 0, 0)),
    IVec4::new((4, 4, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 2, 1, 0)),
    IVec4::new((4, 4, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 2, 2, 0)),
    IVec4::new((4, 4, 4, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 2, 2, 1)),
    IVec4::new((4, 4, 4, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) * IVec4::new((2, 2, 2, 2)),
    IVec4::new((4, 4, 4, 4))
  );
}

#[test]
fn test_mul_assign() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec *= IVec4::new((1, 1, 1, 1));
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec *= IVec4::new((2, 1, 1, 1));
  assert_eq!(vec, IVec4::new((2, 1, 1, 1)));
  vec *= IVec4::new((3, 1, 1, 1));
  assert_eq!(vec, IVec4::new((6, 1, 1, 1)));
  vec *= IVec4::new((3, 2, 1, 1));
  assert_eq!(vec, IVec4::new((18, 2, 1, 1)));
  vec *= IVec4::new((3, 3, 1, 1));
  assert_eq!(vec, IVec4::new((54, 6, 1, 1)));
  vec *= IVec4::new((3, 3, 2, 1));
  assert_eq!(vec, IVec4::new((162, 18, 2, 1)));
  vec *= IVec4::new((3, 3, 3, 1));
  assert_eq!(vec, IVec4::new((486, 54, 6, 1)));
  vec *= IVec4::new((3, 3, 3, 2));
  assert_eq!(vec, IVec4::new((1458, 162, 18, 2)));
  vec *= IVec4::new((3, 3, 3, 3));
  assert_eq!(vec, IVec4::new((4374, 486, 54, 6)));
}

#[test]
fn test_div_i32() {
  assert_eq!(IVec4::new((0, 0, 0, 0)) / 1, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)) / 1, IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)) / 1, IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)) / 1, IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) / 1, IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) / 2, IVec4::new((1, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) / 4, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)) / 4, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)) / 4, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)) / 4, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) / 4, IVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_div_i32_assign() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec /= 1;
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec /= 2;
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
  vec /= 5;
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_div() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((1, 1, 1, 1)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((2, 1, 1, 1)),
    IVec4::new((1, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 1, 1, 1)),
    IVec4::new((0, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 2, 1, 1)),
    IVec4::new((0, 1, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 4, 1, 1)),
    IVec4::new((0, 0, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 4, 2, 1)),
    IVec4::new((0, 0, 1, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 4, 4, 1)),
    IVec4::new((0, 0, 0, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 4, 4, 2)),
    IVec4::new((0, 0, 0, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) / IVec4::new((4, 4, 4, 4)),
    IVec4::new((0, 0, 0, 0))
  );
}

#[test]
fn test_div_assign() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec /= IVec4::new((1, 1, 1, 1));
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec /= IVec4::new((2, 1, 1, 1));
  assert_eq!(vec, IVec4::new((0, 1, 1, 1)));
  vec /= IVec4::new((5, 1, 1, 1));
  assert_eq!(vec, IVec4::new((0, 1, 1, 1)));
  vec /= IVec4::new((5, 2, 1, 1));
  assert_eq!(vec, IVec4::new((0, 0, 1, 1)));
  vec /= IVec4::new((5, 5, 1, 1));
  assert_eq!(vec, IVec4::new((0, 0, 1, 1)));
  vec /= IVec4::new((5, 5, 2, 1));
  assert_eq!(vec, IVec4::new((0, 0, 0, 1)));
  vec /= IVec4::new((5, 5, 5, 1));
  assert_eq!(vec, IVec4::new((0, 0, 0, 1)));
  vec /= IVec4::new((5, 5, 5, 2));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
  vec /= IVec4::new((5, 5, 5, 5));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_set() {
  let mut vec = IVec4::new(());
  vec.set(IVec4::new((0, 0, 0, 0)));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
  vec.set(IVec4::new((1, 0, 0, 0)));
  assert_eq!(vec, IVec4::new((1, 0, 0, 0)));
  vec.set(IVec4::new((2, 0, 0, 0)));
  assert_eq!(vec, IVec4::new((2, 0, 0, 0)));
  vec.set(IVec4::new((2, 1, 0, 0)));
  assert_eq!(vec, IVec4::new((2, 1, 0, 0)));
  vec.set(IVec4::new((2, 2, 0, 0)));
  assert_eq!(vec, IVec4::new((2, 2, 0, 0)));
  vec.set(IVec4::new((2, 2, 1, 0)));
  assert_eq!(vec, IVec4::new((2, 2, 1, 0)));
  vec.set(IVec4::new((2, 2, 2, 0)));
  assert_eq!(vec, IVec4::new((2, 2, 2, 0)));
  vec.set(IVec4::new((2, 2, 2, 1)));
  assert_eq!(vec, IVec4::new((2, 2, 2, 1)));
  vec.set(IVec4::new((2, 2, 2, 2)));
  assert_eq!(vec, IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_bitand_i32() {
  assert_eq!(IVec4::new((0, 0, 0, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) & 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) & 1, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) & 3, IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_bitand_assign_i32() {
  let mut vec = IVec4::new((3, 3, 3, 3));
  vec &= 3;
  assert_eq!(vec, IVec4::new((3, 3, 3, 3)));
  vec &= 1;
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec &= 0;
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_bitand_ivec4() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) & IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) & IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) & IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) & IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) & IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) & IVec4::new((1, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) & IVec4::new((3, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) & IVec4::new((3, 1, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) & IVec4::new((3, 3, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) & IVec4::new((3, 3, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) & IVec4::new((3, 3, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) & IVec4::new((3, 3, 1, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) & IVec4::new((3, 3, 3, 0)),
    IVec4::new((2, 2, 2, 0))
  );
}

#[test]
fn test_bitand_assign_ivec4() {
  let mut vec = IVec4::new((3, 3, 3, 3));
  vec &= IVec4::new((3, 3, 3, 3));
  assert_eq!(vec, IVec4::new((3, 3, 3, 3)));
  vec &= IVec4::new((3, 1, 3, 3));
  assert_eq!(vec, IVec4::new((3, 1, 3, 3)));
  vec &= IVec4::new((3, 0, 3, 3));
  assert_eq!(vec, IVec4::new((3, 0, 3, 3)));
  vec &= IVec4::new((1, 0, 3, 3));
  assert_eq!(vec, IVec4::new((1, 0, 3, 3)));
  vec &= IVec4::new((0, 0, 3, 3));
  assert_eq!(vec, IVec4::new((0, 0, 3, 3)));
  vec &= IVec4::new((0, 0, 1, 3));
  assert_eq!(vec, IVec4::new((0, 0, 1, 3)));
  vec &= IVec4::new((0, 0, 0, 3));
  assert_eq!(vec, IVec4::new((0, 0, 0, 3)));
  vec &= IVec4::new((0, 0, 0, 1));
  assert_eq!(vec, IVec4::new((0, 0, 0, 1)));
  vec &= IVec4::new((0, 0, 0, 0));
  assert_eq!(vec, IVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_shl_i32() {
  assert_eq!(IVec4::new((0, 0, 0, 0)) << 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)) << 0, IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)) << 0, IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)) << 0, IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) << 0, IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)) << 0, IVec4::new((2, 2, 1, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)) << 0, IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)) << 0, IVec4::new((2, 2, 2, 1)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) << 0, IVec4::new((2, 2, 2, 2)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) << 1, IVec4::new((4, 4, 4, 4)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) << 2, IVec4::new((8, 8, 8, 8)));
}

#[test]
fn test_shl_assign_i32() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec <<= 0;
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec <<= 1;
  assert_eq!(vec, IVec4::new((2, 2, 2, 2)));
  vec <<= 2;
  assert_eq!(vec, IVec4::new((8, 8, 8, 8)));
}

#[test]
fn test_shl_ivec4() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((1, 0, 0, 0)),
    IVec4::new((4, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 0, 0, 0)),
    IVec4::new((8, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 1, 0, 0)),
    IVec4::new((8, 4, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 2, 0, 0)),
    IVec4::new((8, 8, 2, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 2, 1, 0)),
    IVec4::new((8, 8, 4, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 2, 2, 0)),
    IVec4::new((8, 8, 8, 2))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 2, 2, 1)),
    IVec4::new((8, 8, 8, 4))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) << IVec4::new((2, 2, 2, 2)),
    IVec4::new((8, 8, 8, 8))
  );
}

#[test]
fn test_shl_assign_ivec4() {
  let mut vec = IVec4::new((1, 1, 1, 1));
  vec <<= IVec4::new((0, 0, 0, 0));
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
  vec <<= IVec4::new((1, 0, 0, 0));
  assert_eq!(vec, IVec4::new((2, 1, 1, 1)));
  vec <<= IVec4::new((2, 0, 0, 0));
  assert_eq!(vec, IVec4::new((8, 1, 1, 1)));
  vec <<= IVec4::new((2, 1, 0, 0));
  assert_eq!(vec, IVec4::new((32, 2, 1, 1)));
  vec <<= IVec4::new((2, 2, 0, 0));
  assert_eq!(vec, IVec4::new((128, 8, 1, 1)));
  vec <<= IVec4::new((2, 2, 1, 0));
  assert_eq!(vec, IVec4::new((512, 32, 2, 1)));
  vec <<= IVec4::new((2, 2, 2, 0));
  assert_eq!(vec, IVec4::new((2048, 128, 8, 1)));
  vec <<= IVec4::new((2, 2, 2, 1));
  assert_eq!(vec, IVec4::new((8192, 512, 32, 2)));
  vec <<= IVec4::new((2, 2, 2, 2));
  assert_eq!(vec, IVec4::new((32768, 2048, 128, 8)));
}

#[test]
fn test_shr_i32() {
  assert_eq!(IVec4::new((0, 0, 0, 0)) >> 0, IVec4::new((0, 0, 0, 0)));
  assert_eq!(IVec4::new((1, 0, 0, 0)) >> 0, IVec4::new((1, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 0, 0, 0)) >> 0, IVec4::new((2, 0, 0, 0)));
  assert_eq!(IVec4::new((2, 1, 0, 0)) >> 0, IVec4::new((2, 1, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 0, 0)) >> 0, IVec4::new((2, 2, 0, 0)));
  assert_eq!(IVec4::new((2, 2, 1, 0)) >> 0, IVec4::new((2, 2, 1, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 0)) >> 0, IVec4::new((2, 2, 2, 0)));
  assert_eq!(IVec4::new((2, 2, 2, 1)) >> 0, IVec4::new((2, 2, 2, 1)));
  assert_eq!(IVec4::new((2, 2, 2, 2)) >> 0, IVec4::new((2, 2, 2, 2)));
  assert_eq!(IVec4::new((4, 4, 4, 4)) >> 1, IVec4::new((2, 2, 2, 2)));
  assert_eq!(IVec4::new((8, 8, 8, 8)) >> 2, IVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_shr_assign_i32() {
  let mut vec = IVec4::new((8, 8, 8, 8));
  vec >>= 0;
  assert_eq!(vec, IVec4::new((8, 8, 8, 8)));
  vec >>= 1;
  assert_eq!(vec, IVec4::new((4, 4, 4, 4)));
  vec >>= 2;
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
}

#[test]
fn test_shr_ivec4() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)) >> IVec4::new((0, 0, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((4, 2, 2, 2)) >> IVec4::new((1, 0, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 2, 2, 2)) >> IVec4::new((2, 0, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 4, 2, 2)) >> IVec4::new((2, 1, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 8, 2, 2)) >> IVec4::new((2, 2, 0, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 8, 4, 2)) >> IVec4::new((2, 2, 1, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 8, 8, 2)) >> IVec4::new((2, 2, 2, 0)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 8, 8, 4)) >> IVec4::new((2, 2, 2, 1)),
    IVec4::new((2, 2, 2, 2))
  );
  assert_eq!(
    IVec4::new((8, 8, 8, 8)) >> IVec4::new((2, 2, 2, 2)),
    IVec4::new((2, 2, 2, 2))
  );
}

#[test]
fn test_shr_assign_ivec4() {
  let mut vec = IVec4::new((32768, 2048, 128, 8));
  vec >>= IVec4::new((0, 0, 0, 0));
  assert_eq!(vec, IVec4::new((32768, 2048, 128, 8)));
  vec >>= IVec4::new((1, 0, 0, 0));
  assert_eq!(vec, IVec4::new((16384, 2048, 128, 8)));
  vec >>= IVec4::new((2, 0, 0, 0));
  assert_eq!(vec, IVec4::new((4096, 2048, 128, 8)));
  vec >>= IVec4::new((2, 1, 0, 0));
  assert_eq!(vec, IVec4::new((1024, 1024, 128, 8)));
  vec >>= IVec4::new((2, 2, 0, 0));
  assert_eq!(vec, IVec4::new((256, 256, 128, 8)));
  vec >>= IVec4::new((2, 2, 1, 0));
  assert_eq!(vec, IVec4::new((64, 64, 64, 8)));
  vec >>= IVec4::new((2, 2, 2, 0));
  assert_eq!(vec, IVec4::new((16, 16, 16, 8)));
  vec >>= IVec4::new((2, 2, 2, 1));
  assert_eq!(vec, IVec4::new((4, 4, 4, 4)));
  vec >>= IVec4::new((2, 2, 2, 2));
  assert_eq!(vec, IVec4::new((1, 1, 1, 1)));
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)).get_x_fliped(),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)).get_x_fliped(),
    IVec4::new((-1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)).get_x_fliped(),
    IVec4::new((-2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)).get_x_fliped(),
    IVec4::new((-2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)).get_x_fliped(),
    IVec4::new((-2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)).get_x_fliped(),
    IVec4::new((-2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)).get_x_fliped(),
    IVec4::new((-2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)).get_x_fliped(),
    IVec4::new((-2, 2, 2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)).get_x_fliped(),
    IVec4::new((-2, 2, 2, 2))
  );
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)).get_y_fliped(),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)).get_y_fliped(),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)).get_y_fliped(),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)).get_y_fliped(),
    IVec4::new((2, -1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)).get_y_fliped(),
    IVec4::new((2, -2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)).get_y_fliped(),
    IVec4::new((2, -2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)).get_y_fliped(),
    IVec4::new((2, -2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)).get_y_fliped(),
    IVec4::new((2, -2, 2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)).get_y_fliped(),
    IVec4::new((2, -2, 2, 2))
  );
}

#[test]
fn test_get_z_fliped() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)).get_z_fliped(),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)).get_z_fliped(),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)).get_z_fliped(),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)).get_z_fliped(),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)).get_z_fliped(),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)).get_z_fliped(),
    IVec4::new((2, 2, -1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)).get_z_fliped(),
    IVec4::new((2, 2, -2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)).get_z_fliped(),
    IVec4::new((2, 2, -2, 1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)).get_z_fliped(),
    IVec4::new((2, 2, -2, 2))
  );
}

#[test]
fn test_get_w_fliped() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)).get_w_fliped(),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)).get_w_fliped(),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)).get_w_fliped(),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)).get_w_fliped(),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)).get_w_fliped(),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)).get_w_fliped(),
    IVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)).get_w_fliped(),
    IVec4::new((2, 2, 2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)).get_w_fliped(),
    IVec4::new((2, 2, 2, -1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)).get_w_fliped(),
    IVec4::new((2, 2, 2, -2))
  );
}

#[test]
fn test_get_fliped() {
  assert_eq!(
    IVec4::new((0, 0, 0, 0)).get_fliped(),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((1, 0, 0, 0)).get_fliped(),
    IVec4::new((-1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 0, 0, 0)).get_fliped(),
    IVec4::new((-2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 1, 0, 0)).get_fliped(),
    IVec4::new((-2, -1, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 0, 0)).get_fliped(),
    IVec4::new((-2, -2, 0, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 1, 0)).get_fliped(),
    IVec4::new((-2, -2, -1, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 0)).get_fliped(),
    IVec4::new((-2, -2, -2, 0))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 1)).get_fliped(),
    IVec4::new((-2, -2, -2, -1))
  );
  assert_eq!(
    IVec4::new((2, 2, 2, 2)).get_fliped(),
    IVec4::new((-2, -2, -2, -2))
  );
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", IVec4::new((0, 0, 0, 0))), "0,0,0,0");
  assert_eq!(format!("{}", IVec4::new((1, 0, 0, 0))), "1,0,0,0");
  assert_eq!(format!("{}", IVec4::new((2, 0, 0, 0))), "2,0,0,0");
  assert_eq!(format!("{}", IVec4::new((2, 1, 0, 0))), "2,1,0,0");
  assert_eq!(format!("{}", IVec4::new((2, 2, 0, 0))), "2,2,0,0");
  assert_eq!(format!("{}", IVec4::new((2, 2, 1, 0))), "2,2,1,0");
  assert_eq!(format!("{}", IVec4::new((2, 2, 2, 0))), "2,2,2,0");
  assert_eq!(format!("{}", IVec4::new((2, 2, 2, 1))), "2,2,2,1");
  assert_eq!(format!("{}", IVec4::new((2, 2, 2, 2))), "2,2,2,2");
}

#[test]
fn test_into_dvec4() {
  assert_eq!(
    DVec4::from(IVec4::new((0, 0, 0, 0))),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((1, 0, 0, 0))),
    DVec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((2, 0, 0, 0))),
    DVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((2, 1, 0, 0))),
    DVec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((2, 2, 0, 0))),
    DVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((2, 2, 1, 0))),
    DVec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec4::from(IVec4::new((2, 2, 2, 0))),
    DVec4::new((2.0, 2.0, 2.0, 0.0))
  );
}

#[test]
fn test_into_fvec4() {
  assert_eq!(
    FVec4::from(IVec4::new((0, 0, 0, 0))),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((1, 0, 0, 0))),
    FVec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((2, 0, 0, 0))),
    FVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((2, 1, 0, 0))),
    FVec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((2, 2, 0, 0))),
    FVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((2, 2, 1, 0))),
    FVec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    FVec4::from(IVec4::new((2, 2, 2, 0))),
    FVec4::new((2.0, 2.0, 2.0, 0.0))
  );
}

#[test]
fn test_into_uvec4() {
  assert_eq!(
    UVec4::from(IVec4::new((0, 0, 0, 0))),
    UVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((1, 0, 0, 0))),
    UVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((2, 0, 0, 0))),
    UVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((2, 1, 0, 0))),
    UVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((2, 2, 0, 0))),
    UVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((2, 2, 1, 0))),
    UVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    UVec4::from(IVec4::new((2, 2, 2, 0))),
    UVec4::new((2, 2, 2, 0))
  );
}
