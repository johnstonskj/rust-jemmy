use jemmy::*;

const EX_NUMBER_ON_STREET: u32 = 101;
const EX_UNIT_NUMBER: u32 = 24;
const EX_STREET_LINE_1: &str = "101 My Street";
const EX_STREET_LINE_2: &str = "Unit 202";

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ get!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_get_base_case() {
    pub struct Address {
        street_1: String,
    }
    impl Address {
        get!(pub street_1 => String);
    }

    let address = Address {
        street_1: EX_STREET_LINE_1.to_string(),
    };
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_base_case_rename() {
    pub struct Address {
        street_or_building: String,
    }
    impl Address {
        get!(pub street_1 => street_or_building, String);
    }

    let address = Address {
        street_or_building: EX_STREET_LINE_1.to_string(),
    };
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_copy_case() {
    pub struct Address {
        number_on_street: u32,
    }
    impl Address {
        get!(pub number_on_street => copy u32);
    }

    let address = Address {
        number_on_street: EX_NUMBER_ON_STREET,
    };
    assert_eq!(address.number_on_street(), EX_NUMBER_ON_STREET);
}

#[test]
fn test_get_copy_case_rename() {
    pub struct Address {
        number: u32,
    }
    impl Address {
        get!(pub number_on_street => number, copy u32);
    }

    let address = Address {
        number: EX_NUMBER_ON_STREET,
    };
    assert_eq!(address.number_on_street(), EX_NUMBER_ON_STREET);
}

#[test]
fn test_get_optional_case() {
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        get!(pub street_2 => optional String);
    }

    let address = Address {
        street_2: Some(EX_STREET_LINE_2.to_string()),
    };
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_get_optional_case_rename() {
    pub struct Address {
        street_additional: Option<String>,
    }
    impl Address {
        get!(pub street_2 => street_additional, optional String);
    }

    let address = Address {
        street_additional: Some(EX_STREET_LINE_2.to_string()),
    };
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_get_optional_copy_case() {
    pub struct Address {
        unit: Option<u32>,
    }
    impl Address {
        get!(pub unit => optional copy u32);
    }

    let address = Address { unit: Some(24) };
    assert_eq!(address.unit(), Some(24));
}

#[test]
fn test_get_optional_copy_case_rename() {
    pub struct Address {
        unit_number: Option<u32>,
    }
    impl Address {
        get!(pub unit => unit_number, optional copy u32);
    }

    let address = Address {
        unit_number: Some(EX_UNIT_NUMBER),
    };
    assert_eq!(address.unit(), Some(EX_UNIT_NUMBER));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ get_mut!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_get_mut_base_case() {
    pub struct Address {
        street_1: String,
    }
    impl Address {
        get_mut!(pub street_1 => String);
    }

    let mut address = Address {
        street_1: EX_STREET_LINE_1.to_string(),
    };
    let line_1 = address.street_1_mut();
    assert_eq!(line_1, EX_STREET_LINE_1);
    line_1.replace_range(0.., "202 My Street");
    assert_eq!(line_1, "202 My Street");
}

#[test]
fn test_get_mut_base_case_rename() {
    struct Address {
        street_line_1: String,
    }
    impl Address {
        get_mut!(pub street_1 => street_line_1, String);
    }

    let mut address = Address {
        street_line_1: EX_STREET_LINE_1.to_string(),
    };
    let line_1 = address.street_1_mut();
    assert_eq!(line_1, EX_STREET_LINE_1);
    line_1.replace_range(0.., "202 My Street");
    assert_eq!(line_1, "202 My Street");
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ set!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_set_base_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        set!(pub street_1 => String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1.as_str(), "");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_1.as_str(), EX_STREET_LINE_1);
}

#[test]
fn test_set_base_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        set!(pub street_1 => street_line_1, String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_line_1.as_str(), "");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_line_1.as_str(), EX_STREET_LINE_1);
}

#[test]
fn test_set_into_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        set!(pub street_1 => into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1.as_str(), "");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_1.as_str(), EX_STREET_LINE_1);
}

#[test]
fn test_set_into_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        set!(pub street_1 => street_line_1, into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_line_1.as_str(), "");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_line_1.as_str(), EX_STREET_LINE_1);
}

