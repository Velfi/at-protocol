use crate::cid::Cid;
use super::Operation;

const OP_UPDATE_ATP_PDS: &str = "update_atp_pds";

#[derive(Debug, Clone, Hash)]
pub struct UpdateAtpPds {
    pub r#type: &'static str,
    pub service: String,
    pub prev: Option<Cid>,
    pub sig: String,
}