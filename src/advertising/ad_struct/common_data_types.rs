/// List of Common Data Types from Assigned Numbers, 2.3

#[derive(Debug)]
#[repr(u8)]
pub(crate) enum CommonDataType {
    Flags = 0x01,
    IncompleteListOfServiceUuid16 = 0x02,
    CompleteListOfServiceUuid16 = 0x03,
    IncompleteListOfServiceUuid32 = 0x4,
    CompleteListOfServiceUuid32 = 0x05,
    IncompleteListOfServiceUuid128 = 0x06,
    CompleteListOfServiceUuid128 = 0x07,
    // ShortenedLocalName = 0x08,
    // CompleteLocalName = 0x09,
    TxPowerLevel = 0x0A,
}
