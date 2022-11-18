use crate::operation::OperationLog;

#[non_exhaustive]
pub struct Document {
    pub signing_key: (),
    pub recovery_key: (),
    pub username: (),
    /// Personal Data Server for the related AT Protocol repository
    pub atp_pds: (),
}

impl Document {
    pub fn from_operation_log(_operation_log: &OperationLog) -> Self {
        todo!()
    }
}
