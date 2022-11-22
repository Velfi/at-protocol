use std::fmt;

#[cfg(feature = "serde_json")]
use serde_json::{Map, Value};

pub enum Type {
    String,
    Number,
    Integer,
    Boolean,
}

impl Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Number => write!(f, "u64"),
            Type::Integer => write!(f, "i64"),
            Type::Boolean => write!(f, "bool"),
        }
    }
}

#[cfg(feature = "serde_json")]
impl TryFrom<&Value> for Type {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.as_str() {
            Some("string") => Ok(Self::String),
            Some("number") => Ok(Self::Number),
            Some("integer") => Ok(Self::Integer),
            Some("boolean") => Ok(Self::Boolean),
            _ => Err(Error::InvalidField("type")),
        }
    }
}

pub enum Default {
    String(String),
    Boolean(bool),
    Number(u64),
}

#[cfg(feature = "serde_json")]
impl TryFrom<&Value> for Default {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(Self::String(s.to_string())),
            Value::Bool(b) => Ok(Self::Boolean(*b)),
            Value::Number(n) => {
                if let Some(i) = n.as_u64() {
                    Ok(Self::Number(i))
                } else {
                    Err(Error::InvalidField("default"))
                }
            }
            _ => Err(Error::InvalidField("default")),
        }
    }
}

pub struct Parameter {
    pub r#type: Type,
    pub description: Option<String>,
    pub default: Option<Default>,
    pub required: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub minimum: Option<u64>,
    pub maximum: Option<u64>,
}

#[cfg(feature = "serde_json")]
impl TryFrom<&Map<String, Value>> for Parameter {
    type Error = Error;

    fn try_from(value: &Map<String, Value>) -> Result<Self, Self::Error> {
        let r#type = value
            .get("type")
            .ok_or(Error::MissingField("type"))?
            .try_into()?;

        let description = value
            .get("description")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);

        // TODO write this in a more readable way
        let default = if let Some(v) = value.get("default") {
            Some(v.try_into()?)
        } else {
            None
        };

        let required = value.get("required").and_then(Value::as_bool);

        let min_length = value.get("minLength").and_then(Value::as_u64);

        let max_length = value.get("maxLength").and_then(Value::as_u64);

        let minimum = value.get("minimum").and_then(Value::as_u64);

        let maximum = value.get("maximum").and_then(Value::as_u64);

        Ok(Self {
            r#type,
            description,
            default,
            required,
            min_length,
            max_length,
            minimum,
            maximum,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing field '{0}'")]
    MissingField(&'static str),
    #[error("invalid field '{0}'")]
    InvalidField(&'static str),
}
