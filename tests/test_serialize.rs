use serde_derive::Serialize;
use proptest::*;
use proptest_derive::*;
use strum::Display;
use percent_encoding::*;

#[derive(Serialize)]
struct NewType<T>(T);

#[derive(Serialize)]
struct Struct<T> {
    field: T,
}

#[derive(Serialize, Arbitrary, Debug, Display, Clone)]
enum UnitEnum {
    A,
    B,
    C,
}

#[derive(Serialize)]
struct Unit;

//const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

macro_rules! assert_to_string {
    ($value:expr, $string:literal) => {
        assert_eq!(serde_urlencoded::to_string(&$value), Ok(format!($string)));
    };
}

macro_rules! assert_error {
    ($value:expr, $error:literal) => {
        assert_eq!(serde_urlencoded::to_string(&$value).err().unwrap().to_string(), format!($error));
    };
}

macro_rules! assert_field_display {
    ($value:expr) => {
        assert_field_display!($value, $value);
    };

    ($value:expr, $encoding:expr) => {
        let value = $value.clone();
        let encoding = $encoding;
        assert_to_string!([("field", value.clone())], "field={encoding}");
        assert_to_string!([("field", Some(value.clone()))], "field={encoding}");
        assert_to_string!([("field", NewType(value.clone()))], "field={encoding}");
        assert_to_string!([("field", Some(NewType(value.clone())))], "field={encoding}");
        assert_to_string!(Struct { field: value.clone() }, "field={encoding}");
    };
}

proptest! {
    #[test]
    fn serialize_i8(value: i8) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_i16(value: i16) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_i32(value: i32) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_i64(value: i64) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_i128(value: i128) {
        assert_error!(value, "i128 is not supported");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_u8(value: u8) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_u16(value: u16) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_u32(value: u32) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_u64(value: u64) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_u128(value: u128) {
        assert_error!(value, "u128 is not supported");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_f32(value: f32) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        assert_field_display!(value, part);
    }

    #[test]
    fn serialize_f64(value: f64) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        assert_field_display!(value, part);
    }

    #[test]
    fn serialize_bool(value: bool) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }

    #[test]
    fn serialize_string(value: String) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        let display: String = value.chars().map(encode_char).collect();
        assert_field_display!(value, display);
    }

    #[test]
    fn serialize_char(value: char) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        let display = encode_char(value);
        assert_field_display!(value, display);
    }

    #[test]
    fn serialize_enum(value: UnitEnum) {
        assert_error!(value, "top-level serializer supports only maps and structs");
        assert_field_display!(value);
    }
}

fn encode_char(input: char) -> String {
    let mut char_bytes: [u8; 4] = [0; 4];
    match input {
        value if value.is_ascii_alphanumeric() => format!("{value}"),
        value if value == '*' => format!("{value}"),
        value if value == '.' => format!("{value}"),
        value if value == '-' => format!("{value}"),
        value if value == '_' => format!("{value}"),
        value if value == ' ' => format!("+"),
        value => value.encode_utf8(&mut char_bytes[..]).bytes().map(|b| format!("%{b:02X}")).collect(),
    }
}

#[test]
fn serialize_option_map_int() {
    assert_to_string!(
        [("first", Some(23)), ("middle", None), ("last", Some(42))],
        "first=23&last=42"
    );
}

#[test]
fn serialize_option_map_string() {
    assert_to_string!([
        ("first", Some("hello")),
        ("middle", None),
        ("last", Some("world")),
    ], "first=hello&last=world");
}

#[test]
fn serialize_option_map_bool() {
    assert_to_string!(
        [("one", Some(true)), ("two", Some(false))],
        "one=true&two=false"
    );
}

#[test]
fn serialize_map_bool() {
    let params = &[("one", true), ("two", false)];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("one=true&two=false".to_owned())
    );
}

#[test]
fn serialize_unit_enum() {
    let params = &[("one", UnitEnum::A), ("two", UnitEnum::B), ("three", UnitEnum::C)];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("one=A&two=B&three=C".to_owned())
    );
}

#[test]
fn serialize_unit_struct() {
    assert_eq!(serde_urlencoded::to_string(Unit), Ok("".to_owned()));
}

#[test]
fn serialize_unit_type() {
    assert_eq!(serde_urlencoded::to_string(()), Ok("".to_owned()));
}
