use std::io::prelude::*;

struct VersionPrefix<W> {
    version: u8,
    has_seen_version: bool,
    wrapped: W,
}

impl<W> VersionPrefix<W> {
    fn new(version: char, wrapped: W) -> Self {
        VersionPrefix {
            version: version as u8,
            has_seen_version: false,
            wrapped,
        }
    }
}

pub struct VersionPrefixWriter<W>(VersionPrefix<W>);

impl<W> VersionPrefixWriter<W>
where
    W: Write,
{
    pub fn new(version: char, wrapped: W) -> VersionPrefixWriter<W> {
        VersionPrefixWriter(VersionPrefix::new(version, wrapped))
    }
}

impl<W> Write for VersionPrefixWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut offset = 0;
        if !self.0.has_seen_version {
            self.0.has_seen_version = true;
            self.0.wrapped.write(&[self.0.version])?;
            offset = 1;
        }
        self.0.wrapped.write(buf).map(|n| n + offset)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.wrapped.flush()
    }
}

pub struct VersionPrefixReader<R>(VersionPrefix<R>, Option<u8>);

impl<R> VersionPrefixReader<R>
where
    R: Read,
{
    pub fn new(version: char, wrapped: R) -> VersionPrefixReader<R> {
        VersionPrefixReader(VersionPrefix::new(version, wrapped), None)
    }

    /// return `None` when no bytes have been read, or the truth value
    /// of the statement "the first byte read matched the expected version".
    pub fn had_expected_version(&self) -> Option<bool> {
        self.1.map(|found| found == self.0.version)
    }
}

impl<R> Read for VersionPrefixReader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.0.has_seen_version {
            self.0.has_seen_version = true;
            let mut head_buf = vec![0];
            self.0.wrapped.read_exact(&mut head_buf)?;
            self.1 = Some(head_buf[0]);
        }
        self.0.wrapped.read(buf)
    }
}
