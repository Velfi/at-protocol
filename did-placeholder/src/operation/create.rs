use super::{Operation, ToBytes};
use crate::cid::Cid;
use std::fmt;

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
    pub fn cid(&self) -> Cid {
        self.clone().into()
    }

    pub fn prev(&self) -> Option<Cid> {
        self.prev.clone()
    }

    pub fn builder() -> CreateBuilder {
        CreateBuilder::default()
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
        let signing_key = self
            .signing_key
            .ok_or(Error::missing_field("signing_key"))?;
        let recovery_key = self
            .recovery_key
            .ok_or(Error::missing_field("recovery_key"))?;
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

impl ToBytes for Create {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.signing_key.as_bytes());
        bytes.extend_from_slice(self.recovery_key.as_bytes());
        bytes.extend_from_slice(self.username.as_bytes());
        bytes.extend_from_slice(self.service.as_bytes());
        if let Some(prev) = self.prev {
            bytes.extend_from_slice(prev.as_bytes());
        }
        bytes.extend_from_slice(self.sig.as_bytes());

        bytes
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
}

// impl DidKey {
//     /// Create a new DID key from a [Create] operation as per [the standard](https://atproto.com/specs/did-plc#how-it-works)
//     pub fn new(create_operation: Create) -> Self {
//         let cid = create_operation.cid();
//         let mut encoded_cid = base32::encode(Alphabet::RFC4648 { padding: true }, cid.as_bytes());
//         encoded_cid.truncate(24);

//         DidKey { key: format!("did:plc:{encoded_cid}") }
//     }

//     pub fn as_str(&self) -> &str {
//         self.key.as_str()
//     }

//     pub fn to_bytes(&self) -> Vec<u8> {
//         hex::decode(self.key).expect("key will always be valid hex")
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::operation::Create;

//     use super::DidKey;

//     #[test]
//     fn snapshot_test() {
//         let create_op = Create::builder()
//             .
//             .build()
//             .expect("is a valid create operation");

//         let did_key = DidKey::new(create_op);
//         let expected = "";
//         let actual = did_key.as_str();

//         assert_eq!(expected, actual);
//     }
// }
