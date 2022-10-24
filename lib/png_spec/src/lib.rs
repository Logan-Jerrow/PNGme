pub mod chunk;
pub mod chunk_type;
pub mod png;

mod util {
    pub enum Bit {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
    }

    // pub const LEAST_SIG: Bit = Bit::Zero;

    pub const MOST_SIG: Bit = Bit::Seven;

    impl From<Bit> for u8 {
        fn from(b: Bit) -> Self {
            match b {
                Bit::Zero => 0,
                Bit::One => 1,
                Bit::Two => 2,
                Bit::Three => 3,
                Bit::Four => 4,
                Bit::Five => 5,
                Bit::Six => 6,
                Bit::Seven => 7,
            }
        }
    }

    /// Gets the bit at the index 'bit'; note that index 0 is least sig bit, 7 is most sig bit.
    pub fn get_bit(byte: u8, bit: Bit) -> bool {
        ((byte >> u8::from(bit)) & 1) != 0
    }
}
