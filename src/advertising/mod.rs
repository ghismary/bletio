mod ad_struct;
mod advertising_data;
mod advertising_parameters;
mod flags;
mod tx_power_level;

pub use ad_struct::{
    FlagsAdStruct, ManufacturerSpecificDataAdStruct, ServiceListCompletion, ServiceUuid128AdStruct,
    ServiceUuid16AdStruct, ServiceUuid32AdStruct, TxPowerLevelAdStruct,
};
pub use advertising_data::AdvertisingData;
pub use advertising_parameters::{
    AdvertisingChannelMap, AdvertisingFilterPolicy, AdvertisingIntervalValue,
    AdvertisingParameters, AdvertisingType, OwnAddressType, PeerAddress, PeerAddressType,
};
pub use flags::Flags;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum AdvertisingEnable {
    Disabled = 0x00,
    Enabled = 0x01,
}
