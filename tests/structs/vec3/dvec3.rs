use iron_ingot::{math::ApproxEq, DVec3, FVec3, IVec3, UVec3};

#[test]
fn test_from_f64() {
  assert_eq!(DVec3::new(0.0), DVec3::new((0.0, 0.0, 0.0)));
  assert_eq!(DVec3::new(1.0), DVec3::new((1.0, 1.0, 1.0)));
  assert_eq!(DVec3::new(2.0), DVec3::new((2.0, 2.0, 2.0)));
}

#[test]
fn test_with_x() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).with_x(0.0),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).with_x(0.0),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).with_x(0.0),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).with_x(0.0),
    DVec3::new((0.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_x(0.0),
    DVec3::new((0.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_x(1.0),
    DVec3::new((1.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_x(2.0),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).with_x(2.0),
    DVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).with_x(2.0),
    DVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_y() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).with_y(0.0),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).with_y(0.0),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).with_y(0.0),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).with_y(0.0),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_y(0.0),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_y(1.0),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_y(2.0),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).with_y(2.0),
    DVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).with_y(2.0),
    DVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_z() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).with_z(0.0),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).with_z(0.0),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).with_z(0.0),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).with_z(0.0),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_z(0.0),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_z(1.0),
    DVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).with_z(2.0),
    DVec3::new((2.0, 2.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).with_z(2.0),
    DVec3::new((2.0, 2.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).with_z(2.0),
    DVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_add() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) + DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) + DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) + DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) + DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) + DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) + DVec3::new((1.0, 0.0, 0.0)),
    DVec3::new((3.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) + DVec3::new((2.0, 0.0, 0.0)),
    DVec3::new((4.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) + DVec3::new((2.0, 1.0, 0.0)),
    DVec3::new((4.0, 3.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) + DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((4.0, 4.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) + DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((4.0, 4.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) + DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((4.0, 4.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) + DVec3::new((2.0, 2.0, 1.0)),
    DVec3::new((4.0, 4.0, 3.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) + DVec3::new((2.0, 2.0, 2.0)),
    DVec3::new((4.0, 4.0, 4.0))
  );
}

#[test]
fn test_add_assign() {
  let mut vec = DVec3::new(());
  vec += DVec3::new((0.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((0.0, 0.0, 0.0)));
  vec += DVec3::new((1.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((1.0, 0.0, 0.0)));
  vec += DVec3::new((2.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((3.0, 0.0, 0.0)));
  vec += DVec3::new((2.0, 1.0, 0.0));
  assert_eq!(vec, DVec3::new((5.0, 1.0, 0.0)));
  vec += DVec3::new((2.0, 2.0, 0.0));
  assert_eq!(vec, DVec3::new((7.0, 3.0, 0.0)));
  vec += DVec3::new((2.0, 2.0, 1.0));
  assert_eq!(vec, DVec3::new((9.0, 5.0, 1.0)));
  vec += DVec3::new((2.0, 2.0, 2.0));
  assert_eq!(vec, DVec3::new((11.0, 7.0, 3.0)));
}

#[test]
fn test_sub() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) - DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) - DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) - DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) - DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) - DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) - DVec3::new((1.0, 0.0, 0.0)),
    DVec3::new((1.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) - DVec3::new((2.0, 0.0, 0.0)),
    DVec3::new((0.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) - DVec3::new((2.0, 1.0, 0.0)),
    DVec3::new((0.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) - DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) - DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((0.0, 0.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) - DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((0.0, 0.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) - DVec3::new((2.0, 2.0, 1.0)),
    DVec3::new((0.0, 0.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) - DVec3::new((2.0, 2.0, 2.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
}

#[test]
fn test_sub_assign() {
  let mut vec = DVec3::new(());
  vec -= DVec3::new((0.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((0.0, 0.0, 0.0)));
  vec -= DVec3::new((1.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((-1.0, 0.0, 0.0)));
  vec -= DVec3::new((2.0, 0.0, 0.0));
  assert_eq!(vec, DVec3::new((-3.0, 0.0, 0.0)));
  vec -= DVec3::new((2.0, 1.0, 0.0));
  assert_eq!(vec, DVec3::new((-5.0, -1.0, 0.0)));
  vec -= DVec3::new((2.0, 2.0, 0.0));
  assert_eq!(vec, DVec3::new((-7.0, -3.0, 0.0)));
  vec -= DVec3::new((2.0, 2.0, 1.0));
  assert_eq!(vec, DVec3::new((-9.0, -5.0, -1.0)));
  vec -= DVec3::new((2.0, 2.0, 2.0));
  assert_eq!(vec, DVec3::new((-11.0, -7.0, -3.0)));
}

#[test]
fn test_mul_f64() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) * 0.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) * 0.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) * 0.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) * 0.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) * 0.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) * 1.0,
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) * 2.0,
    DVec3::new((4.0, 4.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) * 2.0,
    DVec3::new((4.0, 4.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * 2.0,
    DVec3::new((4.0, 4.0, 4.0))
  );
}

#[test]
fn test_mul_f64_assign() {
  let mut vec = DVec3::new((1.0, 1.0, 1.0));
  vec *= 1.0;
  assert_eq!(vec, DVec3::new((1.0, 1.0, 1.0)));
  vec *= 2.0;
  assert_eq!(vec, DVec3::new((2.0, 2.0, 2.0)));
  vec *= 3.0;
  assert_eq!(vec, DVec3::new((6.0, 6.0, 6.0)));
}

#[test]
fn test_mul() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((0.0, 0.0, 0.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((1.0, 0.0, 0.0)),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((2.0, 0.0, 0.0)),
    DVec3::new((4.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((2.0, 1.0, 0.0)),
    DVec3::new((4.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((2.0, 2.0, 0.0)),
    DVec3::new((4.0, 4.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((2.0, 2.0, 1.0)),
    DVec3::new((4.0, 4.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) * DVec3::new((2.0, 2.0, 2.0)),
    DVec3::new((4.0, 4.0, 4.0))
  );
}

#[test]
fn test_mul_assign() {
  let mut vec = DVec3::new((1.0, 1.0, 1.0));
  vec *= DVec3::new((1.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((1.0, 1.0, 1.0)));
  vec *= DVec3::new((2.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((2.0, 1.0, 1.0)));
  vec *= DVec3::new((3.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((6.0, 1.0, 1.0)));
  vec *= DVec3::new((3.0, 2.0, 1.0));
  assert_eq!(vec, DVec3::new((18.0, 2.0, 1.0)));
  vec *= DVec3::new((3.0, 3.0, 1.0));
  assert_eq!(vec, DVec3::new((54.0, 6.0, 1.0)));
  vec *= DVec3::new((3.0, 3.0, 2.0));
  assert_eq!(vec, DVec3::new((162.0, 18.0, 2.0)));
  vec *= DVec3::new((3.0, 3.0, 3.0));
  assert_eq!(vec, DVec3::new((486.0, 54.0, 6.0)));
}

#[test]
fn test_div_f64() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) / 1.0,
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) / 1.0,
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) / 1.0,
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) / 1.0,
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) / 1.0,
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) / 2.0,
    DVec3::new((1.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) / 4.0,
    DVec3::new((0.5, 0.5, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) / 4.0,
    DVec3::new((0.5, 0.5, 0.25))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / 4.0,
    DVec3::new((0.5, 0.5, 0.5))
  );
}

#[test]
fn test_div_f64_assign() {
  let mut vec = DVec3::new((1.0, 1.0, 1.0));
  vec /= 1.0;
  assert_eq!(vec, DVec3::new((1.0, 1.0, 1.0)));
  vec /= 2.0;
  assert_eq!(vec, DVec3::new((0.5, 0.5, 0.5)));
  vec /= 5.0;
  assert_eq!(vec, DVec3::new((0.1, 0.1, 0.1)));
}

#[test]
fn test_div() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((1.0, 1.0, 1.0)),
    DVec3::new((2.0, 2.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((2.0, 1.0, 1.0)),
    DVec3::new((1.0, 2.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((4.0, 1.0, 1.0)),
    DVec3::new((0.5, 2.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((4.0, 2.0, 1.0)),
    DVec3::new((0.5, 1.0, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((4.0, 4.0, 1.0)),
    DVec3::new((0.5, 0.5, 2.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((4.0, 4.0, 2.0)),
    DVec3::new((0.5, 0.5, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)) / DVec3::new((4.0, 4.0, 4.0)),
    DVec3::new((0.5, 0.5, 0.5))
  );
}

#[test]
fn test_div_assign() {
  let mut vec = DVec3::new((1.0, 1.0, 1.0));
  vec /= DVec3::new((1.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((1.0, 1.0, 1.0)));
  vec /= DVec3::new((2.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((0.5, 1.0, 1.0)));
  vec /= DVec3::new((5.0, 1.0, 1.0));
  assert_eq!(vec, DVec3::new((0.1, 1.0, 1.0)));
  vec /= DVec3::new((5.0, 2.0, 1.0));
  assert_eq!(vec, DVec3::new((0.02, 0.5, 1.0)));
  vec /= DVec3::new((5.0, 5.0, 1.0));
  assert_eq!(vec, DVec3::new((0.004, 0.1, 1.0)));
  vec /= DVec3::new((5.0, 5.0, 2.0));
  assert_eq!(vec, DVec3::new((0.0008, 0.02, 0.5)));
  vec /= DVec3::new((5.0, 5.0, 5.0));
  assert_eq!(vec, DVec3::new((0.00016, 0.004, 0.1)));
}

#[test]
fn test_set() {
  let mut vec = DVec3::new(());
  vec.set(DVec3::new((0.0, 0.0, 0.0)));
  assert_eq!(vec, DVec3::new((0.0, 0.0, 0.0)));
  vec.set(DVec3::new((1.0, 0.0, 0.0)));
  assert_eq!(vec, DVec3::new((1.0, 0.0, 0.0)));
  vec.set(DVec3::new((2.0, 0.0, 0.0)));
  assert_eq!(vec, DVec3::new((2.0, 0.0, 0.0)));
  vec.set(DVec3::new((2.0, 1.0, 0.0)));
  assert_eq!(vec, DVec3::new((2.0, 1.0, 0.0)));
  vec.set(DVec3::new((2.0, 2.0, 0.0)));
  assert_eq!(vec, DVec3::new((2.0, 2.0, 0.0)));
  vec.set(DVec3::new((2.0, 2.0, 1.0)));
  assert_eq!(vec, DVec3::new((2.0, 2.0, 1.0)));
  vec.set(DVec3::new((2.0, 2.0, 2.0)));
  assert_eq!(vec, DVec3::new((2.0, 2.0, 2.0)));
}

#[test]
fn test_get_squared_len() {
  assert_eq!(DVec3::new((0.0, 0.0, 0.0)).get_squared_len(), 0.0);
  assert_eq!(DVec3::new((1.0, 0.0, 0.0)).get_squared_len(), 1.0);
  assert_eq!(DVec3::new((2.0, 0.0, 0.0)).get_squared_len(), 4.0);
  assert_eq!(DVec3::new((2.0, 1.0, 0.0)).get_squared_len(), 5.0);
  assert_eq!(DVec3::new((2.0, 2.0, 0.0)).get_squared_len(), 8.0);
  assert_eq!(DVec3::new((2.0, 2.0, 1.0)).get_squared_len(), 9.0);
  assert_eq!(DVec3::new((2.0, 2.0, 2.0)).get_squared_len(), 12.0);
}

#[test]
fn test_get_len() {
  assert_eq!(DVec3::new((0.0, 0.0, 0.0)).get_len(), 0.0);
  assert_eq!(DVec3::new((1.0, 0.0, 0.0)).get_len(), 1.0);
  assert_eq!(DVec3::new((2.0, 0.0, 0.0)).get_len(), 2.0);
  assert_eq!(DVec3::new((2.0, 1.0, 0.0)).get_len(), 5.0f64.sqrt());
  assert_eq!(DVec3::new((2.0, 2.0, 0.0)).get_len(), 8.0f64.sqrt());
  assert!(DVec3::new((2.0, 2.0, 1.0)).get_len().approx_eq(3.0));
  assert!(DVec3::new((2.0, 2.0, 2.0))
    .get_len()
    .approx_eq(12.0f64.sqrt()));
}

#[test]
fn test_get_normalized() {
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).get_normalized(),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).get_normalized(),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((3.0, 0.0, 0.0)).get_normalized(),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((3.0, 4.0, 0.0)).get_normalized(),
    DVec3::new((0.6, 0.8, 0.0))
  );
  assert_eq!(
    DVec3::new((6.0, 8.0, 0.0)).get_normalized(),
    DVec3::new((0.6, 0.8, 0.0))
  );
  assert!(DVec3::new((2.0, 2.0, 1.0))
    .get_normalized()
    .approx_eq(DVec3::new((2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0))));
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).get_x_fliped(),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).get_x_fliped(),
    DVec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).get_x_fliped(),
    DVec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).get_x_fliped(),
    DVec3::new((-2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).get_x_fliped(),
    DVec3::new((-2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).get_x_fliped(),
    DVec3::new((-2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).get_x_fliped(),
    DVec3::new((-2.0, 2.0, 2.0))
  );
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).get_y_fliped(),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).get_y_fliped(),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).get_y_fliped(),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).get_y_fliped(),
    DVec3::new((2.0, -1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).get_y_fliped(),
    DVec3::new((2.0, -2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).get_y_fliped(),
    DVec3::new((2.0, -2.0, 1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).get_y_fliped(),
    DVec3::new((2.0, -2.0, 2.0))
  );
}

#[test]
fn test_get_z_fliped() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).get_z_fliped(),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).get_z_fliped(),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).get_z_fliped(),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).get_z_fliped(),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).get_z_fliped(),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).get_z_fliped(),
    DVec3::new((2.0, 2.0, -1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).get_z_fliped(),
    DVec3::new((2.0, 2.0, -2.0))
  );
}

#[test]
fn test_get_fliped() {
  assert_eq!(
    DVec3::new((0.0, 0.0, 0.0)).get_fliped(),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((1.0, 0.0, 0.0)).get_fliped(),
    DVec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 0.0, 0.0)).get_fliped(),
    DVec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 1.0, 0.0)).get_fliped(),
    DVec3::new((-2.0, -1.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 0.0)).get_fliped(),
    DVec3::new((-2.0, -2.0, 0.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 1.0)).get_fliped(),
    DVec3::new((-2.0, -2.0, -1.0))
  );
  assert_eq!(
    DVec3::new((2.0, 2.0, 2.0)).get_fliped(),
    DVec3::new((-2.0, -2.0, -2.0))
  );
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", DVec3::new((0.0, 0.0, 0.0))), "0,0,0");
  assert_eq!(format!("{}", DVec3::new((1.0, 0.0, 0.0))), "1,0,0");
  assert_eq!(format!("{}", DVec3::new((2.0, 0.0, 0.0))), "2,0,0");
  assert_eq!(format!("{}", DVec3::new((2.0, 1.0, 0.0))), "2,1,0");
  assert_eq!(format!("{}", DVec3::new((2.0, 2.0, 0.0))), "2,2,0");
  assert_eq!(format!("{}", DVec3::new((2.0, 2.0, 1.0))), "2,2,1");
  assert_eq!(format!("{}", DVec3::new((2.0, 2.0, 2.0))), "2,2,2");
}

