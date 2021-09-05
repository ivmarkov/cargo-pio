use std::convert::TryFrom;
use std::fs;

use anyhow::{anyhow, bail, Context, Error};
use serde::Deserialize;

use super::{index, ObjKind, Version};

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Cache {
    pub version: Version,
    pub entries: Vec<Entry>,
}

impl TryFrom<&index::Reply> for Cache {
    type Error = Error;
    fn try_from(value: &index::Reply) -> Result<Self, Self::Error> {
        if value.kind != ObjKind::Cache {
            bail!("reply is not a cache object");
        }
        if value.version.major != ObjKind::Cache.expected_major_version() {
            bail!("cache object version not supported");
        }

        Ok(
            serde_json::from_reader(&fs::File::open(&value.json_file)?).with_context(|| {
                anyhow!(
                    "Parsing cmake-file-api cache object file '{}' failed",
                    value.json_file.display()
                )
            })?,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Entry {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub entry_type: Type,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "String")]
pub enum Type {
    Bool,
    Path,
    Filepath,
    String,
    Internal,
    Static,
    Uninitialized,
    Other(String),
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        match s.as_str() {
            "BOOL" => Self::Bool,
            "PATH" => Self::Path,
            "FILEPATH" => Self::Filepath,
            "STRING" => Self::String,
            "INTERNAL" => Self::Internal,
            "STATIC" => Self::Static,
            "UNINITIALIZED" => Self::Uninitialized,
            _ => Self::Other(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE", tag = "name", content = "value")]
pub enum Property {
    Advanced(String),
    Helpstring(String),
    Modified(String),
    Strings(String),
    Type(Type),
    Value(String),
    #[serde(other)]
    Unknown,
}