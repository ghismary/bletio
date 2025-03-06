use bletio_utils::{Buffer, BufferOps};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    advertising_data::ADVERTISING_DATA_SIZE, Error, PublicDeviceAddress, RandomAddress, Rssi,
};

const LE_ADVERTISING_REPORT_EVENT_MAX_SIZE: usize = 251;

/// Number of reports contained in a LE Advertising Report event.
///
/// Its value is between 0x01 and 0x19.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct LeAdvertisingReportNumReports {
    value: u8,
}

impl LeAdvertisingReportNumReports {
    const fn try_new(value: u8) -> Result<Self, Error> {
        if (value < 0x01) || (value > Self::MAX) {
            Err(Error::InvalidLeAdvertisingReportNumReports(value))
        } else {
            Ok(Self { value })
        }
    }

    const fn value(&self) -> u8 {
        self.value
    }

    const MAX: u8 = 0x19;
}

impl TryFrom<u8> for LeAdvertisingReportNumReports {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

/// Event type contained in a LE Advertising Report event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidLeAdvertisingReportEventType))]
#[repr(u8)]
#[non_exhaustive]
pub enum LeAdvertisingReportEventType {
    /// Connectable and scannable undirected advertising (`ADV_IND`).
    ConnectableUndirected = 0x00,
    /// Connectable directed advertising (`ADV_DIRECT_IND`).
    ConnectableDirected = 0x01,
    /// Scannable undirected advertising (`ADV_SCAN_IND`).
    ScannableUndirected = 0x02,
    /// Non connectable undirected advertising (`ADV_NONCONN_IND`).
    NonConnectableUndirected = 0x03,
    /// Scan Response (`SCAN_RSP`).
    ScanResponse = 0x04,
}

/// Address type contained in a LE Advertising Report event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = Error, constructor = Error::InvalidLeAdvertisingReportAddressType))]
#[repr(u8)]
#[non_exhaustive]
enum LeAdvertisingReportAddressType {
    /// Public device address.
    PublicDevice = 0x00,
    /// Random device address.
    RandomDevice = 0x01,
    /// Public identity address (corresponds to a resolved RPA).
    PublicIdentity = 0x02,
    /// Random (static) identity address (corresponds to a resolved RPA).
    RandomIdentity = 0x03,
}

/// Address contained in a LE Advertising Report event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LeAdvertisingReportAddress {
    PublicDevice(PublicDeviceAddress),
    RandomDevice(RandomAddress),
    PublicIdentity(PublicDeviceAddress),
    RandomIdentity(RandomAddress),
}

impl LeAdvertisingReportAddress {
    pub const fn value(&self) -> &[u8; 6] {
        match self {
            Self::PublicDevice(address) => address.value(),
            Self::RandomDevice(address) => address.value(),
            Self::PublicIdentity(address) => address.value(),
            Self::RandomIdentity(address) => address.value(),
        }
    }
}

/// Data contained in a LE Advertising Report event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
pub type LeAdvertisingReportData = Buffer<ADVERTISING_DATA_SIZE>;

/// A single report contained in a LE Advertising Report Event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeAdvertisingReport {
    event_type: LeAdvertisingReportEventType,
    address: LeAdvertisingReportAddress,
    data: LeAdvertisingReportData,
    rssi: Option<Rssi>,
}

impl LeAdvertisingReport {
    fn new(
        event_type: LeAdvertisingReportEventType,
        address: LeAdvertisingReportAddress,
        data: LeAdvertisingReportData,
        rssi: Option<Rssi>,
    ) -> Self {
        Self {
            event_type,
            address,
            data,
            rssi,
        }
    }

    pub fn address(&self) -> &LeAdvertisingReportAddress {
        &self.address
    }

    pub fn data(&self) -> &LeAdvertisingReportData {
        &self.data
    }

    pub fn event_type(&self) -> LeAdvertisingReportEventType {
        self.event_type
    }

    pub fn rssi(&self) -> Option<Rssi> {
        self.rssi
    }
}

/// List of all the reports contained in a LE Advertising Report Event.
///
/// See [Core Specification 6.0, Vol.4, Part E, 7.7.65.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bacd71f4-fabc-238d-72ee-f9aaaf5cbf22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeAdvertisingReportList {
    data: Buffer<LE_ADVERTISING_REPORT_EVENT_MAX_SIZE>,
    num_reports: LeAdvertisingReportNumReports,
}

