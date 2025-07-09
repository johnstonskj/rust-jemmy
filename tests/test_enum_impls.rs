use jemmy::*;

const EX_ADDRESS: &str = "101 My Street";
const EX_XREF_ID: u64 = 867234;
const EX_XREF_ID_STR: &str = "867234";

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Enums ❱ Implementations ❱ From<> for Variant
// ------------------------------------------------------------------------------------------------

#[test]
fn test_impl_from_for_variant_base_case() {
    #[derive(Debug, PartialEq)]
    pub enum TypedAddress {
        Known(String),
    }
    impl_from_for_variant!(String => TypedAddress, Known);

    let address: TypedAddress = String::from(EX_ADDRESS).into();
    assert_eq!(address, TypedAddress::Known(EX_ADDRESS.to_string()));
}

#[test]
fn test_impl_from_for_variant_into_case() {
    #[derive(Debug, PartialEq)]
    pub enum TypedAddress {
        Known(String),
    }
    impl_from_for_variant!(into String => TypedAddress, Known);

    let address: TypedAddress = EX_ADDRESS.into();
    assert_eq!(address, TypedAddress::Known(EX_ADDRESS.to_string()));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Enums ❱ Implementations ❱ Delegate for Enum
// ------------------------------------------------------------------------------------------------

#[test]
fn test_impl_delegate_for_all_case() {
    #[derive(Debug, PartialEq)]
    pub enum TypedAddress {
        Known(String),
        XRef(u64),
    }
    impl_delegate_for_all!(TypedAddress [ Known, XRef ] => pub to_string -> String);

    let address = TypedAddress::Known(EX_ADDRESS.to_string());
    assert_eq!(address.to_string(), EX_ADDRESS.to_string());

    let address = TypedAddress::XRef(EX_XREF_ID);
    assert_eq!(address.to_string(), EX_XREF_ID_STR.to_string());
}
