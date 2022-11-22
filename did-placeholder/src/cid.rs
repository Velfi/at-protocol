/// A cryptographic ID
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Cid {
    pub id: String,
}

impl Cid {
    pub fn new(id: String) -> Self {
        Cid { id }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.id.as_bytes()
    }
}