impl LeAdvertisingReportList {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.num_reports.value() as usize
    }
}

pub struct LeAdvertisingReportListIterator<'a> {
    data: &'a [u8],
    next_index: usize,
}

impl Iterator for LeAdvertisingReportListIterator<'_> {
    type Item = LeAdvertisingReport;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.data.len() {
            return None;
        }
        // INVARIANT: The parsing is known to be ok, it has already been done when creating the LeAdvertisingReportList
        let (_, (len, report)) =
            parser::le_advertising_report(&self.data[self.next_index..]).unwrap();
        self.next_index += len;
        Some(report)
    }
}

impl LeAdvertisingReportList {
    fn new(num_reports: LeAdvertisingReportNumReports, data: &[u8]) -> Self {
        Self {
            // INVARIANT: The data buffer is known to be big enough
            data: data.try_into().unwrap(),
            num_reports,
        }
    }

    pub fn iter(&self) -> LeAdvertisingReportListIterator {
        LeAdvertisingReportListIterator {
            data: self.data.data(),
            next_index: 0,
        }
    }
}

pub(crate) mod parser {
    use nom::{
        bytes::take,
        combinator::{consumed, eof, map, map_res},
        number::le_u8,
        IResult, Parser,
    };

    use crate::{device_address::parser::address, LeMetaEvent};

    use super::*;

    fn le_advertising_report_num_reports(
        input: &[u8],
    ) -> IResult<&[u8], LeAdvertisingReportNumReports> {
        map_res(le_u8(), LeAdvertisingReportNumReports::try_from).parse(input)
    }

    fn le_advertising_report_event_type(
        input: &[u8],
    ) -> IResult<&[u8], LeAdvertisingReportEventType> {
        map_res(le_u8(), LeAdvertisingReportEventType::try_from).parse(input)
    }

    fn le_advertising_report_address_type(
        input: &[u8],
    ) -> IResult<&[u8], LeAdvertisingReportAddressType> {
        map_res(le_u8(), LeAdvertisingReportAddressType::try_from).parse(input)
    }

