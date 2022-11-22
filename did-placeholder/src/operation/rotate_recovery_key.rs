use crate::cid::Cid;
use crate::did_key::DidKey;
use super::Operation;

const OP_ROTATE_RECOVERY_KEY: &str = "rotate_recovery_key";

#[derive(Debug, Clone, Hash)]
pub struct RotateRecoveryKey {
    pub r#type: &'static str,
    pub key: DidKey,
    pub prev: Option<Cid>,
    pub sig: String,
}
