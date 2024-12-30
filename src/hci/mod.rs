mod command;

#[derive(Debug)]
#[repr(u8)]
enum PacketType {
    Command = 0x01,
    AclData = 0x02,
    SynchronousData = 0x03,
    Event = 0x04,
}