    fn le_advertising_report_address(input: &[u8]) -> IResult<&[u8], LeAdvertisingReportAddress> {
        let (rest, address_type) = le_advertising_report_address_type(input)?;
        let (rest, address) = address(rest)?;
        Ok((
            rest,
            match address_type {
                LeAdvertisingReportAddressType::PublicDevice => {
                    LeAdvertisingReportAddress::PublicDevice(address.into())
                }
                LeAdvertisingReportAddressType::RandomDevice => {
                    LeAdvertisingReportAddress::RandomDevice(address.try_into().map_err(|_| {
                        nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Fail,
                        ))
                    })?)
                }
                LeAdvertisingReportAddressType::PublicIdentity => {
                    LeAdvertisingReportAddress::PublicIdentity(address.into())
                }
                LeAdvertisingReportAddressType::RandomIdentity => {
                    LeAdvertisingReportAddress::RandomIdentity(address.try_into().map_err(
                        |_| {
                            nom::Err::Failure(nom::error::Error::new(
                                input,
                                nom::error::ErrorKind::Fail,
                            ))
                        },
                    )?)
                }
            },
        ))
    }

    fn le_advertising_report_data_length(input: &[u8]) -> IResult<&[u8], u8> {
        le_u8().parse(input)
    }

    fn le_advertising_report_data(input: &[u8]) -> IResult<&[u8], LeAdvertisingReportData> {
        let (rest, data_length) = le_advertising_report_data_length(input)?;
        map_res(take(data_length), TryInto::try_into).parse(rest)
    }

    fn le_advertising_report_rssi(input: &[u8]) -> IResult<&[u8], Option<Rssi>> {
        map_res(le_u8(), |v| match v {
            0x7F => Ok::<_, Error>(None),
            _ => Ok(Some(Rssi::try_new(v as i8)?)),
        })
        .parse(input)
    }

    pub(crate) fn le_advertising_report(
        input: &[u8],
    ) -> IResult<&[u8], (usize, LeAdvertisingReport)> {
        map(
            consumed((
                le_advertising_report_event_type,
                le_advertising_report_address,
                le_advertising_report_data,
                le_advertising_report_rssi,
            )),
            |(consumed, (event_type, address, data, rssi))| {
                (
                    consumed.len(),
                    LeAdvertisingReport::new(event_type, address, data, rssi),
                )
            },
        )
        .parse(input)
    }

    pub(crate) fn le_advertising_report_event(input: &[u8]) -> IResult<&[u8], LeMetaEvent> {
        let (parameters, num_reports) = le_advertising_report_num_reports(input)?;
        let mut index: u8 = 0;
        let mut rest = parameters;
        while index < num_reports.value() {
            // We don't care about the result, it's just to check that the report is valid
            let (r, _) = le_advertising_report(rest)?;
            rest = r;
            index += 1;
        }
        eof(rest)?;
        Ok((
            &[],
            LeMetaEvent::LeAdvertisingReport(LeAdvertisingReportList::new(num_reports, parameters)),
        ))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{packet::parser::packet, AdvertisingData, Event, LeMetaEvent, Packet};

    use super::*;

    #[rstest]
    #[case(0x01)]
    #[case(0x19)]
    #[case(0x0A)]
    fn test_le_advertising_num_reports_success(#[case] input: u8) {
        let num_reports: LeAdvertisingReportNumReports = input.try_into().unwrap();
        assert_eq!(num_reports.value(), input);
    }

    #[rstest]
    #[case(0x00)]
    #[case(0x1A)]
    #[case(0xFF)]
    fn test_le_advertising_num_reports_failure(#[case] input: u8) {
        let err = LeAdvertisingReportNumReports::try_new(input);
        assert_eq!(err, Err(Error::InvalidLeAdvertisingReportNumReports(input)));
    }

    #[rstest]
    #[case(
        PublicDeviceAddress::default(),
        LeAdvertisingReportAddress::PublicDevice(PublicDeviceAddress::default())
    )]
    #[case(
        PublicDeviceAddress::default(),
        LeAdvertisingReportAddress::PublicIdentity(PublicDeviceAddress::default())
    )]
    fn test_le_advertising_report_public_address(
        #[case] address: PublicDeviceAddress,
        #[case] le_advertising_report_address: LeAdvertisingReportAddress,
    ) {
        let event_type = LeAdvertisingReportEventType::NonConnectableUndirected;
        let data = LeAdvertisingReportData::default();
        let rssi = Some(Rssi::default());
        let report = LeAdvertisingReport::new(
            event_type,
            le_advertising_report_address.clone(),
            data.clone(),
            rssi,
        );
        assert_eq!(report.event_type(), event_type);
        assert_eq!(report.address(), &le_advertising_report_address);
        assert_eq!(report.address().value(), address.value());
        assert_eq!(report.data(), &data);
        assert_eq!(report.rssi(), rssi);
    }

    #[rstest]
    #[case(
        RandomAddress::try_from([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap(),
        LeAdvertisingReportAddress::RandomDevice(RandomAddress::try_from([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap())
    )]
    #[case(
        RandomAddress::try_from([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap(),
        LeAdvertisingReportAddress::RandomIdentity(RandomAddress::try_from([0x28, 0xC8, 0xE9, 0x7D, 0x6A, 0xF7]).unwrap())
    )]
    fn test_le_advertising_report_random_address(
        #[case] address: RandomAddress,
        #[case] le_advertising_report_address: LeAdvertisingReportAddress,
    ) {
        let event_type = LeAdvertisingReportEventType::ConnectableDirected;
        let data = LeAdvertisingReportData::default();
        let rssi = Some(Rssi::default());
        let report = LeAdvertisingReport::new(
            event_type,
            le_advertising_report_address.clone(),
            data.clone(),
            rssi,
        );
        assert_eq!(report.event_type(), event_type);
        assert_eq!(report.address(), &le_advertising_report_address);
        assert_eq!(report.address().value(), address.value());
        assert_eq!(report.data(), &data);
        assert_eq!(report.rssi(), rssi);
    }

    #[rstest]
    #[case(1.try_into().unwrap(), &[
        0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
        240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204
    ])]
    #[case(2.try_into().unwrap(), &[
        0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
        240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204,
        4, 1, 160, 215, 105, 192, 58, 123, 31, 30, 8, 90, 101, 112, 104, 121, 114, 32, 80, 101,
        114, 105, 112, 104, 101, 114, 97, 108, 32, 83, 97, 109, 112, 108, 101, 32, 76, 111, 110, 103, 204
    ])]
    fn test_le_advertising_report_list(
        #[case] num_reports: LeAdvertisingReportNumReports,
        #[case] data: &[u8],
    ) {
        let report_list = LeAdvertisingReportList::new(num_reports, data);
        assert_eq!(report_list.len(), num_reports.value() as usize);
        let mut it = report_list.iter();
        for _ in 0..report_list.len() {
            assert!(it.next().is_some());
        }
        assert_eq!(it.next(), None);
    }

    #[rstest]
    #[case::one_report(
        LeAdvertisingReportList::new(
            1.try_into().unwrap(),
            &[0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
                240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204]
        ),
        &[4, 62, 41, 2, 1, 0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
            240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204]
    )]
    #[case::two_reports(
        LeAdvertisingReportList::new(
            2.try_into().unwrap(),
            &[0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
                240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204,
                4, 1, 160, 215, 105, 192, 58, 123, 31, 30, 8, 90, 101, 112, 104, 121, 114, 32, 80, 101,
                114, 105, 112, 104, 101, 114, 97, 108, 32, 83, 97, 109, 112, 108, 101, 32, 76, 111, 110, 103, 204]
        ),
        &[4, 62, 82, 2, 2, 0, 1, 160, 215, 105, 192, 58, 123, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
            240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204,
            4, 1, 160, 215, 105, 192, 58, 123, 31, 30, 8, 90, 101, 112, 104, 121, 114, 32, 80, 101,
            114, 105, 112, 104, 101, 114, 97, 108, 32, 83, 97, 109, 112, 108, 101, 32, 76, 111, 110, 103, 204]
    )]
    #[case::public_device_address_and_unknown_rssi(
        LeAdvertisingReportList::new(
            1.try_into().unwrap(),
            &[0, 0, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
                240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 127]
        ),
        &[4, 62, 41, 2, 1, 0, 0, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
            240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 127]
    )]
    #[case::public_identity_address(
        LeAdvertisingReportList::new(
            1.try_into().unwrap(),
            &[0, 2, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
                240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204]
        ),
        &[4, 62, 41, 2, 1, 0, 2, 0xCD, 0x2E, 0x0B, 0x04, 0x32, 0x56, 29, 2, 1, 6, 7, 3, 13, 24, 15, 24, 5, 24, 17, 7,
            240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204]
    )]
    fn test_le_advertising_report_event_parsing(
        #[case] le_advertising_report_list: LeAdvertisingReportList,
        #[case] input: &[u8],
    ) {
        let (rest, packet) = packet(input).unwrap();
        assert_eq!(
            packet,
            Packet::Event(Event::LeMeta(LeMetaEvent::LeAdvertisingReport(
                le_advertising_report_list
            )))
        );
        assert!(rest.is_empty());
    }

    #[rstest]
    #[case::invalid_random_device_address(
        &[4, 62, 41, 2, 1, 0, 1, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 29, 2, 1, 6, 7, 3, 13, 24, 15,
            24, 5, 24, 17, 7, 240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204])]
    #[case::invalid_random_identity_address(
        &[4, 62, 41, 2, 1, 0, 3, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 29, 2, 1, 6, 7, 3, 13, 24, 15,
            24, 5, 24, 17, 7, 240, 222, 188, 154, 120, 86, 52, 18, 120, 86, 52, 18, 120, 86, 52, 18, 204])]
    fn test_le_advertising_report_event_parsing_failure(#[case] input: &[u8]) {
        assert!(packet(input).is_err());
    }

    #[test]
    fn test_advertising_data_from_le_advertising_report_data() {
        let data = [25; 16];
        let mut buffer: Buffer<31> = Buffer::default();
        buffer.copy_from_slice(&data[..]).unwrap();
        let data = LeAdvertisingReportData::from(buffer);
        let adv_data: AdvertisingData = (&data).into();
        assert_eq!(
            adv_data.data(),
            &[16, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25]
        );
    }
}
