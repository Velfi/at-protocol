use super::{Operation, ToBytes};
use crate::cid::Cid;
use crate::did_key::DidKey;
use std::fmt;

const OP_ROTATE_RECOVERY_KEY: &str = "rotate_recovery_key";

#[derive(Debug, Clone, Hash)]
pub struct RotateRecoveryKey {
    pub key: DidKey,
    pub prev: Option<Cid>,
    pub sig: String,
}

impl RotateRecoveryKey {
    pub fn builder() -> RotateRecoveryKeyBuilder {
        RotateRecoveryKeyBuilder::default()
    }
}

#[derive(Default)]
pub struct RotateRecoveryKeyBuilder {
    pub key: Option<DidKey>,
    pub prev: Option<Cid>,
    pub sig: Option<String>,
}

impl RotateRecoveryKeyBuilder {
    pub fn key(mut self, key: DidKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn prev(mut self, prev: Cid) -> Self {
        self.prev = Some(prev);
        self
    }

    pub fn sig(mut self, sig: String) -> Self {
        self.sig = Some(sig);
        self
    }

    pub fn build(self) -> Result<RotateRecoveryKey, Error> {
        let key = self.key.ok_or(Error::missing_field("key"))?;
        let prev = self.prev;
        let sig = self.sig.ok_or(Error::missing_field("sig"))?;

        Ok(RotateRecoveryKey { key, prev, sig })
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

impl Error {
    fn missing_field(field_name: &'static str) -> Self {
        Self(ErrorRepr::MissingField(field_name))
    }
}

#[derive(thiserror::Error, Debug)]
enum ErrorRepr {
    MissingField(&'static str),
}

impl fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorRepr::MissingField(field_name) => write!(f, "missing required field {field_name}"),
        }
    }
}

impl ToBytes for RotateRecoveryKey {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.key.as_bytes());
        if let Some(prev) = self.prev {
            bytes.extend_from_slice(prev.as_bytes());
        }
        bytes.extend_from_slice(self.sig.as_bytes());

        bytes
    }
}

impl Operation for RotateRecoveryKey {
    fn r#type(&self) -> String {
        OP_ROTATE_RECOVERY_KEY.to_owned()
    }

    fn prev(&self) -> Option<Cid> {
        self.prev.clone()
    }

    fn sig(&self) -> String {
        self.sig.clone()
    }
}
