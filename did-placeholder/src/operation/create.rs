use sha2::{Sha256, Digest};
use std::fmt;
use crate::cid::Cid;
use super::Operation;

const OP_CREATE: &str = "create";

#[derive(Debug, Clone, Hash)]
pub struct Create {
    signing_key: String,
    recovery_key: String,
    username: String,
    service: String,
    prev: Option<Cid>,
    sig: String,
}

impl Create {
    pub fn builder() -> CreateBuilder {
        CreateBuilder::default()
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.signing_key.as_bytes());
        bytes.extend_from_slice(self.recovery_key.as_bytes());
        bytes.extend_from_slice(self.username.as_bytes());
        bytes.extend_from_slice(self.service.as_bytes());
        if let Some(prev) = self.prev {
            bytes.extend_from_slice(prev.as_bytes());
        }
        bytes
    }
}

#[derive(Default)]
pub struct CreateBuilder {
    pub signing_key: Option<String>,
    pub recovery_key: Option<String>,
    pub username: Option<String>,
    pub service: Option<String>,
    pub prev: Option<Cid>,
    pub sig: Option<String>,
}

impl CreateBuilder {
    pub fn signing_key(mut self, signing_key: String) -> Self {
        self.signing_key = Some(signing_key);
        self
    }

    pub fn recovery_key(mut self, recovery_key: String) -> Self {
        self.recovery_key = Some(recovery_key);
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn service(mut self, service: String) -> Self {
        self.service = Some(service);
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

    pub fn build(self) -> Result<Create, Error> {
        let signing_key = self.signing_key.ok_or(Error::missing_field("signing_key"))?;
        let recovery_key = self.recovery_key.ok_or(Error::missing_field("recovery_key"))?;
        let username = self.username.ok_or(Error::missing_field("username"))?;
        let service = self.service.ok_or(Error::missing_field("service"))?;
        let prev = self.prev;
        let sig = self.sig.ok_or(Error::missing_field("sig"))?;

        Ok(Create {
            signing_key,
            recovery_key,
            username,
            service,
            prev,
            sig,
        })
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

impl Operation for Create {
    fn r#type(&self) -> String {
        OP_CREATE.to_owned()
    }

    fn prev(&self) -> Option<Cid> {
        self.prev.clone()
    }

    fn sig(&self) -> String {
        self.sig.clone()
    }

    fn hash(self) -> String {
        let mut hasher = Sha256::new(); 
        let bytes = self.to_bytes();
        hasher.update(bytes);
        let res = hasher.finalize();

        hex::encode(res)
    }
}