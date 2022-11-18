pub mod parameter;
pub mod request;
pub mod response;

pub use parameter::Parameter;

pub enum XrpcBodyEncoding {
    String(String),
    StringList(Vec<String>),
}

pub struct XrpcBody {
    pub encoding: XrpcBodyEncoding,
    pub schema: (),
    pub description: Option<String>,
}

pub struct XrpcError {
    pub name: String,
    pub description: Option<String>,
}
