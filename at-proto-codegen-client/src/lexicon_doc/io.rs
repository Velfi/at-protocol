use anyhow::Context;
use serde_json::value::{Map, Value};
use std::collections::{HashMap, HashSet};

type JsonMap = Map<String, Value>;
type JsonArray = Vec<Value>;

pub struct IoSchema {
    r#type: String,
    required: HashSet<String>,
    properties: HashMap<String, xrpc::Parameter>,
}

impl TryFrom<&JsonMap> for IoSchema {
    type Error = anyhow::Error;

    fn try_from(value: &JsonMap) -> Result<Self, Self::Error> {
        let r#type = value
            .get("type")
            .ok_or_else(|| anyhow::anyhow!("missing field 'type'"))?
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'type'"))?
            .to_string();

        let required = value
            .get("required")
            .ok_or_else(|| anyhow::anyhow!("missing field 'required'"))?
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'required'"))?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(ToOwned::to_owned)
                    .ok_or_else(|| anyhow::anyhow!("invalid 'required' value '{}'", v))
            })
            .collect::<Result<HashSet<String>, anyhow::Error>>()?;

        let properties = value
            .get("properties")
            .ok_or_else(|| anyhow::anyhow!("missing field 'properties'"))?
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'properties'"))?
            .iter()
            .map(|(k, v)| {
                v.as_object()
                    .ok_or_else(|| anyhow::anyhow!("invalid field 'properties.{}'", k))
                    .and_then(|v| {
                        xrpc::Parameter::try_from(v)
                            .map(|v| (k.to_string(), v))
                            .context("couldn't create XRPC parameter from JSON")
                    })
            })
            .collect::<Result<HashMap<String, xrpc::Parameter>, anyhow::Error>>()?;

        Ok(Self {
            r#type,
            required,
            properties,
        })
    }
}

pub struct Input {
    pub description: Option<String>,
    pub encoding: mime::Mime,
    pub schema: IoSchema,
}

impl Input {
    pub fn required(&self) -> &HashSet<String> {
        &self.schema.required
    }

    pub fn properties(&self) -> &HashMap<String, xrpc::Parameter> {
        &self.schema.properties
    }
}

impl TryFrom<&JsonMap> for Input {
    type Error = anyhow::Error;

    fn try_from(value: &JsonMap) -> Result<Self, Self::Error> {
        let description = value
            .get("description")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);

        let encoding = value
            .get("encoding")
            .ok_or_else(|| anyhow::anyhow!("missing field 'encoding'"))?
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'encoding'"))?
            .parse::<mime::Mime>()
            .map_err(|e| anyhow::anyhow!("invalid field 'encoding': {}", e))?;

        let schema = value
            .get("schema")
            .ok_or_else(|| anyhow::anyhow!("missing field 'schema'"))?
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'schema'"))?
            .try_into()?;

        Ok(Self {
            description,
            encoding,
            schema,
        })
    }
}

pub struct Output {
    pub description: Option<String>,
    pub encoding: mime::Mime,
    pub schema: IoSchema,
}

impl Output {
    pub fn required(&self) -> &HashSet<String> {
        &self.schema.required
    }

    pub fn properties(&self) -> &HashMap<String, xrpc::Parameter> {
        &self.schema.properties
    }
}

impl TryFrom<&JsonMap> for Output {
    type Error = anyhow::Error;

    fn try_from(value: &JsonMap) -> Result<Self, Self::Error> {
        let description = value
            .get("description")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);

        let encoding = value
            .get("encoding")
            .ok_or_else(|| anyhow::anyhow!("missing field 'encoding'"))?
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'encoding'"))?
            .parse::<mime::Mime>()
            .map_err(|e| anyhow::anyhow!("invalid field 'encoding': {}", e))?;

        let schema = value
            .get("schema")
            .ok_or_else(|| anyhow::anyhow!("missing field 'schema'"))?
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'schema'"))?
            .try_into()?;

        Ok(Self {
            description,
            encoding,
            schema,
        })
    }
}

pub struct Error {
    variants: Vec<ErrorVariant>,
}

impl TryFrom<&JsonArray> for Error {
    type Error = anyhow::Error;

    fn try_from(value: &JsonArray) -> Result<Self, Self::Error> {
        let variants = value
            .iter()
            .map(|v| {
                v.as_object()
                    .ok_or_else(|| anyhow::anyhow!("invalid error variant '{}'", v))
                    .and_then(|v| ErrorVariant::try_from(v))
            })
            .collect::<Result<Vec<ErrorVariant>, anyhow::Error>>()?;

        Ok(Self { variants })
    }
}

pub struct ErrorVariant {
    name: String,
}

impl TryFrom<&JsonMap> for ErrorVariant {
    type Error = anyhow::Error;

    fn try_from(value: &JsonMap) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .ok_or_else(|| anyhow::anyhow!("missing field 'name'"))?
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("invalid field 'name'"))?
            .to_string();

        Ok(Self { name })
    }
}
