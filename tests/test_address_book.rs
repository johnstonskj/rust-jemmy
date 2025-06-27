use jemmy::*;

#[derive(Default)]
pub struct Address {
    street_1: String,
    street_2: Option<String>,
    city: String,
    region_or_state: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
}

impl Address {
    with_get_and_set!(pub street_1 => into String);
    get_set_and_unset!(pub street_2 => String);

    with!(pub city => into String);
    get!(pub city => String);
    set!(pub city => into String);

    get!(pub region_or_state => optional String);
    set!(pub region_or_state => optional String);
    unset!(pub region_or_state);

    get!(pub country => optional String);
    set!(pub country => optional String);
    unset!(pub country);

    get!(pub postal_code => optional String);
    set!(pub postal_code => optional String);
    unset!(pub postal_code);
}

pub enum TypedAddress {
    Home(Address),
    Work(Address),
    Other(Address),
}

impl TypedAddress {
    is_as_variant!(Home => Address);
    is_as_variant!(Work => Address);
    is_as_variant!(Other => Address);
}

#[test]
fn test_address_with_street_1() {
    let address = Address::default().with_street_1("101 My Street");
    assert_eq!(address.street_1(), &"101 My Street".to_string());
}

#[test]
fn test_address_street_1_setter() {
    let mut address = Address::default();
    address.set_street_1("101 My Street");
    assert_eq!(address.street_1(), &"101 My Street".to_string());
}

#[test]
fn test_typed_address_is_as() {
    let typed_address = TypedAddress::Home(Address::default().with_city("Toronto"));

    assert!(typed_address.is_home());
    assert_eq!(
        typed_address.as_home().unwrap().city(),
        &"Toronto".to_string()
    );

    assert!(!typed_address.is_work());
    assert!(!typed_address.is_other());
}
