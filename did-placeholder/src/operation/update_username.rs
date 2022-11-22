use crate::cid::Cid;
use super::Operation;

const OP_UPDATE_USERNAME: &str = "update_username";

#[derive(Debug, Clone, Hash)]
pub struct UpdateUsername {
    pub r#type: &'static str,
    pub username: String,
    pub prev: Option<Cid>,
    pub sig: String,
}
