use iron_ingot::{DBound, DSeq, FBound, FSeq, IBound, UBound};

#[test]
fn test_get_middle() {
  assert_eq!(UBound::new((0, 0)).get_middle(), 0);
  assert_eq!(UBound::new((0, 1)).get_middle(), 0);
  assert_eq!(UBound::new((0, 2)).get_middle(), 1);
  assert_eq!(UBound::new((1, 2)).get_middle(), 1);
  assert_eq!(UBound::new((2, 2)).get_middle(), 2);
}

#[test]
fn test_is_intersect() {
  assert!(UBound::new((0, 0)).is_intersect(UBound::new((0, 0))));
  assert!(UBound::new((0, 1)).is_intersect(UBound::new((0, 0))));
  assert!(UBound::new((0, 2)).is_intersect(UBound::new((0, 0))));
  assert!(!UBound::new((1, 2)).is_intersect(UBound::new((0, 0))));
  assert!(!UBound::new((2, 2)).is_intersect(UBound::new((0, 0))));
  assert!(!UBound::new((2, 2)).is_intersect(UBound::new((0, 1))));
  assert!(UBound::new((2, 2)).is_intersect(UBound::new((0, 2))));
  assert!(UBound::new((2, 2)).is_intersect(UBound::new((1, 2))));
  assert!(UBound::new((2, 2)).is_intersect(UBound::new((2, 2))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    UBound::new((0, 0)).intersect(UBound::new((0, 0))),
    Some(UBound::new((0, 0)))
  );
  assert_eq!(
    UBound::new((0, 1)).intersect(UBound::new((0, 0))),
    Some(UBound::new((0, 0)))
  );
  assert_eq!(
    UBound::new((0, 2)).intersect(UBound::new((0, 0))),
    Some(UBound::new((0, 0)))
  );
  assert_eq!(UBound::new((1, 2)).intersect(UBound::new((0, 0))), None);
  assert_eq!(UBound::new((2, 2)).intersect(UBound::new((0, 0))), None);
  assert_eq!(UBound::new((2, 2)).intersect(UBound::new((0, 1))), None);
  assert_eq!(
    UBound::new((2, 2)).intersect(UBound::new((0, 2))),
    Some(UBound::new((2, 2)))
  );
  assert_eq!(
    UBound::new((2, 2)).intersect(UBound::new((1, 2))),
    Some(UBound::new((2, 2)))
  );
  assert_eq!(
    UBound::new((2, 2)).intersect(UBound::new((2, 2))),
    Some(UBound::new((2, 2)))
  );
}

#[test]
fn test_has() {
  assert!(UBound::new((0, 0)).has(0));
  assert!(UBound::new((0, 1)).has(0));
  assert!(UBound::new((0, 2)).has(0));
  assert!(!UBound::new((1, 2)).has(0));
  assert!(!UBound::new((2, 2)).has(0));
  assert!(!UBound::new((2, 2)).has(1));
  assert!(UBound::new((2, 2)).has(2));
}

#[test]
fn test_rand() {
  assert_eq!(UBound::new((0, 0)).rand(), 0);
  assert!(UBound::new((0, 1)).rand() <= 1);
  assert!(UBound::new((0, 2)).rand() <= 2);
  assert!(UBound::new((1, 2)).rand() >= 1);
  assert!(UBound::new((1, 2)).rand() <= 2);
  assert_eq!(UBound::new((2, 2)).rand(), 2);
}

#[test]
fn test_into_dbound() {
  assert_eq!(DBound::from(UBound::new((0, 1))), DBound::new((0.0, 1.0)));
  assert_eq!(DBound::from(UBound::new((0, 2))), DBound::new((0.0, 2.0)));
  assert_eq!(DBound::from(UBound::new((1, 2))), DBound::new((1.0, 2.0)));
}

#[test]
fn test_into_fbound() {
  assert_eq!(FBound::from(UBound::new((0, 1))), FBound::new((0.0, 1.0)));
  assert_eq!(FBound::from(UBound::new((0, 2))), FBound::new((0.0, 2.0)));
  assert_eq!(FBound::from(UBound::new((1, 2))), FBound::new((1.0, 2.0)));
}

#[test]
fn test_into_ibound() {
  assert_eq!(IBound::from(UBound::new((0, 1))), IBound::new((0, 1)));
  assert_eq!(IBound::from(UBound::new((0, 2))), IBound::new((0, 2)));
  assert_eq!(IBound::from(UBound::new((1, 2))), IBound::new((1, 2)));
}

#[test]
fn test_into_dseq() {
  assert_eq!(DSeq::from(UBound::new((0, 1))), DSeq::new(0.0, 1.0));
  assert_eq!(DSeq::from(UBound::new((0, 2))), DSeq::new(0.0, 2.0));
  assert_eq!(DSeq::from(UBound::new((1, 2))), DSeq::new(1.0, 2.0));
}

#[test]
fn test_into_fseq() {
  assert_eq!(FSeq::from(UBound::new((0, 1))), FSeq::new(0.0, 1.0));
  assert_eq!(FSeq::from(UBound::new((0, 2))), FSeq::new(0.0, 2.0));
  assert_eq!(FSeq::from(UBound::new((1, 2))), FSeq::new(1.0, 2.0));
}
