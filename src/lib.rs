use base64::read::DecoderReader as Base64Decoder;
use flate2::read::ZlibDecoder;
use objects::{Blueprint, BlueprintBook};
use objwalk::Objwalk;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::io::prelude::*;
use thiserror::Error;
use version_prefix::{VersionPrefixReader, VersionPrefixWriter};

pub mod objects;
pub mod objwalk;
pub mod version_prefix;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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

impl Objwalk for Container {
    fn walk_structure<F>(&self, visit: F)
    where
        F: Fn(&dyn Any),
    {
        match self {
            Self::BlueprintBook(bb) => {
                visit(bb);
                bb.walk_structure(visit)
            }
            Self::Blueprint(b) => {
                visit(b);
                b.walk_structure(visit)
            }
        }
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

pub struct BlueprintCodec;

impl BlueprintCodec {
    /// write the blueprint string to the given writer
    pub fn encode<W: Write>(writer: W, container: &Container) -> Result<()> {
        // the final step before sending the data out is to prepend a 0.
        let mut writer = VersionPrefixWriter::new('0', writer);
        // before we prepend that 0, we need to base64-encode the stream
        let mut writer = base64::write::EncoderWriter::new(&mut writer, base64::STANDARD);
        // note: we can't just hand this off, because we'll need to call its
        // `finish` method later
        {
            // before we base64 it, we should compress it
            use flate2::{write::ZlibEncoder, Compression};
            let writer = ZlibEncoder::new(writer.by_ref(), Compression::new(9));
            // actually write this struct to the stream
            serde_json::to_writer(writer, container)?;
        }
        writer.finish().map_err(|e| e.into())
    }

    /// produce a new owned string containing the blueprint string
    pub fn encode_string(container: &Container) -> Result<String> {
        let mut out = Vec::new();
        Self::encode(&mut out, container)?;
        Ok(String::from_utf8(out)?)
    }

    /// read the blueprint string from the given reader
    pub fn decode<R: Read>(reader: R) -> Result<Container> {
        let mut out = Err(Error::NoData);
        Self::decode_to_reader(reader, |reader| {
            out = serde_json::from_reader(reader).map_err(|e| e.into())
        })?;
        out
    }

    /// decode the blueprint string (`reader`) into a json stream
    ///
    /// the json stream is provided as the argument to the inner function
    ///
    /// typically it is more useful to use `decode` instead, but this method
    /// gives flexibility in the event that it is required
    pub fn decode_to_reader<R, F>(reader: R, inner: F) -> Result<()>
    where
        R: Read,
        F: FnOnce(ZlibDecoder<Base64Decoder<VersionPrefixReader<R>>>),
    {
        // the first step is to take off the initial byte and check it
        let mut reader = VersionPrefixReader::new('0', reader);
        // note: we can't just hand this off, because we'll need to call its
        // `had_expected_version` method later
        {
            // decode base64
            let reader = Base64Decoder::new(reader.by_ref(), base64::STANDARD);
            // decompress it
            let reader = ZlibDecoder::new(reader);
            // hand it off to the inner closure
            inner(reader);
        }
        if !reader.had_expected_version().ok_or(Error::NoData)? {
            Err(Error::UnknownVersion)?;
        }
        Ok(())
    }

    /// read the blueprint string from the given input
    pub fn decode_string(blueprint: &str) -> Result<Container> {
        Self::decode(blueprint.as_bytes())
    }
}
