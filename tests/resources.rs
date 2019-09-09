use std::str::FromStr;

use proptest::prelude::*;

use ot_rs::api::resources::{LabelName, LabelValue, Resource};

proptest! {
    #[test]
    fn valid_name(ref s in "[ -~]{1,255}") {
        let a = LabelName::from_str(s);
        prop_assert!(a.is_ok());
        let aa = a.unwrap();
        prop_assert_eq!(aa.value(), s);
    }

    #[test]
    fn invalid_name_empty(ref s in "[ -~]{0}") {
        let a = LabelName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_name_so_long(ref s in "[ -~]{256}") {
        let a = LabelName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_name_chars(ref s in "[^ -~]{1, 255}") {
        let a = LabelName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn valid_value(ref s in "[ -~]{0,255}") {
        let a = LabelValue::from_str(s);
        prop_assert!(a.is_ok());
        let aa = a.unwrap();
        prop_assert_eq!(aa.value(), s);
    }

    #[test]
    fn invalid_value_so_long(ref s in "[ -~]{256}") {
        let a = LabelValue::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_value_chars(ref s in "[^ -~]{1, 255}") {
        let a = LabelValue::from_str(s);
        prop_assert!(a.is_err());
    }
}

#[test]
fn merge_test() {
    let mut x = Resource::default();
    x.try_upsert("a", "1").unwrap();
    let mut y = Resource::default();
    y.try_upsert("b", "2").unwrap();
    x.merge(&y);
    assert!(x
        .labels()
        .any(|(k, v)| k.value() == "a" && v.value() == "1"));
    assert!(x
        .labels()
        .any(|(k, v)| k.value() == "b" && v.value() == "2"));
}
