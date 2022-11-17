use crate::operation::OperationLog;

#[non_exhaustive]
struct Document {
    signing_key: (),
    recovery_key: (),
    username: (),
    /// Personal Data Server for the related AT Protocol repository
    atp_pds: (),
}

impl Document {
    pub fn from_operation_log(operation_log: &OperationLog) -> Self {
        todo!()
    }
}
