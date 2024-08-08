#![allow(dead_code)]

extern crate bitpack;
extern crate bitval;

#[cfg(test)]
mod tests {
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
        println!("Packed 32: {:#032b}", packed);
        assert_eq!(packed, 0b000000000000000000000000001101); // 21 in decimal
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
        assert_eq!(packed, 0b00000000000000000000000000000000000000000000000000000010101101); // 0xB5B5 in hexadecimal
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
            i: bool,
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
}
