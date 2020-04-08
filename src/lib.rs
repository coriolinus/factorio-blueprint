use objects::{Blueprint, BlueprintBook};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use thiserror::Error;
use version_prefix::{VersionPrefixReader, VersionPrefixWriter};

pub mod objects;
pub mod version_prefix;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Container {
    BlueprintBook(BlueprintBook),
    Blueprint(Blueprint),
}

impl From<BlueprintBook> for Container {
    fn from(b: BlueprintBook) -> Container {
        Container::BlueprintBook(b)
    }
}

impl From<Blueprint> for Container {
    fn from(b: Blueprint) -> Container {
        Container::Blueprint(b)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("json problem")]
    Json(#[from] serde_json::Error),
    #[error("failed to write valid utf8")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("io troubles; probably transient")]
    Io(#[from] std::io::Error),
    #[error("unexpected blueprint string version byte")]
    UnknownVersion,
    #[error("failed to read any data")]
    NoData,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Container {
    /// write the blueprint string to the given writer
    pub fn write_blueprint<W: Write>(&self, writer: W) -> Result<()> {
        // the final step before sending the data out is to prepend a 0.
        let mut writer = VersionPrefixWriter::new('0', writer);
        // before we prepend that 0, we need to base64-encode the stream
        let mut writer = base64::write::EncoderWriter::new(&mut writer, base64::STANDARD);
        // note: we can't just hand this off, because we'll need to call its
        // `finish` method later
        {
            // before we base64 it, we should compress it
            use flate2::{write::DeflateEncoder, Compression};
            let writer = DeflateEncoder::new(writer.by_ref(), Compression::new(9));
            // actually write this struct to the stream
            serde_json::to_writer(writer, self)?;
        }
        writer.finish().map_err(|e| e.into())
    }

    /// produce a new owned string containing the blueprint string
    pub fn blueprint(&self) -> Result<String> {
        let mut out = Vec::new();
        self.write_blueprint(&mut out)?;
        Ok(String::from_utf8(out)?)
    }

    /// read the blueprint string from the given reader
    pub fn read_blueprint<R: Read>(reader: R) -> Result<Self> {
        // we'll need this later
        let out;
        // the first step is to take off the initial byte and check it
        let mut reader = VersionPrefixReader::new('0', reader);
        // note: we can't just hand this off, because we'll need to call its
        // `had_expected_version` method later
        {
            let reader = base64::read::DecoderReader::new(reader.by_ref(), base64::STANDARD);
            // decompress it
            let reader = flate2::read::DeflateDecoder::new(reader);
            // parse the json
            out = serde_json::from_reader(reader)?;
        }
        if !reader.had_expected_version().ok_or(Error::NoData)? {
            Err(Error::UnknownVersion)?;
        }
        Ok(out)
    }

    /// read the blueprint string from the given input
    pub fn parse(blueprint: &str) -> Result<Self> {
        Self::read_blueprint(blueprint.as_bytes())
    }
}
