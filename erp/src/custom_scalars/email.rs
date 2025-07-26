use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use grand_line::*;
use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Email(pub String);

#[Scalar(name = "Email")]
impl ScalarType for Email {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) if ValidateEmail::validate_email(&s) => Ok(Email(s.clone())),
            Value::String(s) => Err(InputValueError::custom(format!(
                "Invalid email format: {}",
                s
            ))),
            v => Err(InputValueError::expected_type(v)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.clone())
    }
}