#[test]
fn test_approx_eq() {
  assert!(DVec3::new((0.0, 0.0, 0.0)).approx_eq(DVec3::new((0.0, 0.0, 0.0))));
  assert!(!DVec3::new((1.0, 0.0, 0.0)).approx_eq(DVec3::new((0.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 0.0, 0.0)).approx_eq(DVec3::new((0.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 1.0, 0.0)).approx_eq(DVec3::new((0.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 0.0)).approx_eq(DVec3::new((0.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 0.0)).approx_eq(DVec3::new((1.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 0.0)).approx_eq(DVec3::new((2.0, 0.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 0.0)).approx_eq(DVec3::new((2.0, 1.0, 0.0))));
  assert!(DVec3::new((2.0, 2.0, 0.0)).approx_eq(DVec3::new((2.0, 2.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 1.0)).approx_eq(DVec3::new((2.0, 2.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 2.0)).approx_eq(DVec3::new((2.0, 2.0, 0.0))));
  assert!(!DVec3::new((2.0, 2.0, 2.0)).approx_eq(DVec3::new((2.0, 2.0, 1.0))));
  assert!(DVec3::new((2.0, 2.0, 2.0)).approx_eq(DVec3::new((2.0, 2.0, 2.0))));
}

#[test]
fn test_into_fvec3() {
  assert_eq!(
    FVec3::from(DVec3::new((0.0, 0.0, 0.0))),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((1.0, 0.0, 0.0))),
    FVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((2.0, 0.0, 0.0))),
    FVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((2.0, 1.0, 0.0))),
    FVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((2.0, 2.0, 0.0))),
    FVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((2.0, 2.0, 1.0))),
    FVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    FVec3::from(DVec3::new((2.0, 2.0, 2.0))),
    FVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_into_ivec3() {
  assert_eq!(
    IVec3::from(DVec3::new((0.0, 0.0, 0.0))),
    IVec3::new((0, 0, 0))
  );
  assert_eq!(
    IVec3::from(DVec3::new((1.0, 0.0, 0.0))),
    IVec3::new((1, 0, 0))
  );
  assert_eq!(
    IVec3::from(DVec3::new((2.0, 0.0, 0.0))),
    IVec3::new((2, 0, 0))
  );
  assert_eq!(
    IVec3::from(DVec3::new((2.0, 1.0, 0.0))),
    IVec3::new((2, 1, 0))
  );
  assert_eq!(
    IVec3::from(DVec3::new((2.0, 2.0, 0.0))),
    IVec3::new((2, 2, 0))
  );
  assert_eq!(
    IVec3::from(DVec3::new((2.0, 2.0, 1.0))),
    IVec3::new((2, 2, 1))
  );
  assert_eq!(
    IVec3::from(DVec3::new((2.0, 2.0, 2.0))),
    IVec3::new((2, 2, 2))
  );
}

#[test]
fn test_into_uvec3() {
  assert_eq!(
    UVec3::from(DVec3::new((0.0, 0.0, 0.0))),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::from(DVec3::new((1.0, 0.0, 0.0))),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::from(DVec3::new((2.0, 0.0, 0.0))),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::from(DVec3::new((2.0, 1.0, 0.0))),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::from(DVec3::new((2.0, 2.0, 0.0))),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::from(DVec3::new((2.0, 2.0, 1.0))),
    UVec3::new((2, 2, 1))
  );
  assert_eq!(
    UVec3::from(DVec3::new((2.0, 2.0, 2.0))),
    UVec3::new((2, 2, 2))
  );
}
