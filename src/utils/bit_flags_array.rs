use core::ops::{BitAnd, BitOr, BitXor, Not};

use bitflags::Bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitFlagsArray<const CAP: usize>(pub(crate) [u8; CAP]);

impl<const CAP: usize> BitFlagsArray<CAP> {
    pub(crate) const fn new(byte: usize, bit: usize) -> Self {
        let mut s = Self::EMPTY;
        s.0[byte] |= 1 << bit;
        s
    }
}

impl<const CAP: usize> Default for BitFlagsArray<CAP> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<const CAP: usize> BitAnd for BitFlagsArray<CAP> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = BitFlagsArray::EMPTY;
        self.0
            .iter()
            .zip(rhs.0.iter())
            .enumerate()
            .for_each(|(i, (a, b))| result.0[i] = a & b);
        result
    }
}

impl<const CAP: usize> BitOr for BitFlagsArray<CAP> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = BitFlagsArray::EMPTY;
        self.0
            .iter()
            .zip(rhs.0.iter())
            .enumerate()
            .for_each(|(i, (a, b))| result.0[i] = a | b);
        result
    }
}

impl<const CAP: usize> BitXor for BitFlagsArray<CAP> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = BitFlagsArray::EMPTY;
        self.0
            .iter()
            .zip(rhs.0.iter())
            .enumerate()
            .for_each(|(i, (a, b))| result.0[i] = a ^ b);
        result
    }
}

impl<const CAP: usize> Not for BitFlagsArray<CAP> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result = self;
        self.0
            .iter()
            .enumerate()
            .for_each(|(i, v)| result.0[i] = !v);
        result
    }
}

impl<const CAP: usize> Bits for BitFlagsArray<CAP> {
    const EMPTY: Self = Self([0; CAP]);
    const ALL: Self = Self([0xFF; CAP]);
}

macro_rules! bitflags_array {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $CAP:literal {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = ($byte:expr, $bit:expr);
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        $vis struct $BitFlags(BitFlagsArray<$CAP>);

        impl $BitFlags {
            $(
                $(#[$inner $($args)*])*
                $vis const $Flag: $BitFlags = $BitFlags(BitFlagsArray::new($byte, $bit));
            )*
        }

        impl bitflags::Flags for $BitFlags {
            const FLAGS: &'static [bitflags::Flag<Self>] = &[
                $(
                    bitflags::Flag::new(stringify!($Flag), $BitFlags::$Flag),
                )*
            ];

            type Bits = BitFlagsArray<$CAP>;

            fn from_bits_retain(bits: Self::Bits) -> Self {
                $BitFlags(bits)
            }

            fn bits(&self) -> Self::Bits {
                self.0
            }
        }

        impl core::ops::BitAnd for $BitFlags {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitOr for $BitFlags {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitXor for $BitFlags {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl core::ops::Not for $BitFlags {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
    };
}

pub(crate) use bitflags_array;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitflagsarray_default() {
        let value: BitFlagsArray<2> = BitFlagsArray::<2>::default();
        assert_eq!(value, BitFlagsArray::EMPTY);
        assert_eq!(value, BitFlagsArray([0x00, 0x00]));
    }

    #[test]
    fn test_bitflagsarray_and() {
        let value1: BitFlagsArray<2> = BitFlagsArray::new(0, 0);
        let value2: BitFlagsArray<2> = BitFlagsArray::new(0, 1);
        assert_eq!(value1 & value2, BitFlagsArray::EMPTY);
    }

    #[test]
    fn test_bitflagsarray_or() {
        let value1: BitFlagsArray<2> = BitFlagsArray::new(0, 2);
        let value2: BitFlagsArray<2> = BitFlagsArray::new(1, 7);
        assert_eq!(value1 | value2, BitFlagsArray([0x04, 0x80]));
    }

    #[test]
    fn test_bitflagsarray_xor() {
        let value: BitFlagsArray<2> = BitFlagsArray::new(1, 3);
        assert_eq!(value ^ value, BitFlagsArray::EMPTY);
    }

    #[test]
    fn test_bitflagsarray_not() {
        let value: BitFlagsArray<2> = BitFlagsArray::new(0, 5);
        assert_eq!(!value, BitFlagsArray([0xDF, 0xFF]));
    }

    #[test]
    fn test_bitflags_array_macro() {
        use bitflags::Flags;

        bitflags_array! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            struct TestBitFlagsArray: 2 {
                const A = (0, 3);
                const B = (1, 5);
            }
        }

        let arr = TestBitFlagsArray::default();
        assert_eq!(arr.bits(), BitFlagsArray([0, 0]));
        assert_eq!(TestBitFlagsArray::A.bits(), BitFlagsArray([0x08, 0x00]));
        assert_eq!(TestBitFlagsArray::B.bits(), BitFlagsArray([0x00, 0x20]));

        assert_eq!(
            TestBitFlagsArray::A & TestBitFlagsArray::A,
            TestBitFlagsArray::A
        );
        assert_eq!(
            TestBitFlagsArray::A & TestBitFlagsArray::B,
            TestBitFlagsArray::empty()
        );

        assert_eq!(
            (TestBitFlagsArray::A | TestBitFlagsArray::A),
            TestBitFlagsArray::A
        );
        assert_eq!(
            (TestBitFlagsArray::A | TestBitFlagsArray::B).bits(),
            BitFlagsArray([0x08, 0x20])
        );

        assert_eq!(
            (TestBitFlagsArray::A ^ TestBitFlagsArray::A).bits(),
            BitFlagsArray([0x00, 0x00])
        );
        assert_eq!(
            (TestBitFlagsArray::A ^ TestBitFlagsArray::B).bits(),
            BitFlagsArray([0x08, 0x20])
        );

        let not_a = !TestBitFlagsArray::A;
        assert_ne!(not_a, TestBitFlagsArray::A);
        assert_eq!(not_a.bits(), BitFlagsArray([0xF7, 0xFF]));

        let arr = TestBitFlagsArray::from_bits_retain(BitFlagsArray([0x38, 0x21]));
        assert!(arr.contains(TestBitFlagsArray::A));
        assert!(arr.contains(TestBitFlagsArray::B));
        assert_eq!(arr.bits(), BitFlagsArray([0x38, 0x21]));
    }
}
