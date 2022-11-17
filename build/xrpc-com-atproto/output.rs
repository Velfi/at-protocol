/// Output for the Create operation
pub struct CreateOutput {
    pub handle: String,
    pub did: String,
    pub accessJwt: String,
    pub refreshJwt: String,
}
/// Output for the CreateInviteCode operation
pub struct CreateInviteCodeOutput {
    pub code: String,
}
/// Output for the Delete operation
pub struct DeleteOutput {
    // this output has no fields
}
/// Output for the Get operation
pub struct GetOutput {
    // this output has no fields
}
/// Output for the RequestPasswordReset operation
pub struct RequestPasswordResetOutput {
    // this output has no fields
}
/// Output for the ResetPassword operation
pub struct ResetPasswordOutput {
    // this output has no fields
}
/// Output for the Resolve operation
pub struct ResolveOutput {
    pub did: String,
}
/// Output for the BatchWrite operation
pub struct BatchWriteOutput {
    // this output has no fields
}
/// Output for the CreateRecord operation
pub struct CreateRecordOutput {
    pub uri: String,
    pub cid: String,
}
/// Output for the DeleteRecord operation
pub struct DeleteRecordOutput {
    // this output has no fields
}
/// Output for the Describe operation
pub struct DescribeOutput {
    // this output has no fields
}
/// Output for the GetRecord operation
pub struct GetRecordOutput {
    // this output has no fields
}
/// Output for the ListRecords operation
pub struct ListRecordsOutput {
    // this output has no fields
}
/// Output for the PutRecord operation
pub struct PutRecordOutput {
    pub uri: String,
    pub cid: String,
}
/// Output for the GetAccountsConfig operation
pub struct GetAccountsConfigOutput {
    // this output has no fields
}
/// Output for the Create operation
pub struct CreateOutput {
    pub refreshJwt: String,
    pub did: String,
    pub handle: String,
    pub accessJwt: String,
}
/// Output for the Delete operation
pub struct DeleteOutput {
    // this output has no fields
}
/// Output for the Get operation
pub struct GetOutput {
    pub handle: String,
    pub did: String,
}
/// Output for the Refresh operation
pub struct RefreshOutput {
    pub refreshJwt: String,
    pub did: String,
    pub accessJwt: String,
    pub handle: String,
}
/// Output for the GetRepo operation
pub struct GetRepoOutput {
    // this output has no fields
}
/// Output for the GetRoot operation
pub struct GetRootOutput {
    pub root: String,
}
/// Output for the UpdateRepo operation
pub struct UpdateRepoOutput {
    // this output has no fields
}
