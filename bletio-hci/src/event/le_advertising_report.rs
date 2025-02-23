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
            0x7F => Ok::<_, crate::Error>(None),
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
