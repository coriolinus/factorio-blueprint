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

        loop {
            let n = self.inner.read(&mut my_buf)?;
            if n == 0 {
                // the underlying reader is done
                return Ok(0);
            }

            my_buf.truncate(n);
            my_buf.retain(|d| !d.is_ascii_whitespace());

            // ensure we got at least a byte to return
            if !my_buf.is_empty() {
                buf[..my_buf.len()].copy_from_slice(&my_buf);
                return Ok(my_buf.len());
            }

            // otherwise reset and reread
            my_buf.resize(buf.len(), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn functionality() {
        const TEST: &str = " t/prau0INWoWUQ0LgQ\tMUdJRcCetBZP\nAyD+DCpn 01yWZT/LBo3Ogk0INwwuAtKNI ";

        let mut stripped = String::new();
        super::WhitespaceRemover::new(TEST.as_bytes())
            .read_to_string(&mut stripped)
            .unwrap();

        assert_eq!(
            stripped,
            "t/prau0INWoWUQ0LgQMUdJRcCetBZPAyD+DCpn01yWZT/LBo3Ogk0INwwuAtKNI"
        );
    }

    #[test]
    fn long_whitespace() {
        let mut test_string = "a".to_owned();
        test_string.reserve_exact(161);
        for _ in 0..20 {
            test_string.push_str("        ");
        }
        test_string.push('b');

        let mut stripped = String::new();
        super::WhitespaceRemover::new(test_string.as_bytes())
            .read_to_string(&mut stripped)
            .unwrap();

        assert_eq!(stripped, "ab");
    }
}
