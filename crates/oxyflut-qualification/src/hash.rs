//! Streaming SHA-256 primitives for immutable qualification inputs.

use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use sha2::{Digest, Sha256};
use thiserror::Error;

const STREAM_BUFFER_BYTES: usize = 64 * 1024;
const SHA256_HEX_LENGTH: usize = 64;

/// A SHA-256 digest represented by its 32 binary bytes.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    /// Returns the digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Sha256Digest")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Reports why a string cannot represent a SHA-256 digest.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum DigestParseError {
    /// The string does not contain exactly 64 lowercase hexadecimal characters.
    #[error("SHA-256 digest must contain exactly 64 lowercase hexadecimal characters")]
    InvalidLength,
    /// The string contains an uppercase or non-hexadecimal character.
    #[error("SHA-256 digest contains a non-lowercase-hexadecimal character")]
    InvalidCharacter,
}

impl FromStr for Sha256Digest {
    type Err = DigestParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();
        if bytes.len() != SHA256_HEX_LENGTH {
            return Err(DigestParseError::InvalidLength);
        }

        let mut digest = [0_u8; 32];
        for (index, [first, second]) in bytes.as_chunks::<2>().0.iter().copied().enumerate() {
            let Some(high) = hex_nibble(first) else {
                return Err(DigestParseError::InvalidCharacter);
            };
            let Some(low) = hex_nibble(second) else {
                return Err(DigestParseError::InvalidCharacter);
            };
            let Some(slot) = digest.get_mut(index) else {
                return Err(DigestParseError::InvalidLength);
            };
            *slot = (high << 4) | low;
        }

        Ok(Self(digest))
    }
}

/// Hashes a reader with a fixed-size buffer.
///
/// The reader is consumed incrementally and is never copied into one whole-file allocation.
///
/// # Errors
///
/// Returns an I/O error when reading the input fails.
pub fn hash_reader<R: Read>(mut reader: R) -> io::Result<Sha256Digest> {
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; STREAM_BUFFER_BYTES];

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let bytes: [u8; 32] = hasher.finalize().into();
    Ok(Sha256Digest(bytes))
}

/// Hashes a file with the fixed-size streaming buffer.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be opened or read.
pub fn hash_file(path: &Path) -> io::Result<Sha256Digest> {
    hash_reader(File::open(path)?)
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::io::{self, Cursor, Read};
    use std::path::PathBuf;

    use super::{STREAM_BUFFER_BYTES, Sha256Digest, hash_file, hash_reader};

    #[test]
    fn hash_matches_published_nist_vectors() -> Result<(), Box<dyn Error>> {
        let cases = [
            (
                Vec::new(),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                b"abc".to_vec(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_vec(),
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ];

        for (input, expected) in cases {
            let digest = hash_reader(Cursor::new(input))?;
            assert_eq!(digest.to_string(), expected);
            assert_eq!(expected.parse::<Sha256Digest>()?, digest);
        }

        let digest = hash_reader(RepeatByteReader::new(b'a', 1_000_000))?;
        assert_eq!(
            digest.to_string(),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );

        Ok(())
    }

    #[test]
    fn hash_streams_file_fixture_with_fixed_buffer() -> Result<(), Box<dyn Error>> {
        let path = temporary_fixture_path("hash-file");
        let contents = vec![b'x'; STREAM_BUFFER_BYTES + 17];
        fs::write(&path, &contents)?;

        let digest = hash_file(&path)?;
        assert_eq!(
            digest.to_string(),
            "54f8940a815f0942a0bc317f3a543fdb161e2928eed2446108202ef84e601b08"
        );
        fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn hash_reader_never_requests_more_than_its_fixed_buffer() -> Result<(), Box<dyn Error>> {
        let reader = BoundedReader::new(STREAM_BUFFER_BYTES + 1);
        let digest = hash_reader(reader)?;
        assert_eq!(
            digest.to_string(),
            "008ffc88d3c96a9f307524eb361e47c5222a887fc45fa0c1fb8d429c5c23b430"
        );
        Ok(())
    }

    #[test]
    fn hash_reader_retries_interrupted_reads() -> Result<(), Box<dyn Error>> {
        let digest = hash_reader(InterruptedReader::new(b"abc"))?;
        assert_eq!(
            digest.to_string(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        Ok(())
    }

    #[test]
    fn hash_digest_parsing_requires_lowercase_hex() {
        assert!(
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
                .parse::<Sha256Digest>()
                .is_err()
        );
        assert!("abc".parse::<Sha256Digest>().is_err());
    }

    fn temporary_fixture_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-{name}-{}", std::process::id()))
    }

    struct RepeatByteReader {
        byte: u8,
        remaining: usize,
    }

    impl RepeatByteReader {
        const fn new(byte: u8, remaining: usize) -> Self {
            Self { byte, remaining }
        }
    }

    impl Read for RepeatByteReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let count = buffer.len().min(self.remaining);
            buffer[..count].fill(self.byte);
            self.remaining -= count;
            Ok(count)
        }
    }

    struct InterruptedReader {
        bytes: Cursor<&'static [u8]>,
        interrupted: bool,
    }

    impl InterruptedReader {
        fn new(bytes: &'static [u8]) -> Self {
            Self {
                bytes: Cursor::new(bytes),
                interrupted: false,
            }
        }
    }

    impl Read for InterruptedReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if !self.interrupted {
                self.interrupted = true;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            self.bytes.read(buffer)
        }
    }

    struct BoundedReader {
        remaining: usize,
    }

    impl BoundedReader {
        const fn new(remaining: usize) -> Self {
            Self { remaining }
        }
    }

    impl Read for BoundedReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if buffer.len() > STREAM_BUFFER_BYTES {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "reader requested an oversized buffer",
                ));
            }
            let count = buffer.len().min(self.remaining);
            buffer[..count].fill(b'a');
            self.remaining -= count;
            Ok(count)
        }
    }
}
