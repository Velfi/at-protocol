
use crate::cid::Cid;
use crate::did_key::DidKey;
// TODO I want Operations to be `Hash` but that trait isn't object safe
// use std::hash::Hash;

pub trait Operation {
    /// operation type
    fn r#type(&self) -> String;
    /// pointer to the CID of the previous operation in the log
    fn prev(&self) -> Option<Cid>;
    /// base64url encoded signature of the operation
    fn sig(&self) -> String;
}

pub struct OperationLog {
    pub log: Vec<Box<dyn Operation>>,
}

pub const OP_CREATE: &str = "create";

pub struct Create {
    pub r#type: &'static str,
    pub signing_key: String,
    pub recovery_key: String,
    pub username: String,
    pub service: String,
    pub prev: Option<Cid>,
    pub sig: String,
}

pub const OP_UPDATE_USERNAME: &str = "update_username";

pub struct UpdateUsername {
    pub r#type: &'static str,
    pub username: String,
    pub prev: Option<Cid>,
    pub sig: String,
}

pub const OP_UPDATE_ATP_PDS: &str = "update_atp_pds";

pub struct UpdateAtpPds {
    pub r#type: &'static str,
    pub service: String,
    pub prev: Option<Cid>,
    pub sig: String,
}

pub const OP_ROTATE_SIGNING_KEY: &str = "rotate_signing_key";

pub struct RotateSigningKey {
    pub r#type: &'static str,
    pub key: DidKey,
    pub prev: Option<Cid>,
    pub sig: String,
}

pub const OP_ROTATE_RECOVERY_KEY: &str = "rotate_recovery_key";

pub struct RotateRecoveryKey {
    pub r#type: &'static str,
    pub key: DidKey,
    pub prev: Option<Cid>,
    pub sig: String,
}
