use crate::cid::Cid;
use crate::did_key::DidKey;
use super::Operation;

const OP_ROTATE_SIGNING_KEY: &str = "rotate_signing_key";

#[derive(Debug, Clone, Hash)]
pub struct RotateSigningKey {
    pub r#type: &'static str,
    pub key: DidKey,
    pub prev: Option<Cid>,
    pub sig: String,
}
