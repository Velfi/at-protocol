/// Input for the Create operation
pub struct CreateInput {
    pub handle: String,
    pub recoveryKey: String,
    pub inviteCode: String,
    pub password: String,
    pub email: String,
}
/// Input for the CreateInviteCode operation
pub struct CreateInviteCodeInput {
    pub useCount: u64,
}
/// Input for the Delete operation
pub struct DeleteInput {
    // this input has no fields
}
/// Input for the Get operation
pub struct GetInput {
    // this input has no fields
}
/// Input for the RequestPasswordReset operation
pub struct RequestPasswordResetInput {
    pub email: String,
}
/// Input for the ResetPassword operation
pub struct ResetPasswordInput {
    pub password: String,
    pub token: String,
}
/// Input for the Resolve operation
pub struct ResolveInput {
    // this input has no fields
}
/// Input for the BatchWrite operation
pub struct BatchWriteInput {
    // this input has no fields
}
/// Input for the CreateRecord operation
pub struct CreateRecordInput {
    // this input has no fields
}
/// Input for the DeleteRecord operation
pub struct DeleteRecordInput {
    /// The DID of the repo.
    pub did: String,
    /// The NSID of the record collection.
    pub collection: String,
    /// The key of the record.
    pub rkey: String,
}
/// Input for the Describe operation
pub struct DescribeInput {
    // this input has no fields
}
/// Input for the GetRecord operation
pub struct GetRecordInput {
    // this input has no fields
}
/// Input for the ListRecords operation
pub struct ListRecordsInput {
    // this input has no fields
}
/// Input for the PutRecord operation
pub struct PutRecordInput {
    // this input has no fields
}
/// Input for the GetAccountsConfig operation
pub struct GetAccountsConfigInput {
    // this input has no fields
}
/// Input for the Create operation
pub struct CreateInput {
    pub handle: String,
    pub password: String,
}
/// Input for the Delete operation
pub struct DeleteInput {
    // this input has no fields
}
/// Input for the Get operation
pub struct GetInput {
    // this input has no fields
}
/// Input for the Refresh operation
pub struct RefreshInput {
    // this input has no fields
}
/// Input for the GetRepo operation
pub struct GetRepoInput {
    // this input has no fields
}
/// Input for the GetRoot operation
pub struct GetRootInput {
    // this input has no fields
}
/// Input for the UpdateRepo operation
pub struct UpdateRepoInput {
    // this input has no fields
}
