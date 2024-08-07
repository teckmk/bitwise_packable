extern crate bitval;

#[cfg(test)]
mod tests {
    use bitval::Bitfield;

    #[test]
    fn test_new_bitfield() {
        let bitfield = Bitfield::new(128);
        assert_eq!(bitfield.parts.len(), 2); // 128 bits should be in 2 u64s
        assert_eq!(bitfield.parts[0], 0);
        assert_eq!(bitfield.parts[1], 0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: 64")]
    fn test_set_out_of_bounds() {
        let mut bitfield = Bitfield::new(64);
        bitfield.set(64, true); // Out of bounds
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: 64")]
    fn test_get_out_of_bounds() {
        let bitfield = Bitfield::new(64);
        bitfield.get(64); // Out of bounds
    }

    #[test]
    fn test_set_and_get() {
        let mut bitfield = Bitfield::new(128);

        // Set some bits
        bitfield.set(0, true);
        bitfield.set(63, true);
        bitfield.set(64, true);
        bitfield.set(127, true);

        // Get and assert the bits
        assert_eq!(bitfield.get(0), true);
        assert_eq!(bitfield.get(63), true);
        assert_eq!(bitfield.get(64), true);
        assert_eq!(bitfield.get(127), true);

        // Assert some bits are false
        assert_eq!(bitfield.get(1), false);
        assert_eq!(bitfield.get(62), false);
        assert_eq!(bitfield.get(65), false);
        assert_eq!(bitfield.get(126), false);
    }

    #[test]
    fn test_set_and_get_multiple_parts() {
        let mut bitfield = Bitfield::new(130); // More than one u64 part

        // Set some bits
        bitfield.set(64, true); // This should go into the second u64 part
        bitfield.set(128, true); // This should go into the third part (which doesn't exist in a 130-bit bitfield)

        // Get and assert the bits
        assert_eq!(bitfield.get(64), true);
        assert_eq!(bitfield.get(128), true);

        // Assert some bits are false
        assert_eq!(bitfield.get(0), false);
        assert_eq!(bitfield.get(63), false);
        assert_eq!(bitfield.get(65), false);
    }

    #[test]
    fn test_set_bits() {
        let mut bitfield = Bitfield::new(64);

        bitfield.set(3, true);
        bitfield.set(7, true);
        bitfield.set(63, true);

        assert_eq!(bitfield.parts[0], (1 << 3) | (1 << 7) | (1 << 63));
    }

    #[test]
    fn test_get_bits() {
        let mut bitfield = Bitfield::new(64);

        bitfield.set(10, true);
        bitfield.set(11, true);

        assert_eq!(bitfield.get(10), true);
        assert_eq!(bitfield.get(11), true);
        assert_eq!(bitfield.get(12), false);
    }
}
