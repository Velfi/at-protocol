use sha2::{Sha256, Digest};

use crate::operation::{Operation};

/// A cryptographic ID
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Cid {
    id: String,
}

impl Cid {
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.id.as_bytes()
    }
}

impl<T> From<T> for Cid 
where
    T: Operation,
{
    fn from(op: T) -> Self {
        // get the sha256 hash of the operation, base32 encoded and truncated to 24 characters
        // as per https://atproto.com/specs/did-plc#how-it-works
        let bytes = op.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let id = hex::encode(result);
        Cid { id }
    }
}
