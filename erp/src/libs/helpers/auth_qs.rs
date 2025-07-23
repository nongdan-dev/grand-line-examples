use grand_line::serde::{Deserialize, Serialize};
use serde_qs::{self as qs};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct IdSecret {
    pub id: String,
    pub secret: String,
}
#[derive(Debug)]
pub enum QsError {
    SerializationError(qs::Error),
    DeserializationError(qs::Error),
    InvalidData,
}
impl From<qs::Error> for QsError {
    fn from(err: qs::Error) -> Self {
        QsError::SerializationError(err)
    }
}
// Convert struct to query string
pub fn qs_stable_stringify<T: Serialize>(data: &T) -> Result<String, QsError> {
    qs::to_string(data).map_err(QsError::from)
}

pub fn qs_parse<T: for<'de> Deserialize<'de>>(query:&str) ->Result<T,QsError>{
    qs::from_str(query).map_err(QsError::from)
}

// Convert id secret to string
pub fn qs_id_secret(t: &IdSecret) -> Result<String, QsError> {
    qs_stable_stringify(t)
}

pub fn qs_id_secret_parse(encoded: Option<&str>) -> Option<IdSecret>{
    encoded.and_then(|s| qs_parse::<IdSecret>(s).ok()).and_then(|data|{
        if data.id.is_empty() || data.secret.is_empty(){
            None
        }else {
            Some(data)
        }
    })
}
