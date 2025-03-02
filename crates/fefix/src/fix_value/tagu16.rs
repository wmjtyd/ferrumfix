use super::{FixValue, ERR_INT_INVALID, ERR_UTF8};
use crate::{Buffer, BufferWriter, TagU16};
use std::fmt::Write;

impl<'a> FixValue<'a> for TagU16 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let initial_len = buffer.len();
        write!(BufferWriter(buffer), "{}", self).unwrap();
        buffer.len() - initial_len
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(data)
            .map_err(|_| ERR_UTF8)?
            .parse()
            .map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let n = u32::deserialize_lossy(data)?;
        Ok(TagU16::new(n.max(1) as u16).unwrap())
    }
}
