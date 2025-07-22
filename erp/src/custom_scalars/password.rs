use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use grand_line::*;
use serde::{Deserialize, Serialize};
use validator::ValidateRegex;
use regex::Regex;

pub const PASSWORD_REGEX: &str = r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^A-Za-z0-9]).{8,}$";

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Password(String);

#[Scalar(name = "Password")]
impl ScalarType for Password {
    fn parse(value: Value) -> InputValueResult<Self> {
        let regex = Regex::new(PASSWORD_REGEX).unwrap();
        match value {
            Value::String(s) if ValidateRegex::validate_regex(&s,regex) => Ok(Password(s.clone())),
            Value::String(s) => Err(InputValueError::custom(format!(
                "Invalid password format: {}",
                s
            ))),
            v => Err(InputValueError::expected_type(v)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.clone())
    }
}
