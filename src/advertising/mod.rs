pub mod ad_struct;
mod advertising_data;
pub mod advertising_parameters;

pub use ad_struct::flags::Flags;
pub use ad_struct::tx_power_level::TxPowerLevel;
pub use advertising_data::AdvertisingData;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum AdvertisingEnable {
    Disabled = 0x00,
    Enabled = 0x01,
}
