use jemmy::*;

const EX_ADDRESS: &str = "101 My Street";
const EX_XREF_ID: u64 = 867234;

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Enums ❱ is_variant!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_is_variant_base_case() {
    pub enum TypedAddress {
        Known,
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Unknown);
    }

    let address = TypedAddress::Unknown;
    assert!(address.is_unknown());
    assert!(!TypedAddress::Known.is_unknown());
}

#[test]
fn test_is_variant_base_case_rename() {
    pub enum TypedAddress {
        Known,
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Unknown, not_known);
    }

    let address = TypedAddress::Unknown;
    assert!(address.is_not_known());
    assert!(!TypedAddress::Known.is_not_known());
}

#[test]
fn test_is_variant_alternate_base_case() {
    pub enum TypedAddress {
        Known,
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Unknown => ());
    }

    let address = TypedAddress::Unknown;
    assert!(address.is_unknown());
    assert!(!TypedAddress::Known.is_unknown());
}

#[test]
fn test_is_variant_alternate_base_case_rename() {
    pub enum TypedAddress {
        Known,
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Unknown, not_known => ());
    }

    let address = TypedAddress::Unknown;
    assert!(address.is_not_known());
    assert!(!TypedAddress::Known.is_not_known());
}

#[test]
fn test_is_variant_data_case() {
    #[allow(dead_code)]
    pub enum TypedAddress {
        Known(String),
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Known => String);
    }

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    assert!(address.is_known());
    assert!(!TypedAddress::Unknown.is_known());
}

#[test]
fn test_is_variant_data_case_rename() {
    #[allow(dead_code)]
    pub enum TypedAddress {
        Known(String),
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Known, address => String);
    }

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    assert!(address.is_address());
    assert!(!TypedAddress::Unknown.is_address());
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Enums ❱ as_variant!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_as_variant_base_case() {
    pub enum TypedAddress {
        Known(String),
    }
    impl TypedAddress {
        as_variant!(pub Known => String);
    }

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    let actual = address.as_known().unwrap();
    assert_eq!(actual, EX_ADDRESS);
}

#[test]
fn test_as_variant_base_case_rename() {
    pub enum TypedAddress {
        Known(String),
    }
    impl TypedAddress {
        as_variant!(pub Known, address => String);
    }

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    let actual = address.as_address().unwrap();
    assert_eq!(actual, EX_ADDRESS);
}

#[test]
fn test_as_variant_copy_case() {
    pub enum TypedAddress {
        XRef(u64),
    }
    impl TypedAddress {
        as_variant!(pub XRef => copy u64);
    }

    let address = TypedAddress::XRef(EX_XREF_ID);
    let actual = address.as_x_ref().unwrap();
    assert_eq!(actual, EX_XREF_ID);
}

#[test]
fn test_as_variant_copy_case_rename() {
    pub enum TypedAddress {
        XRef(u64),
    }
    impl TypedAddress {
        as_variant!(pub XRef, external_id => copy u64);
    }

    let address = TypedAddress::XRef(EX_XREF_ID);
    let actual = address.as_external_id().unwrap();
    assert_eq!(actual, EX_XREF_ID);
}

#[test]
#[allow(clippy::let_unit_value)]
#[allow(clippy::unit_cmp)]
fn test_as_variant_empty_case() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown => ());
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_unknown().unwrap();
    assert_eq!(actual, ());
}

#[test]
#[allow(clippy::let_unit_value)]
#[allow(clippy::unit_cmp)]
fn test_as_variant_empty_case_rename() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown, not_known => ());
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_not_known().unwrap();
    assert_eq!(actual, ());
}

#[test]
#[allow(clippy::let_unit_value)]
#[allow(clippy::unit_cmp)]
fn test_as_variant_empty_2_case() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown);
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_unknown().unwrap();
    assert_eq!(actual, ());
}

#[test]
#[allow(clippy::let_unit_value)]
#[allow(clippy::unit_cmp)]
fn test_as_variant_empty_2_case_rename() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown, not_known);
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_not_known().unwrap();
    assert_eq!(actual, ());
}

#[test]
fn test_as_variant_value_case() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown => true, bool);
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_unknown().unwrap();
    assert!(actual);
}

#[test]
fn test_as_variant_value_case_rename() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown, not_known => true, bool);
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_not_known().unwrap();
    assert!(actual);
}

#[test]
fn test_as_variant_value_case_2() {
    #[derive(Debug, PartialEq)]
    pub enum Error {
        InvalidAddressError,
    }
    pub enum TypedAddress {
        BadlyFormed,
    }
    impl TypedAddress {
        as_variant!(pub BadlyFormed => Error::InvalidAddressError, Error);
    }

    let address = TypedAddress::BadlyFormed;
    let actual = address.as_badly_formed().unwrap();
    assert_eq!(actual, Error::InvalidAddressError);
}

#[test]
fn test_as_variant_value_case_2_rename() {
    #[derive(Debug, PartialEq)]
    pub enum Error {
        InvalidAddressError,
    }
    pub enum TypedAddress {
        BadlyFormed,
    }
    impl TypedAddress {
        as_variant!(pub BadlyFormed, messed_up => Error::InvalidAddressError, Error);
    }

    let address = TypedAddress::BadlyFormed;
    let actual = address.as_messed_up().unwrap();
    assert_eq!(actual, Error::InvalidAddressError);
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Enums ❱ Combinators ❱ is_as_variant!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_is_as_variant_base_case() {
    pub enum TypedAddress {
        Known(String),
    }
    impl TypedAddress {
        is_as_variant!(pub Known => String);
    }

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    assert!(address.is_known());
    let actual = address.as_known().unwrap();
    assert_eq!(actual, EX_ADDRESS);
}

#[test]
fn test_is_as_variant_copy_case() {
    pub enum TypedAddress {
        XRef(u64),
    }
    impl TypedAddress {
        is_as_variant!(pub XRef => copy u64);
    }

    let address = TypedAddress::XRef(EX_XREF_ID);
    assert!(address.is_x_ref());
    let actual = address.as_x_ref().unwrap();
    assert_eq!(actual, EX_XREF_ID);
}