#[test]
fn test_set_optional_case() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        set!(pub street_2 => optional String);
    }

    let mut address = Address::default();
    assert_eq!(&address.street_2, &None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(&address.street_2, &Some(EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_set_optional_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        set!(pub street_2 => street_line_2, optional String);
    }

    let mut address = Address::default();
    assert_eq!(&address.street_line_2, &None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(&address.street_line_2, &Some(EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_set_optional_into_case() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        set!(pub street_2 => optional into String);
    }

    let mut address = Address::default();
    assert_eq!(&address.street_2, &None);
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(&address.street_2, &Some(EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_set_optional_into_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        set!(pub street_2 => street_line_2, optional into String);
    }

    let mut address = Address::default();
    assert_eq!(&address.street_line_2, &None);
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(&address.street_line_2, &Some(EX_STREET_LINE_2.to_string()));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ unset!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_unset_base_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        unset!(pub street_2);
    }

    let mut address = Address {
        street_2: Some(EX_STREET_LINE_2.to_string()),
    };
    assert_eq!(&address.street_2, &Some(EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(&address.street_2, &None);
}

#[test]
fn test_unset_base_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        unset!(pub street_2 => street_line_2);
    }

    let mut address = Address {
        street_line_2: Some(EX_STREET_LINE_2.to_string()),
    };
    assert_eq!(&address.street_line_2, &Some(EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(&address.street_line_2, &None);
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ with!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_with_base_case() {
    #[derive(Default)]
    pub struct Address {
        street_1: String,
    }
    impl Address {
        with!(pub street_1 => String);
    }

    let address = Address::default().with_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(&address.street_1, &EX_STREET_LINE_1.to_string());
}

#[test]
fn test_with_base_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_1: String,
    }
    impl Address {
        with!(pub street_1 => street_line_1, String);
    }

    let address = Address::default().with_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(&address.street_line_1, &EX_STREET_LINE_1.to_string());
}

#[test]
fn test_with_into_case() {
    #[derive(Default)]
    pub struct Address {
        street_1: String,
    }
    impl Address {
        with!(pub street_1 => into String);
    }

    let address = Address::default().with_street_1(EX_STREET_LINE_1);
    assert_eq!(&address.street_1, &EX_STREET_LINE_1.to_string());
}

#[test]
fn test_with_into_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_1: String,
    }
    impl Address {
        with!(pub street_1 => street_line_1, into String);
    }

    let address = Address::default().with_street_1(EX_STREET_LINE_1);
    assert_eq!(&address.street_line_1, &EX_STREET_LINE_1.to_string());
}

#[test]
fn test_with_optional_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with!(pub street_2 => optional String);
    }

    let address = Address::default().with_street_2(EX_STREET_LINE_1.to_string());
    assert_eq!(&address.street_2, &Some(EX_STREET_LINE_1.to_string()));
}

#[test]
fn test_with_optional_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        with!(pub street_2 => street_line_2, optional String);
    }

    let address = Address::default().with_street_2(EX_STREET_LINE_1.to_string());
    assert_eq!(&address.street_line_2, &Some(EX_STREET_LINE_1.to_string()));
}

#[test]
fn test_with_optional_into_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with!(pub street_2 => optional into String);
    }

    let address = Address::default().with_street_2(EX_STREET_LINE_1);
    assert_eq!(&address.street_2, &Some(EX_STREET_LINE_1.to_string()));
}

#[test]
fn test_with_optional_into_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        with!(pub street_2 => street_line_2, optional into String);
    }

    let address = Address::default().with_street_2(EX_STREET_LINE_1);
    assert_eq!(&address.street_line_2, &Some(EX_STREET_LINE_1.to_string()));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ Combinators ❱ get_and_set!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_get_and_set_base_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        get_and_set!(pub street_1 => String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1(), "");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_and_set_base_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        get_and_set!(pub street_1 => street_line_1, String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1(), "");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_and_set_copy_case() {
    #[derive(Default)]
    struct Address {
        unit: u32,
    }
    impl Address {
        get_and_set!(pub unit => copy u32);
    }

    let mut address = Address::default();
    assert_eq!(address.unit(), 0);
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), EX_UNIT_NUMBER);
}

#[test]
fn test_get_and_set_copy_case_rename() {
    #[derive(Default)]
    struct Address {
        unit_number: u32,
    }
    impl Address {
        get_and_set!(pub unit => unit_number, copy u32);
    }

    let mut address = Address::default();
    assert_eq!(address.unit(), 0);
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), EX_UNIT_NUMBER);
}

