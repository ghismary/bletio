//! Advertising data packets.
//!
//! Normal advertising data and scan response data definitions.

use bletio_utils::{Buffer, BufferOps, Error};

const ADVERTISING_DATA_TOTAL_SIZE: usize = 32;
const ADVERTISING_DATA_SIZE_OFFSET: usize = 0;
const ADVERTISING_DATA_DATA_OFFSET: usize = 1;

/// Advertising Data sent when advertising.
///
/// The packet format for the Advertising Data is defined in
/// [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvertisingData {
    buffer: Buffer<ADVERTISING_DATA_TOTAL_SIZE>,
}

impl AdvertisingData {
    pub fn fill(
        &mut self,
        func: impl FnOnce(
            &mut Buffer<ADVERTISING_DATA_TOTAL_SIZE>,
        ) -> Result<usize, bletio_utils::Error>,
    ) -> Result<usize, bletio_utils::Error> {
        let len = func(&mut self.buffer)?;
        self.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] += len as u8;
        Ok(len)
    }
}

impl Default for AdvertisingData {
    fn default() -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] = 0;
        s.buffer.offset = ADVERTISING_DATA_DATA_OFFSET;
        s
    }
}

impl TryFrom<(u8, [u8; ADVERTISING_DATA_TOTAL_SIZE - 1])> for AdvertisingData {
    type Error = Error;

    fn try_from(value: (u8, [u8; ADVERTISING_DATA_TOTAL_SIZE - 1])) -> Result<Self, Self::Error> {
        let (len, value) = value;
        if len as usize >= ADVERTISING_DATA_TOTAL_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] = len;
        s.buffer.offset = ADVERTISING_DATA_DATA_OFFSET;
        s.buffer.copy_from_slice(&value[..len as usize])?;
        Ok(s)
    }
}

impl bletio_utils::EncodeToBuffer for AdvertisingData {
    fn encode<B: bletio_utils::BufferOps>(
        &self,
        buffer: &mut B,
    ) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(self.buffer.full_data())
    }

    fn encoded_size(&self) -> usize {
        self.buffer.full_data().len()
    }
}

/// Scan Response Data that can be sent when the advertising is scannable.
///
/// The packet format for the Scan Response Data is defined in
/// [Core Specification 6.0, Vol.3, Part C, 11](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-51247611-bdce-274e-095c-afb6d879c55c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResponseData {
    buffer: Buffer<ADVERTISING_DATA_TOTAL_SIZE>,
}

impl ScanResponseData {
    pub fn fill(
        &mut self,
        func: impl FnOnce(
            &mut Buffer<ADVERTISING_DATA_TOTAL_SIZE>,
        ) -> Result<usize, bletio_utils::Error>,
    ) -> Result<usize, bletio_utils::Error> {
        let len = func(&mut self.buffer)?;
        self.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] += len as u8;
        Ok(len)
    }
}

impl Default for ScanResponseData {
    fn default() -> Self {
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] = 0;
        s.buffer.offset = ADVERTISING_DATA_DATA_OFFSET;
        s
    }
}

impl TryFrom<(u8, [u8; ADVERTISING_DATA_TOTAL_SIZE - 1])> for ScanResponseData {
    type Error = Error;

    fn try_from(value: (u8, [u8; ADVERTISING_DATA_TOTAL_SIZE - 1])) -> Result<Self, Self::Error> {
        let (len, value) = value;
        if len as usize >= ADVERTISING_DATA_TOTAL_SIZE {
            return Err(Error::BufferTooSmall);
        }
        let mut s = Self {
            buffer: Default::default(),
        };
        s.buffer.data[ADVERTISING_DATA_SIZE_OFFSET] = len;
        s.buffer.offset = ADVERTISING_DATA_DATA_OFFSET;
        s.buffer.copy_from_slice(&value[..len as usize])?;
        Ok(s)
    }
}

impl bletio_utils::EncodeToBuffer for ScanResponseData {
    fn encode<B: BufferOps>(&self, buffer: &mut B) -> Result<usize, bletio_utils::Error> {
        buffer.copy_from_slice(self.buffer.full_data())
    }

    fn encoded_size(&self) -> usize {
        self.buffer.full_data().len()
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{all_consuming, map_res, verify},
        number::le_u8,
        IResult, Parser,
    };

    use super::*;

    fn advertising_data_length(input: &[u8]) -> IResult<&[u8], u8> {
        verify(le_u8(), |v| (*v as usize) < ADVERTISING_DATA_TOTAL_SIZE).parse(input)
    }

    pub(crate) fn advertising_data(input: &[u8]) -> IResult<&[u8], AdvertisingData> {
        all_consuming(map_res(
            (
                advertising_data_length,
                map_res(take(ADVERTISING_DATA_TOTAL_SIZE - 1), TryInto::try_into),
            ),
            |(len, value)| (len, value).try_into(),
        ))
        .parse(input)
    }

    pub(crate) fn scan_response_data(input: &[u8]) -> IResult<&[u8], ScanResponseData> {
        all_consuming(map_res(
            (
                advertising_data_length,
                map_res(take(ADVERTISING_DATA_TOTAL_SIZE - 1), TryInto::try_into),
            ),
            |(len, value)| (len, value).try_into(),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use bletio_utils::EncodeToBuffer;

    use super::*;

    #[test]
    fn test_advertising_data_try_from_failure() {
        let len = ADVERTISING_DATA_TOTAL_SIZE;
        let data = [0u8; ADVERTISING_DATA_TOTAL_SIZE - 1];
        let err: Result<AdvertisingData, Error> = (len as u8, data).try_into();
        assert_eq!(err, Err(Error::BufferTooSmall));
    }

    #[test]
    fn test_scan_response_data_try_from_failure() {
        let len = ADVERTISING_DATA_TOTAL_SIZE;
        let data = [0u8; ADVERTISING_DATA_TOTAL_SIZE - 1];
        let err: Result<ScanResponseData, Error> = (len as u8, data).try_into();
        assert_eq!(err, Err(Error::BufferTooSmall));
    }

    #[test]
    fn test_advertising_data() -> Result<(), bletio_utils::Error> {
        let data = [25; 16];
        let mut adv_data = AdvertisingData::default();
        assert_eq!(adv_data.encoded_size(), ADVERTISING_DATA_TOTAL_SIZE);
        assert_eq!(&adv_data.buffer.data, &[0; 32]);
        assert_eq!(
            adv_data.fill(|buffer| buffer.copy_from_slice(&data[..]))?,
            16
        );
        assert_eq!(adv_data.encoded_size(), ADVERTISING_DATA_TOTAL_SIZE);
        assert_eq!(
            &adv_data.buffer.data,
            &[
                16, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_response_data() -> Result<(), bletio_utils::Error> {
        let data = [25; 14];
        let mut adv_data = ScanResponseData::default();
        assert_eq!(adv_data.encoded_size(), ADVERTISING_DATA_TOTAL_SIZE);
        assert_eq!(&adv_data.buffer.data, &[0; 32]);
        assert_eq!(
            adv_data.fill(|buffer| buffer.copy_from_slice(&data[..]))?,
            14
        );
        assert_eq!(adv_data.encoded_size(), ADVERTISING_DATA_TOTAL_SIZE);
        assert_eq!(
            &adv_data.buffer.data,
            &[
                14, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        Ok(())
    }
}
