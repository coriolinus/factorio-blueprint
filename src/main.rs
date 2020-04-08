#[cfg(not(feature = "codec-cli"))]
fn main() {}

#[cfg(feature = "codec-cli")]
mod m {
    pub use anyhow::Result;
    pub use factorio_blueprint::BlueprintCodec;
    pub use std::io::{copy, stdin, stdout, BufReader, Cursor, Read};
    pub use std::path::PathBuf;
    pub use structopt::StructOpt;

    /// if neither file nor data are set, reads from stdin
    #[derive(Debug, StructOpt)]
    pub struct CodecOpts {
        /// read this file
        #[structopt(short, long, parse(from_os_str), conflicts_with = "data")]
        file: Option<PathBuf>,
        /// read data from the command line instead of a file
        #[structopt(short, long)]
        data: Option<String>,
    }

    impl CodecOpts {
        pub fn reader(self) -> std::io::Result<Box<dyn Read>> {
            Ok(match (self.file, self.data) {
                (None, None) => Box::new(BufReader::new(stdin())),
                (Some(path), None) => Box::new(BufReader::new(std::fs::File::open(path)?)),
                (None, Some(data)) => Box::new(Cursor::new(data.into_bytes())),
                _ => unreachable!(),
            })
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(about = "Convert between JSON and Factorio blueprint strings")]
    pub enum Opt {
        #[structopt(about = "encode JSON data into a blueprint string")]
        Encode {
            #[structopt(flatten)]
            codec_opts: CodecOpts,
        },
        #[structopt(about = "decode a blueprint string to JSON")]
        Decode {
            #[structopt(flatten)]
            codec_opts: CodecOpts,
        },
    }
}

#[cfg(feature = "codec-cli")]
use m::*;

#[cfg(feature = "codec-cli")]
fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Encode { codec_opts } => {
            let mut reader = codec_opts.reader()?;
            let writer = stdout();
            let writer = writer.lock();
            BlueprintCodec::encode_writer(writer, |mut writer| {
                copy(&mut reader, &mut writer).map(|_| ())
            })?
        }
        Opt::Decode { codec_opts } => {
            BlueprintCodec::decode_reader(codec_opts.reader()?, |mut reader| {
                let writer = stdout();
                let mut writer = writer.lock();
                copy(&mut reader, &mut writer).map(|_| ())
            })?
        }
    }
    Ok(())
}
