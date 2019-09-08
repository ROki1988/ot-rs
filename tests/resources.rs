use ot_rs::api::resources::{ResourceName, ResourceValue};
use std::str::FromStr;

use proptest::prelude::*;

proptest! {
    #[test]
    fn valid_name(ref s in "[ -~]{1,255}") {
        let a = ResourceName::from_str(s);
        prop_assert!(a.is_ok());
    }

    #[test]
    fn invalid_name_empty(ref s in "[ -~]{0}") {
        let a = ResourceName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_name_so_long(ref s in "[ -~]{256}") {
        let a = ResourceName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_name_chars(ref s in "[^ -~]{1, 255}") {
        let a = ResourceName::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn valid_value(ref s in "[ -~]{0,255}") {
        let a = ResourceValue::from_str(s);
        prop_assert!(a.is_ok());
    }

    #[test]
    fn invalid_value_so_long(ref s in "[ -~]{256}") {
        let a = ResourceValue::from_str(s);
        prop_assert!(a.is_err());
    }

    #[test]
    fn invalid_value_chars(ref s in "[^ -~]{1, 255}") {
        let a = ResourceValue::from_str(s);
        prop_assert!(a.is_err());
    }
}
