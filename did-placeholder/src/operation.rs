mod create;
mod rotate_recovery_key;
mod rotate_signing_key;
mod update_atp_pds;
mod update_username;

pub use create::Create;
pub use rotate_recovery_key::RotateRecoveryKey;
pub use rotate_signing_key::RotateSigningKey;
pub use update_atp_pds::UpdateAtpPds;
pub use update_username::UpdateUsername;

use crate::cid::Cid;

pub trait Operation {
    /// operation type
    fn r#type(&self) -> String;
    /// pointer to the CID of the previous operation in the log
    fn prev(&self) -> Option<Cid>;
    /// base64url encoded signature of the operation
    fn sig(&self) -> String;
    /// get the sha256 hash of the operation, base32 encoded and truncated to 24 characters
    /// as per https://atproto.com/specs/did-plc#how-it-works
    fn hash(self) -> String;
}

pub struct OperationLog {
    pub log: Vec<Box<dyn Operation>>,
}
