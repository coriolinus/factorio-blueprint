#[cfg(test)]
mod test;

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

        Ok(loop {
            let n = self.inner.read(&mut my_buf)?;
            if n == 0 {
                // the underlying reader is done
                break 0;
            }

            my_buf.truncate(n);
            my_buf.retain(|d| !d.is_ascii_whitespace());

            // keep reading until we get at least a byte to return
            if !my_buf.is_empty() {
                buf[..my_buf.len()].copy_from_slice(&my_buf);
                break my_buf.len();
            }

            my_buf.resize(buf.len(), 0);
        })
    }
}
