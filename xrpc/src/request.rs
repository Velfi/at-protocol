pub enum Type {
    Query,
    Procedure,
}

pub struct XrpcRequest {
    _type: Type,
}

impl XrpcRequest {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Default)]
pub struct Builder {
    r#type: Option<Type>,
}

impl Builder {
    pub fn r#type(mut self, r#type: Type) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn set_type(&mut self, r#type: Option<Type>) -> &mut Self {
        self.r#type = r#type;
        self
    }

    pub fn build(self) -> Result<XrpcRequest, Error> {
        let r#type = self.r#type.ok_or(Error::MissingField("type"))?;

        Ok(XrpcRequest { _type: r#type })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing field `{0}`")]
    MissingField(&'static str),
}
