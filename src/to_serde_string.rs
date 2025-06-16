use std::fmt::Debug;
use serde::Serialize;
use serde_json::Value;

pub fn to_serde_string<T: Serialize + Debug>(value: &T) -> String {
    if let Ok(Value::String(serialized)) = serde_json::to_value(value) {
        serialized
    } else {
        panic!("Failed to serialize value: {:?}", value);
    }
}

pub trait ToSerdeString {
    fn to_serde_string(&self) -> String;
}

impl<T: Serialize + Debug> ToSerdeString for T {
    fn to_serde_string(&self) -> String {
        to_serde_string(self)
    }
}

pub trait ToSerdeCommaDelimitedString {
    fn to_serde_comma_delimited_string(&self) -> String;
}

impl<T: Serialize + Debug> ToSerdeCommaDelimitedString for Vec<T> {
    fn to_serde_comma_delimited_string(&self) -> String {
        self.iter()
            .map(|v| v.to_serde_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}