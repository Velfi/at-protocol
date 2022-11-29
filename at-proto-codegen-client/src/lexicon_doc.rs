pub mod io;

use serde_json::Value;
use thiserror::Error;
use tracing::{debug, error, warn};
use xrpc::Nsid;

pub enum LexiconDocType {
    Query,
    Procedure,
    Record,
}

impl TryFrom<&str> for LexiconDocType {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "query" => Ok(LexiconDocType::Query),
            "procedure" => Ok(LexiconDocType::Procedure),
            "record" => Ok(LexiconDocType::Record),
            _ => Err(anyhow::anyhow!("invalid lexicon doc type: {}", s)),
        }
    }
}

pub enum LexiconVersion {
    V1,
    Unknown(u64),
}

impl From<u64> for LexiconVersion {
    fn from(version: u64) -> Self {
        match version {
            1 => LexiconVersion::V1,
            _ => LexiconVersion::Unknown(version),
        }
    }
}

#[derive(Default)]
struct Builder {
    lexicon: Option<LexiconVersion>,
    id: Option<String>,
    r#type: Option<LexiconDocType>,
    revision: Option<u64>,
    description: Option<String>,
    input: Option<io::Input>,
    output: Option<io::Output>,
    error: Option<io::Error>,
}

impl Builder {
    fn new() -> Self {
        Self::default()
    }

    fn lexicon(mut self, lexicon: Option<LexiconVersion>) -> Self {
        self.lexicon = lexicon;
        self
    }

    fn id(mut self, id: Option<String>) -> Self {
        self.id = id;
        self
    }

    fn r#type(mut self, r#type: Option<LexiconDocType>) -> Self {
        self.r#type = r#type;
        self
    }

    fn revision(mut self, revision: Option<u64>) -> Self {
        self.revision = revision;
        self
    }

    fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    fn input(mut self, input: Option<io::Input>) -> Self {
        self.input = input;
        self
    }

    fn output(mut self, output: Option<io::Output>) -> Self {
        self.output = output;
        self
    }

    fn error(mut self, error: Option<io::Error>) -> Self {
        self.error = error;
        self
    }

    fn build(self) -> Result<LexiconDoc, LexiconDocError> {
        let lexicon = self
            .lexicon
            .ok_or(LexiconDocError::MissingField("lexicon"))?;
        let id = self.id.ok_or(LexiconDocError::MissingField("id"))?;
        let id = id
            .parse::<Nsid>()
            .map_err(|_| LexiconDocError::InvalidField {
                field: "id",
                value: id,
            })?;
        let r#type = self.r#type.ok_or(LexiconDocError::MissingField("type"))?;

        Ok(LexiconDoc {
            lexicon,
            id,
            r#type,
            revision: self.revision,
            description: self.description,
            input: self.input,
            output: self.output,
            error: self.error,
        })
    }
}

#[allow(dead_code)]
pub struct LexiconDoc {
    lexicon: LexiconVersion,
    id: Nsid,
    r#type: LexiconDocType,
    revision: Option<u64>,
    description: Option<String>,
    input: Option<io::Input>,
    output: Option<io::Output>,
    error: Option<io::Error>,
}

#[allow(dead_code)]
impl LexiconDoc {
    pub fn lexicon(&self) -> &LexiconVersion {
        &self.lexicon
    }

    pub fn id(&self) -> &Nsid {
        &self.id
    }

    pub fn r#type(&self) -> &LexiconDocType {
        &self.r#type
    }

    pub fn revision(&self) -> Option<u64> {
        self.revision
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn input(&self) -> Option<&io::Input> {
        self.input.as_ref()
    }

    pub fn output(&self) -> Option<&io::Output> {
        self.output.as_ref()
    }

    pub fn error(&self) -> Option<&io::Error> {
        self.error.as_ref()
    }

    pub fn from_json(json: &serde_json::Value) -> Result<LexiconDoc, LexiconDocError> {
        let lexicon: Option<LexiconVersion> =
            json.get("lexicon").and_then(Value::as_u64).map(Into::into);
        let id = json
            .get("id")
            .and_then(Value::as_str)
            .map(Into::into)
            .unwrap_or_default();

        debug!("creating lexicon doc for {}", id);

        let r#type: Option<LexiconDocType> =
            json.get("type").and_then(Value::as_str).and_then(|v| {
                match LexiconDocType::try_from(v) {
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                    v => v.ok(),
                }
            });
        let revision = json.get("revision").and_then(Value::as_u64);
        let description = json
            .get("description")
            .and_then(Value::as_str)
            .map(Into::into);
        let input = json.get("input").and_then(Value::as_object).and_then(|v| {
            match io::Input::try_from(v) {
                Err(e) => {
                    warn!("{e}, returning `None` for {id}.input");
                    None
                }
                v => v.ok(),
            }
        });
        let output =
            json.get("output").and_then(Value::as_object).and_then(
                |v| match io::Output::try_from(v) {
                    Err(e) => {
                        warn!("{e}, returning `None` for {id}.output");
                        None
                    }
                    v => v.ok(),
                },
            );
        let error = json.get("error").and_then(Value::as_array).and_then(|v| {
            match io::Error::try_from(v) {
                Err(e) => {
                    warn!("{e}, returning `None` for {id}.error");
                    None
                }
                v => v.ok(),
            }
        });

        Builder::new()
            .lexicon(lexicon)
            .id(Some(id))
            .r#type(r#type)
            .revision(revision)
            .description(description)
            .input(input)
            .output(output)
            .error(error)
            .build()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexiconDocError {
    #[error("The Lexicon document is missing the required field {0}")]
    MissingField(&'static str),
    #[error("The Lexicon document has an invalid {field} field with value {value}")]
    InvalidField { field: &'static str, value: String },
}
