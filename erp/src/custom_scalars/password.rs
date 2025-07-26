use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use grand_line::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Password( pub String);

fn validate_password(s: &str) -> bool {
    let len_ok = s.len() >= 8;
    let has_lower = s.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = s.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    let has_special = s.chars().any(|c| !c.is_ascii_alphanumeric());
    len_ok && has_lower && has_upper && has_digit && has_special
}

#[Scalar(name = "Password")]
impl ScalarType for Password {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) if validate_password(&s) => Ok(Password(s.clone())),
            Value::String(s) => Err(InputValueError::custom(format!(
                "Password must be at least 8 characters, include lowercase, uppercase, digit, and special character: {}",
                s
            ))),
            v => Err(InputValueError::expected_type(v)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.clone())
    }
}