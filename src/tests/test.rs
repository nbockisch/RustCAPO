use std::collections::HashMap;
use crate::config::config;

#[test]
fn initialize_config() {
    let _config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();
}

#[test]
fn test_string_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected = String::from("user");
    let actual = config.get(&"section1.database.user").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u128_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: u128 = 100;
    let actual = config.get_u128(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_i128_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: i128 = 100;
    let actual = config.get_i128(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u64_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: u64 = 100;
    let actual = config.get_u64(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_i64_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: i64 = 100;
    let actual = config.get_i64(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u32_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: u32 = 100;
    let actual = config.get_u32(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_i32_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: i32 = 100;
    let actual = config.get_i32(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u16_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: u16 = 100;
    let actual = config.get_u16(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_i16_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: i16 = 100;
    let actual = config.get_i16(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u8_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: u8 = 100;
    let actual = config.get_u8(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_i8_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: i8 = 100;
    let actual = config.get_i8(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_f64_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: f64 = 3.1415;
    let actual = config.get_f64(&"section4.float.pi").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_f32_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: f32 = 3.1415;
    let actual = config.get_f32(&"section4.float.pi").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_usize_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: usize = 100;
    let actual = config.get_usize(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_isize_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: isize = 100;
    let actual = config.get_isize(&"section3.integer.hundred").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_bool_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected: bool = true;
    let actual = config.get_bool(&"section2.programa.run").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_all_options() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected = HashMap::from([
        (String::from("section1.database.user"), String::from("user")),
        (String::from("section1.database.password"), String::from("password")),
        (String::from("section2.programa.run"), String::from("true")),
        (String::from("section2.programa.output"),
            String::from("/testing/resources")),
        (String::from("section2.programb.run"), String::from("false")),
        (String::from("section2.programb.contactlist"),
            String::from("apples@banannas.com")),
        (String::from("section3.integer.one"), String::from("1")),
        (String::from("section3.integer.hundred"), String::from("100")),
        (String::from("section4.float.pi"), String::from("3.1415")),
        (String::from("section4.float.phi"), String::from("1.6180")),
    ]);
    let actual = config.get_options();

    for (key, val) in expected.iter() {
        assert_eq!(actual.get(&key.to_uppercase()).unwrap(), val);
    }

    let expected_len = 10;
    assert_eq!(actual.len(), expected_len);
}

#[test]
fn test_location_retrieval() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected = String::from("test.properties");
    let actual = config.get_location(&"section1.database.user").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_all_locations() {
    let config = config::CapoConfig::new(Some("test"), Some("src/tests"))
        .unwrap();

    let expected_location = String::from("test.properties");
    let expected = HashMap::from([
        (String::from("section1.database.user"),
            expected_location.clone()),
        (String::from("section1.database.password"), expected_location.clone()),
        (String::from("section2.programa.run"), expected_location.clone()),
        (String::from("section2.programa.output"),
            expected_location.clone()),
        (String::from("section2.programb.run"), expected_location.clone()),
        (String::from("section2.programb.contactlist"),
            expected_location.clone()),
        (String::from("section3.integer.one"), expected_location.clone()),
        (String::from("section3.integer.hundred"), expected_location.clone()),
        (String::from("section4.float.pi"), expected_location.clone()),
        (String::from("section4.float.phi"), expected_location.clone()),
    ]);
    let actual = config.get_locations();

    for (key, val) in expected.iter() {
        assert_eq!(actual.get(&key.to_uppercase()).unwrap(), val);
    }

    let expected_len = 10;
    assert_eq!(actual.len(), expected_len);
}
