pub use error::ChunkTypeError;

mod error;
mod trait_impls;

mod tests;

/// 4-byte chunk type code. For convenience in description and in
/// examining PNG files, type codes are restricted to consist of
/// uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90
/// and 97-122 decimal)
///
/// ['PNG Structure'](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType {
    /// Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
    ///
    /// Chunks that are not strictly necessary in order to meaningfully display the contents of the
    /// file are known as "ancillary" chunks. A decoder encountering an unknown chunk in which the
    /// ancillary bit is 1 can safely ignore the chunk and proceed to display the image. The time
    /// chunk (tIME) is an example of an ancillary chunk.
    ///
    /// Chunks that are necessary for successful display of the file's contents are called
    /// "critical" chunks. A decoder encountering an unknown chunk in which the ancillary bit is 0
    /// must indicate to the user that the image contains information it cannot safely interpret.
    /// The image header chunk (IHDR) is an example of a critical chunk.
    ancillary: u8,

    /// Private bit: bit 5 of second byte
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    ///
    /// A public chunk is one that is part of the PNG specification or is registered in the list of
    /// PNG special-purpose public chunk types. Applications can also define private (unregistered)
    /// chunks for their own purposes. The names of private chunks must have a lowercase second
    /// letter, while public chunks will always be assigned names with uppercase second letters.
    /// Note that decoders do not need to test the private-chunk property bit, since it has no
    /// functional significance; it is simply an administrative convenience to ensure that public
    /// and private chunk names will not conflict.
    ///
    /// # See
    /// ['Additional chunk types'](http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html#C.Additional-chunk-types)
    /// ['Use of private chunks'](http://www.libpng.org/pub/png/spec/1.2/PNG-Encoders.html#E.Use-of-private-chunks)
    private: u8,

    /// Reserved bit: bit 5 of third byte
    /// Must be 0 (uppercase) in files conforming to this version of PNG.
    ///
    /// The significance of the case of the third letter of the chunk name is reserved for possible
    /// future expansion. At the present time all chunk names must have uppercase third letters.
    /// (Decoders should not complain about a lowercase third letter, however, as some future
    /// version of the PNG specification could define a meaning for this bit. It is sufficient to
    /// treat a chunk with a lowercase third letter in the same way as any other unknown chunk
    /// type.)
    reserved: u8,

    /// Safe-to-copy bit: bit 5 of fourth byte
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    ///
    /// This property bit is not of interest to pure decoders, but it is needed by PNG editors
    /// (programs that modify PNG files). This bit defines the proper handling of unrecognized
    /// chunks in a file that is being modified.
    ///
    /// If a chunk's safe-to-copy bit is 1, the chunk may be copied to a modified PNG file whether
    /// or not the software recognizes the chunk type, and regardless of the extent of the file
    /// modifications.
    ///
    /// If a chunk's safe-to-copy bit is 0, it indicates that the chunk depends on the image data.
    /// If the program has made any changes to critical chunks, including addition, modification,
    /// deletion, or reordering of critical chunks, then unrecognized unsafe chunks must not be
    /// copied to the output PNG file. (Of course, if the program does recognize the chunk, it can
    /// choose to output an appropriately modified version.)
    ///
    /// A PNG editor is always allowed to copy all unrecognized chunks if it has only added,
    /// deleted, modified, or reordered ancillary chunks. This implies that it is not permissible
    /// for ancillary chunks to depend on other ancillary chunks.
    ///
    /// PNG editors that do not recognize a critical chunk must report an error and refuse to
    /// process that PNG file at all. The safe/unsafe mechanism is intended for use with ancillary
    /// chunks. The safe-to-copy bit will always be 0 for critical chunks.
    safe_to_copy: u8,
}

impl ChunkType {
    /// Property bits position
    const POSITION: u8 = 5;

    /// Checks if the 5th bit of a property bit is one.
    fn is_property_bit_set(byte: u8) -> bool {
        ((byte >> Self::POSITION) & 1) != 0
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        // restricted to uper and lower case ASCII letters.
        byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
    }

    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }

    /// Returns ture if reserve bit is valid and all four bytes are represented by the characters
    /// A-Z or a-z. Note that this chunk type should always be vaild as it is validated during
    /// construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
            && Self::is_valid_byte(self.ancillary)
            && Self::is_valid_byte(self.private)
            && Self::is_valid_byte(self.reserved)
            && Self::is_valid_byte(self.safe_to_copy)
    }

    /// Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary
    pub fn is_critical(&self) -> bool {
        // self.0[0].is_ascii_uppercase()
        !Self::is_property_bit_set(self.ancillary)
    }

    /// Private bit: bit 5 of second byte
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    pub fn is_public(&self) -> bool {
        // self.0[1].is_ascii_uppercase()
        !Self::is_property_bit_set(self.private)
    }

    /// Reserved bit: bit 5 of third byte
    /// Must be 0 (uppercase) in files conforming to this version of PNG.
    pub fn is_reserved_bit_valid(&self) -> bool {
        // self.reserved.is_ascii_uppercase()
        !Self::is_property_bit_set(self.reserved)
    }

    /// Safe-to-copy bit: bit 5 of fourth byte
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    pub fn is_safe_to_copy(&self) -> bool {
        // !self.safe_to_copy.is_ascii_uppercase()
        Self::is_property_bit_set(self.safe_to_copy)
    }
}
