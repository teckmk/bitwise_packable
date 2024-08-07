extern crate bitpack;
extern crate bitval;

#[cfg(test)]
mod tests {
    use super::*;
    use bitpack::BitwisePackable;
    use bitval::Bitfield;

    #[test]
    fn test_pack_unpack_u8() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i8")]
        struct Example {
            a: bool,
            b: bool,
            c: bool,
        }

        let example = Example {
            a: true,
            b: false,
            c: true,
        };
        let packed = Example::pack(&example);
        assert_eq!(packed, 0b00000101); // 5 in decimal
        let unpacked = Example::unpack(packed);
        assert_eq!(unpacked.a, true);
        assert_eq!(unpacked.b, false);
        assert_eq!(unpacked.c, true);
    }

    #[test]
    fn test_pack_unpack_u16() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i16")]
        struct Example {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
        }

        let example = Example {
            a: true,
            b: false,
            c: true,
            d: true,
        };
        let packed = Example::pack(&example);
        assert_eq!(packed, 0b0000000000001101); // 13 in decimal
        let unpacked = Example::unpack(packed);
        assert_eq!(unpacked.a, true);
        assert_eq!(unpacked.b, false);
        assert_eq!(unpacked.c, true);
        assert_eq!(unpacked.d, true);
    }

    #[test]
    fn test_pack_unpack_u32() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i32")]
        struct Example {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
        }

        let example = Example {
            a: true,
            b: false,
            c: true,
            d: true,
            e: false,
        };
        let packed = Example::pack(&example);
        assert_eq!(packed, 0b00000000000000000000000000010101); // 21 in decimal
        let unpacked = Example::unpack(packed);
        assert_eq!(unpacked.a, true);
        assert_eq!(unpacked.b, false);
        assert_eq!(unpacked.c, true);
        assert_eq!(unpacked.d, true);
        assert_eq!(unpacked.e, false);
    }

    #[test]
    fn test_pack_unpack_u64() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i64")]
        struct Example {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
            f: bool,
            g: bool,
            h: bool,
        }

        let example = Example {
            a: true,
            b: false,
            c: true,
            d: true,
            e: false,
            f: true,
            g: false,
            h: true,
        };
        let packed = Example::pack(&example);
        assert_eq!(packed, 0b1011010101010101); // 0xB5B5 in hexadecimal
        let unpacked = Example::unpack(packed);
        assert_eq!(unpacked.a, true);
        assert_eq!(unpacked.b, false);
        assert_eq!(unpacked.c, true);
        assert_eq!(unpacked.d, true);
        assert_eq!(unpacked.e, false);
        assert_eq!(unpacked.f, true);
        assert_eq!(unpacked.g, false);
        assert_eq!(unpacked.h, true);
    }

    #[test]
    #[should_panic(
        expected = "Overflow occurred during packing: struct 'OverflowExample' has more boolean fields than can be packed in an u8 (8 bits)."
    )]
    fn test_overflow_packing_u8() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i8", overflow = false)]
        struct OverflowExample {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
            f: bool,
            g: bool,
            h: bool,
            i: bool,
        }

        let example = OverflowExample {
            a: true,
            b: false,
            c: true,
            d: true,
            e: false,
            f: true,
            g: false,
            h: true,
            i: false,
        };
        OverflowExample::pack(&example); // This should panic due to overflow
    }

    #[test]
    #[should_panic(
        expected = "Overflow occurred during unpacking: struct 'OverflowExample' has more boolean fields than can be unpacked from an u8 (8 bits)."
    )]
    fn test_overflow_unpacking_u8() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "i8", overflow = false)]
        struct OverflowExample {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
            f: bool,
            g: bool,
            h: bool,
        }

        let packed = 0b11111111; // 255 in decimal
        OverflowExample::unpack(packed); // This should panic due to overflow
    }

    #[test]
    fn test_pack_unpack_auto() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "auto")]
        struct Example {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
        }

        let example = Example {
            a: true,
            b: false,
            c: true,
            d: true,
            e: false,
        };
        let packed = Example::pack(&example);
        let unpacked = Example::unpack(packed);
        assert_eq!(unpacked.a, true);
        assert_eq!(unpacked.b, false);
        assert_eq!(unpacked.c, true);
        assert_eq!(unpacked.d, true);
        assert_eq!(unpacked.e, false);
    }

    #[test]
    #[should_panic(
        expected = "Overflow occurred during packing: struct 'OverflowDynamic' has more boolean fields than can be packed in the provided Bitfield size."
    )]
    fn test_overflow_packing_auto() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "auto", overflow = false)]
        struct OverflowDynamic {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
            f: bool,
            g: bool,
            h: bool,
            i: bool,
            j: bool,
            k: bool,
            l: bool,
            m: bool,
            n: bool,
            o: bool,
            p: bool,
        }

        let example = OverflowDynamic {
            a: true,
            b: false,
            c: true,
            d: true,
            e: false,
            f: true,
            g: false,
            h: true,
            i: false,
            j: true,
            k: false,
            l: true,
            m: true,
            n: false,
            o: true,
            p: false,
        };
        OverflowDynamic::pack(&example); // This should panic due to overflow
    }

    #[test]
    #[should_panic(
        expected = "Overflow occurred during unpacking: struct 'OverflowDynamic' has more boolean fields than can be unpacked from the provided Bitfield size."
    )]
    fn test_overflow_unpacking_auto() {
        #[derive(BitwisePackable)]
        #[bitpack(size = "auto", overflow = false)]
        struct OverflowDynamic {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            e: bool,
            f: bool,
            g: bool,
            h: bool,
            i: bool,
            j: bool,
            k: bool,
            l: bool,
            m: bool,
            n: bool,
            o: bool,
            p: bool,
        }

        let packed = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111]; // More than 64 bits
        OverflowDynamic::unpack(packed); // This should panic due to overflow
    }
}
