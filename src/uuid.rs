// Value from Core specification 4.2, Vol. 3, Part B, 2.5.1
const BLUETOOTH_BASE_UUID: u128 = 0x00000000_0000_1000_8000_00805F9B34FB;

#[derive(Debug, Eq)]
pub enum Uuid {
    Uuid16(Uuid16),
    Uuid32(Uuid32),
    Uuid128(Uuid128),
}

impl Uuid {
    pub fn full(&self) -> Uuid128 {
        match self {
            Uuid::Uuid16(value) => Uuid128::from(*value),
            Uuid::Uuid32(value) => Uuid128::from(*value),
            Uuid::Uuid128(value) => *value,
        }
    }
}

impl PartialEq for Uuid {
    fn eq(&self, other: &Self) -> bool {
        self.full() == other.full()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Uuid16(pub u16);

impl From<u16> for Uuid16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<crate::assigned_numbers::services::Service> for Uuid16 {
    fn from(value: crate::assigned_numbers::services::Service) -> Self {
        (value as u16).into()
    }
}

impl PartialEq<Uuid32> for Uuid16 {
    fn eq(&self, other: &Uuid32) -> bool {
        Uuid32::from(*self) == *other
    }
}

impl PartialEq<Uuid128> for Uuid16 {
    fn eq(&self, other: &Uuid128) -> bool {
        Uuid128::from(*self) == *other
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Uuid32(pub u32);

impl From<u32> for Uuid32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Uuid16> for Uuid32 {
    fn from(value: Uuid16) -> Self {
        (value.0 as u32).into()
    }
}

impl PartialEq<Uuid16> for Uuid32 {
    fn eq(&self, other: &Uuid16) -> bool {
        *self == Uuid32::from(*other)
    }
}

impl PartialEq<Uuid128> for Uuid32 {
    fn eq(&self, other: &Uuid128) -> bool {
        Uuid128::from(*self) == *other
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Uuid128(pub u128);

impl From<u128> for Uuid128 {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<Uuid32> for Uuid128 {
    fn from(value: Uuid32) -> Self {
        (((value.0 as u128) << 96) + BLUETOOTH_BASE_UUID).into()
    }
}

impl From<Uuid16> for Uuid128 {
    fn from(value: Uuid16) -> Self {
        (((value.0 as u128) << 96) + BLUETOOTH_BASE_UUID).into()
    }
}

impl PartialEq<Uuid32> for Uuid128 {
    fn eq(&self, other: &Uuid32) -> bool {
        *self == Uuid128::from(*other)
    }
}

impl PartialEq<Uuid16> for Uuid128 {
    fn eq(&self, other: &Uuid16) -> bool {
        *self == Uuid128::from(*other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uuid16_to_uuid32_conversion() {
        let uuid16 = Uuid16(0x1803);
        let uuid32: Uuid32 = uuid16.into();
        assert_eq!(uuid32.0, 0x0000_01803);
        assert_eq!(uuid16, uuid32);
        assert_eq!(uuid32, uuid16);
    }

    #[test]
    fn test_uuid16_to_uuid128_conversion() {
        let uuid16 = Uuid16(0x180F);
        let uuid128: Uuid128 = uuid16.into();
        assert_eq!(uuid128.0, 0x0000180F_0000_1000_8000_00805F9B34FB);
        assert_eq!(uuid16, uuid128);
        assert_eq!(uuid128, uuid16);
    }

    #[test]
    fn test_uuid32_to_uuid128_conversion() {
        let uuid32 = Uuid32(0x0000_181A);
        let uuid128: Uuid128 = uuid32.into();
        assert_eq!(uuid128.0, 0x0000181A_0000_1000_8000_00805F9B34FB);
        assert_eq!(uuid32, uuid128);
        assert_eq!(uuid128, uuid32);
    }

    #[test]
    fn test_uuids_eq() {
        assert_eq!(Uuid::Uuid16(Uuid16(0x1803)), Uuid::Uuid16(Uuid16(0x1803)));
        assert_eq!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid32(Uuid32(0x0000_1803))
        );
        assert_eq!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB))
        );
        assert_eq!(
            Uuid::Uuid16(Uuid16(0x1803)),
            Uuid::Uuid32(Uuid32(0x0000_1803))
        );
        assert_eq!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid16(Uuid16(0x1803))
        );
        assert_eq!(
            Uuid::Uuid16(Uuid16(0x1803)),
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB))
        );
        assert_eq!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid16(Uuid16(0x1803))
        );
        assert_eq!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB))
        );
        assert_eq!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid32(Uuid32(0x0000_1803))
        );
    }

    #[test]
    fn test_uuids_ne() {
        assert_ne!(Uuid::Uuid16(Uuid16(0x1803)), Uuid::Uuid16(Uuid16(0x180F)));
        assert_ne!(Uuid::Uuid16(Uuid16(0x180F)), Uuid::Uuid16(Uuid16(0x1803)));
        assert_ne!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid32(Uuid32(0x0000_180F))
        );
        assert_ne!(
            Uuid::Uuid32(Uuid32(0x0000_180F)),
            Uuid::Uuid32(Uuid32(0x0000_1803))
        );
        assert_ne!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid128(Uuid128(0x0000180F_0000_1000_8000_00805F9B34FB))
        );
        assert_ne!(
            Uuid::Uuid128(Uuid128(0x0000180F_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB))
        );
        assert_ne!(
            Uuid::Uuid16(Uuid16(0x1803)),
            Uuid::Uuid32(Uuid32(0x0000_180F))
        );
        assert_ne!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid16(Uuid16(0x180F))
        );
        assert_ne!(
            Uuid::Uuid16(Uuid16(0x1803)),
            Uuid::Uuid128(Uuid128(0x0000180F_0000_1000_8000_00805F9B34FB))
        );
        assert_ne!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid16(Uuid16(0x180F))
        );
        assert_ne!(
            Uuid::Uuid32(Uuid32(0x0000_1803)),
            Uuid::Uuid128(Uuid128(0x0000180F_0000_1000_8000_00805F9B34FB))
        );
        assert_ne!(
            Uuid::Uuid128(Uuid128(0x00001803_0000_1000_8000_00805F9B34FB)),
            Uuid::Uuid32(Uuid32(0x0000_180F))
        );
    }
}