#[test]
fn test_get_and_set_into_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        get_and_set!(pub street_1 => into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1(), "");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_and_set_into_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        get_and_set!(pub street_1 => street_line_1, into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_1(), "");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_get_and_set_optional_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        get_and_set!(pub street_2 => optional String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_get_and_set_optional_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        get_and_set!(pub street_2 => street_line_2, optional String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_get_and_set_optional_copy_case() {
    #[derive(Default)]
    pub struct Address {
        unit: Option<u32>,
    }
    impl Address {
        get_and_set!(pub unit => optional copy u32);
    }

    let mut address = Address::default();
    assert_eq!(address.unit(), None);
    address.set_unit(24);
    assert_eq!(address.unit(), Some(24));
}

#[test]
fn test_get_and_set_optional_copy_case_rename() {
    #[derive(Default)]
    pub struct Address {
        unit_number: Option<u32>,
    }
    impl Address {
        get_and_set!(pub unit => unit_number, optional copy u32);
    }

    let mut address = Address::default();
    assert_eq!(address.unit(), None);
    address.set_unit(24);
    assert_eq!(address.unit(), Some(24));
}

#[test]
fn test_get_and_set_optional_into_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        get_and_set!(pub street_2 => optional into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_get_and_set_optional_into_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        get_and_set!(pub street_2 => street_line_2, optional into String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ Combinators ❱ with_get_and_set!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_with_get_and_set_base_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        with_get_and_set!(pub street_1 => String);
    }

    let mut address = Address::default().with_street_1("--".to_string());
    assert_eq!(address.street_1(), "--");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_with_get_and_set_base_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        with_get_and_set!(pub street_1 => street_line_1, String);
    }

    let mut address = Address::default().with_street_1("--".to_string());
    assert_eq!(address.street_1(), "--");
    address.set_street_1(EX_STREET_LINE_1.to_string());
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_with_get_and_set_copy_case() {
    #[derive(Default)]
    struct Address {
        unit: u32,
    }
    impl Address {
        with_get_and_set!(pub unit => copy u32);
    }

    let mut address = Address::default().with_unit(99);
    assert_eq!(address.unit(), 99);
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), EX_UNIT_NUMBER);
}

#[test]
fn test_with_get_and_set_copy_case_rename() {
    #[derive(Default)]
    struct Address {
        unit_number: u32,
    }
    impl Address {
        with_get_and_set!(pub unit => unit_number, copy u32);
    }

    let mut address = Address::default().with_unit(99);
    assert_eq!(address.unit(), 99);
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), EX_UNIT_NUMBER);
}

#[test]
fn test_with_get_and_set_into_case() {
    #[derive(Default)]
    struct Address {
        street_1: String,
    }
    impl Address {
        with_get_and_set!(pub street_1 => into String);
    }

    let mut address = Address::default().with_street_1("--");
    assert_eq!(address.street_1(), "--");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_with_get_and_set_into_case_rename() {
    #[derive(Default)]
    struct Address {
        street_line_1: String,
    }
    impl Address {
        with_get_and_set!(pub street_1 => street_line_1, into String);
    }

    let mut address = Address::default().with_street_1("--");
    assert_eq!(address.street_1(), "--");
    address.set_street_1(EX_STREET_LINE_1);
    assert_eq!(address.street_1(), EX_STREET_LINE_1);
}

#[test]
fn test_with_get_and_set_optional_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with_get_and_set!(pub street_2 => optional String);
    }

    let mut address = Address::default().with_street_2("--".to_string());
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_with_get_and_set_optional_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        with_get_and_set!(pub street_2 => street_line_2, optional String);
    }

    let mut address = Address::default().with_street_2("--".to_string());
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_with_get_and_set_optional_copy_case() {
    #[derive(Default)]
    pub struct Address {
        unit: Option<u32>,
    }
    impl Address {
        with_get_and_set!(pub unit => optional copy u32);
    }

    let mut address = Address::default().with_unit(99);
    assert_eq!(address.unit(), Some(99));
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), Some(EX_UNIT_NUMBER));
}

#[test]
fn test_with_get_and_set_optional_copy_case_rename() {
    #[derive(Default)]
    pub struct Address {
        unit_number: Option<u32>,
    }
    impl Address {
        with_get_and_set!(pub unit => unit_number, optional copy u32);
    }

    let mut address = Address::default().with_unit(99);
    assert_eq!(address.unit(), Some(99));
    address.set_unit(EX_UNIT_NUMBER);
    assert_eq!(address.unit(), Some(EX_UNIT_NUMBER));
}

