use std::hash::Hash;

use crate::cid::Cid;
use crate::did_key::DidKey;

// TODO I want Operations to be `Hash` but that trait isn't object safe

pub trait Operation {
    /// operation type
    fn r#type(&self) -> String;
    /// pointer to the CID of the previous operation in the log
    fn prev(&self) -> Option<Cid>;
    /// base64url encoded signature of the operation
    fn sig(&self) -> String;
}

pub struct OperationLog {
    log: Vec<Box<dyn Operation>>,
}

const OP_CREATE: &str = "create";

pub struct Create {
    r#type: &'static str,
    signing_key: String,
    recovery_key: String,
    username: String,
    service: String,
    prev: Option<Cid>,
    sig: String,
}

pub const OP_UPDATE_USERNAME: &str = "update_username";

struct UpdateUsername {
    r#type: &'static str,
    username: String,
    prev: Option<Cid>,
    sig: String,
}

pub const OP_UPDATE_ATP_PDS: &str = "update_atp_pds";

struct UpdateAtpPds {
    r#type: &'static str,
    service: String,
    prev: Option<Cid>,
    sig: String,
}

pub const OP_ROTATE_SIGNING_KEY: &str = "rotate_signing_key";

struct RotateSigningKey {
    r#type: &'static str,
    key: DidKey,
    prev: Option<Cid>,
    sig: String,
}

pub const OP_ROTATE_RECOVERY_KEY: &str = "rotate_recovery_key";

struct RotateRecoveryKey {
    r#type: &'static str,
    key: DidKey,
    prev: Option<Cid>,
    sig: String,
}
