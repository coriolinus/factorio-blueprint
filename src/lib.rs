use base64::{read::DecoderReader as Base64Decoder, write::EncoderWriter as Base64Encoder};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use objects::{Blueprint, BlueprintBook, DeconstructionPlanner, UpgradePlanner};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use thiserror::Error;
use version_prefix::{VersionPrefixReader, VersionPrefixWriter};
use whitespace_remover::WhitespaceRemover;

pub mod objects;
pub mod version_prefix;
pub mod whitespace_remover;

/// `Container`s are the primary entry point for this library: they contain
/// either a single blueprint, or a blueprint book.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Container {
    BlueprintBook(BlueprintBook),
    Blueprint(Blueprint),
    DeconstructionPlanner(DeconstructionPlanner),
    UpgradePlanner(UpgradePlanner),
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

impl Container {
    pub fn decode<R: Read>(reader: R) -> Result<Self> {
        BlueprintCodec::decode(reader)
    }

    pub fn encode<W: Write>(&self, writer: W) -> Result<()> {
        BlueprintCodec::encode(writer, self)
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

/// Utility class which knows how to convert JSON to and from Factorio's blueprint string format.
pub struct BlueprintCodec;

impl BlueprintCodec {
    /// writer adaptor which encodes json data to blueprint string format
    ///
    /// typically it is more useful to use `encode` instead, but this method
    /// provides some extra flexibility.
    pub fn encode_writer<W, F>(writer: W, inner: F) -> Result<()>
    where
        W: Write,
        F: FnOnce(ZlibEncoder<&mut Base64Encoder<VersionPrefixWriter<W>>>) -> std::io::Result<()>,
    {
        // the final step before sending the data out is to prepend a 0.
        let mut writer = VersionPrefixWriter::new('0', writer);
        // before we prepend that 0, we need to base64-encode the stream
        let mut writer = Base64Encoder::new(&mut writer, base64::STANDARD);
        // note: we can't just hand this off, because we'll need to call its
        // `finish` method later
        {
            // before we base64 it, we should compress it
            let writer = ZlibEncoder::new(writer.by_ref(), Compression::new(9));
            // hand it off to the inner closure
            inner(writer)?;
        }
        writer.finish().map_err(|e| e.into())
    }

    /// write the blueprint string to the given writer
    pub fn encode<W: Write>(writer: W, container: &Container) -> Result<()> {
        Self::encode_writer(writer, |writer| {
            // actually write this struct to the stream
            serde_json::to_writer(writer, container).map_err(|e| e.into())
        })
    }

    /// produce a new owned string containing the blueprint string
    pub fn encode_string(container: &Container) -> Result<String> {
        let mut out = Vec::new();
        Self::encode(&mut out, container)?;
        String::from_utf8(out).map_err(|e| e.into())
    }

    /// reader adaptor which decodes a blueprint string to json
    ///
    /// typically it is more useful to use `decode` instead, but this method
    /// gives flexibility in the event that it is required
    pub fn decode_reader<R, F>(reader: R, inner: F) -> Result<()>
    where
        R: Read,
        F: FnOnce(
            ZlibDecoder<Base64Decoder<VersionPrefixReader<WhitespaceRemover<R>>>>,
        ) -> std::io::Result<()>,
    {
        // first, get rid of all whitespace. We know that the blueprint is
        // base64-encoded, and that character set has no whitespace, so this
        // just makes things a lot more robust.
        let reader = WhitespaceRemover::new(reader);
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
            inner(reader)?;
        }
        if !reader.had_expected_version().ok_or(Error::NoData)? {
            Err(Error::UnknownVersion)?;
        }
        Ok(())
    }

    /// read the blueprint string from the given reader
    pub fn decode<R: Read>(reader: R) -> Result<Container> {
        let mut out = Err(Error::NoData);
        Self::decode_reader(reader, |reader| {
            out = serde_json::from_reader(reader).map_err(|e| e.into());
            Ok(())
        })?;
        out
    }

    /// read the blueprint string from the given input
    pub fn decode_string(blueprint: &str) -> Result<Container> {
        Self::decode(blueprint.as_bytes())
    }
}
