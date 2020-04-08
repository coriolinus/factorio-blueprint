use std::io::{Read, Result};

/// Remove all ascii whitespace from an incoming stream.
pub struct WhitespaceRemover<R> {
    inner: R,
}

impl<R> WhitespaceRemover<R>
where
    R: Read,
{
    pub fn new(inner: R) -> WhitespaceRemover<R> {
        WhitespaceRemover { inner }
    }
}

impl<R> Read for WhitespaceRemover<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut my_buf = vec![0; buf.len()];
        let n = self.inner.read(&mut my_buf)?;
        my_buf.truncate(n);
        my_buf.retain(|d| !d.is_ascii_whitespace());
        buf[..my_buf.len()].copy_from_slice(&my_buf);
        Ok(my_buf.len())
    }
}
