mod ad_struct;
mod advertising_data;
mod advertising_parameters;

pub use ad_struct::AdStruct;
pub use ad_struct::{
    FlagsAdStruct, ServiceUuid128AdStruct, ServiceUuid16AdStruct, ServiceUuid32AdStruct,
};
pub use advertising_data::AdvertisingData;
pub use advertising_parameters::{
    AdvertisingChannelMap, AdvertisingFilterPolicy, AdvertisingIntervalValue,
    AdvertisingParameters, AdvertisingType, OwnAddressType, PeerAddress, PeerAddressType,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum AdvertisingEnable {
    Disabled = 0x00,
    Enabled = 0x01,
}
