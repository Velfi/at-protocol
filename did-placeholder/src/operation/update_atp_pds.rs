use super::{Operation, ToBytes};
use crate::cid::Cid;
use std::fmt;

const OP_UPDATE_ATP_PDS: &str = "update_atp_pds";

#[derive(Debug, Clone, Hash)]
pub struct UpdateAtpPds {
    pub service: String,
    pub prev: Option<Cid>,
    pub sig: String,
}

impl UpdateAtpPds {
    pub fn builder() -> UpdateAtpPdsBuilder {
        UpdateAtpPdsBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateAtpPdsBuilder {
    pub service: Option<String>,
    pub prev: Option<Cid>,
    pub sig: Option<String>,
}

impl UpdateAtpPdsBuilder {
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

    pub fn build(self) -> Result<UpdateAtpPds, Error> {
        let service = self.service.ok_or(Error::missing_field("service"))?;
        let prev = self.prev;
        let sig = self.sig.ok_or(Error::missing_field("sig"))?;

        Ok(UpdateAtpPds { service, prev, sig })
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

impl ToBytes for UpdateAtpPds {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.service.as_bytes());
        if let Some(prev) = self.prev {
            bytes.extend_from_slice(prev.as_bytes());
        }
        bytes.extend_from_slice(self.sig.as_bytes());

        bytes
    }
}

impl Operation for UpdateAtpPds {
    fn r#type(&self) -> String {
        OP_UPDATE_ATP_PDS.to_owned()
    }

    fn prev(&self) -> Option<Cid> {
        self.prev.clone()
    }

    fn sig(&self) -> String {
        self.sig.clone()
    }
}