#[test]
fn test_with_get_and_set_optional_into_case() {
    #[derive(Default)]
    pub struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with_get_and_set!(pub street_2 => optional into String);
    }

    let mut address = Address::default().with_street_2("--");
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

#[test]
fn test_with_get_and_set_optional_into_case_rename() {
    #[derive(Default)]
    pub struct Address {
        street_line_2: Option<String>,
    }
    impl Address {
        with_get_and_set!(pub street_2 => street_line_2, optional into String);
    }

    let mut address = Address::default().with_street_2("--");
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2);
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ Combinators ❱ get_set_and_unset!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_get_set_and_unset_base_case() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        get_set_and_unset!(pub street_2 => String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(address.street_2(), None);
}

#[test]
fn test_get_set_and_unset_base_case_rename() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        get_set_and_unset!(pub street_2 => String);
    }

    let mut address = Address::default();
    assert_eq!(address.street_2(), None);
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(address.street_2(), None);
}

// ------------------------------------------------------------------------------------------------
// Test Cases ❱ Structures ❱ Combinators ❱ with_get_set_and_unset!
// ------------------------------------------------------------------------------------------------

#[test]
fn test_with_get_set_and_unset_base_case() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with_get_set_and_unset!(pub street_2 => String);
    }

    let mut address = Address::default().with_street_2("--".to_string());
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(address.street_2(), None);
}

#[test]
fn test_with_get_set_and_unset_base_case_rename() {
    #[derive(Default)]
    struct Address {
        street_2: Option<String>,
    }
    impl Address {
        with_get_set_and_unset!(pub street_2 => String);
    }

    let mut address = Address::default().with_street_2("--".to_string());
    assert_eq!(address.street_2(), Some(&"--".to_string()));
    address.set_street_2(EX_STREET_LINE_2.to_string());
    assert_eq!(address.street_2(), Some(&EX_STREET_LINE_2.to_string()));
    address.unset_street_2();
    assert_eq!(address.street_2(), None);
}

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
fn test_is_variant_data_case() {
    #[allow(dead_code)]
    pub enum TypedAddress {
        Known(String),
        Unknown,
    }
    impl TypedAddress {
        is_variant!(pub Known => String);
    }

    let address = TypedAddress::Known("Here".to_string());
    assert!(address.is_known());
    assert!(!TypedAddress::Unknown.is_known());
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

    let address = TypedAddress::Known("Here".to_string());
    let actual = address.as_known().unwrap();
    assert_eq!(actual, "Here");
}

#[test]
fn test_as_variant_copy_case() {
    pub enum TypedAddress {
        XRef(u64),
    }
    impl TypedAddress {
        as_variant!(pub XRef => copy u64);
    }

    let address = TypedAddress::XRef(99);
    let actual = address.as_x_ref().unwrap();
    assert_eq!(actual, 99);
}

#[test]
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
fn test_as_variant_value_case() {
    pub enum TypedAddress {
        Unknown,
    }
    impl TypedAddress {
        as_variant!(pub Unknown => true, bool);
    }

    let address = TypedAddress::Unknown;
    let actual = address.as_unknown().unwrap();
    assert_eq!(actual, true);
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

    let address = TypedAddress::Known("Here".to_string());
    assert!(address.is_known());
    let actual = address.as_known().unwrap();
    assert_eq!(actual, "Here");
}

#[test]
fn test_is_as_variant_copy_case() {
    pub enum TypedAddress {
        XRef(u32),
    }
    impl TypedAddress {
        is_as_variant!(pub XRef => copy u32);
    }

    let address = TypedAddress::XRef(99);
    assert!(address.is_x_ref());
    let actual = address.as_x_ref().unwrap();
    assert_eq!(actual, 99);
}

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

    let address: TypedAddress = String::from("Here").into();
    assert_eq!(address, TypedAddress::Known("Here".to_string()));
}

#[test]
fn test_impl_from_for_variant_into_case() {
    #[derive(Debug, PartialEq)]
    pub enum TypedAddress {
        Known(String),
    }
    impl_from_for_variant!(into String => TypedAddress, Known);

    let address: TypedAddress = "Here".into();
    assert_eq!(address, TypedAddress::Known("Here".to_string()));
}
