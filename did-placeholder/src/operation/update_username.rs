use super::{Operation, ToBytes};
use crate::cid::Cid;
use std::fmt;

const OP_UPDATE_USERNAME: &str = "update_username";

#[derive(Debug, Clone, Hash)]
pub struct UpdateUsername {
    pub username: String,
    pub prev: Option<Cid>,
    pub sig: String,
}

impl UpdateUsername {
    pub fn builder() -> UpdateUsernameBuilder {
        UpdateUsernameBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateUsernameBuilder {
    pub username: Option<String>,
    pub prev: Option<Cid>,
    pub sig: Option<String>,
}

impl UpdateUsernameBuilder {
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
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

    pub fn build(self) -> Result<UpdateUsername, Error> {
        let username = self.username.ok_or(Error::missing_field("username"))?;
        let prev = self.prev;
        let sig = self.sig.ok_or(Error::missing_field("sig"))?;

        Ok(UpdateUsername {
            username,
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

impl ToBytes for UpdateUsername {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.username.as_bytes());
        if let Some(prev) = self.prev {
            bytes.extend_from_slice(prev.as_bytes());
        }
        bytes.extend_from_slice(self.sig.as_bytes());

        bytes
    }
}

impl Operation for UpdateUsername {
    fn r#type(&self) -> String {
        OP_UPDATE_USERNAME.to_owned()
    }

    fn prev(&self) -> Option<Cid> {
        self.prev.clone()
    }

    fn sig(&self) -> String {
        self.sig.clone()
    }
}
