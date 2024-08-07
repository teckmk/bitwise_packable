pub struct Bitfield {
    pub parts: Vec<u64>, // Holds the bit values
}

impl Bitfield {
    /// Creates a new `Bitfield` with the specified number of bits.
    pub fn new(size: usize) -> Self {
        let num_parts = (size + 63) / 64; // Calculate the number of `u64` parts needed
        Bitfield {
            parts: vec![0; num_parts],
        }
    }

    /// Sets the value of a specific bit.
    pub fn set(&mut self, index: usize, value: bool) {
        let part = index / 64;
        let bit = index % 64;
        if value {
            self.parts[part] |= 1 << bit;
        } else {
            self.parts[part] &= !(1 << bit);
        }
    }

    /// Gets the value of a specific bit.
    pub fn get(&self, index: usize) -> bool {
        let part = index / 64;
        let bit = index % 64;
        (self.parts[part] & (1 << bit)) != 0
    }
}