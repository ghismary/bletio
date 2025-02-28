//! Assigned numbers for Bluetooth URI schemes.
//!
//! FILE GENERATED FROM REVISION 4f955efbe9144b01c02e2ee2044f43d510cc8da0 OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::advertising::AdvertisingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[num_enum(error_type(name = AdvertisingError, constructor = AdvertisingError::InvalidProvisionedUriSchemeValue))]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth URI schemes defined in
/// [Assigned Numbers, 2.7](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/core/uri_schemes.yaml).
///
/// It is to be used when creating a Uniform Resource Identifier Advertising Structure.
/// See [UriAdStruct::try_new](crate::advertising::ad_struct::UriAdStruct::try_new).
pub enum ProvisionedUriScheme {
    /// "aaa:" URI scheme
    Aaa = 0x0002,
    /// "aaas:" URI scheme
    Aaas = 0x0003,
    /// "about:" URI scheme
    About = 0x0004,
    /// "acap:" URI scheme
    Acap = 0x0005,
    /// "acct:" URI scheme
    Acct = 0x0006,
    /// "cap:" URI scheme
    Cap = 0x0007,
    /// "cid:" URI scheme
    Cid = 0x0008,
    /// "coap:" URI scheme
    Coap = 0x0009,
    /// "coaps:" URI scheme
    Coaps = 0x000A,
    /// "crid:" URI scheme
    Crid = 0x000B,
    /// "data:" URI scheme
    Data = 0x000C,
    /// "dav:" URI scheme
    Dav = 0x000D,
    /// "dict:" URI scheme
    Dict = 0x000E,
    /// "dns:" URI scheme
    Dns = 0x000F,
    /// "file:" URI scheme
    File = 0x0010,
    /// "ftp:" URI scheme
    Ftp = 0x0011,
    /// "geo:" URI scheme
    Geo = 0x0012,
    /// "go:" URI scheme
    Go = 0x0013,
    /// "gopher:" URI scheme
    Gopher = 0x0014,
    /// "h323:" URI scheme
    H323 = 0x0015,
    /// "http:" URI scheme
    Http = 0x0016,
    /// "https:" URI scheme
    Https = 0x0017,
    /// "iax:" URI scheme
    Iax = 0x0018,
    /// "icap:" URI scheme
    Icap = 0x0019,
    /// "im:" URI scheme
    Im = 0x001A,
    /// "imap:" URI scheme
    Imap = 0x001B,
    /// "info:" URI scheme
    Info = 0x001C,
    /// "ipp:" URI scheme
    Ipp = 0x001D,
    /// "ipps:" URI scheme
    Ipps = 0x001E,
    /// "iris:" URI scheme
    Iris = 0x001F,
    /// "iris.beep:" URI scheme
    IrisBeep = 0x0020,
    /// "iris.xpc:" URI scheme
    IrisXpc = 0x0021,
    /// "iris.xpcs:" URI scheme
    IrisXpcs = 0x0022,
    /// "iris.lwz:" URI scheme
    IrisLwz = 0x0023,
    /// "jabber:" URI scheme
    Jabber = 0x0024,
    /// "ldap:" URI scheme
    Ldap = 0x0025,
    /// "mailto:" URI scheme
    Mailto = 0x0026,
    /// "mid:" URI scheme
    Mid = 0x0027,
    /// "msrp:" URI scheme
    Msrp = 0x0028,
    /// "msrps:" URI scheme
    Msrps = 0x0029,
    /// "mtqp:" URI scheme
    Mtqp = 0x002A,
    /// "mupdate:" URI scheme
    Mupdate = 0x002B,
    /// "news:" URI scheme
    News = 0x002C,
    /// "nfs:" URI scheme
    Nfs = 0x002D,
    /// "ni:" URI scheme
    Ni = 0x002E,
    /// "nih:" URI scheme
    Nih = 0x002F,
    /// "nntp:" URI scheme
    Nntp = 0x0030,
    /// "opaquelocktoken:" URI scheme
    Opaquelocktoken = 0x0031,
    /// "pop:" URI scheme
    Pop = 0x0032,
    /// "pres:" URI scheme
    Pres = 0x0033,
    /// "reload:" URI scheme
    Reload = 0x0034,
    /// "rtsp:" URI scheme
    Rtsp = 0x0035,
    /// "rtsps:" URI scheme
    Rtsps = 0x0036,
    /// "rtspu:" URI scheme
    Rtspu = 0x0037,
    /// "service:" URI scheme
    Service = 0x0038,
    /// "session:" URI scheme
    Session = 0x0039,
    /// "shttp:" URI scheme
    Shttp = 0x003A,
    /// "sieve:" URI scheme
    Sieve = 0x003B,
    /// "sip:" URI scheme
    Sip = 0x003C,
    /// "sips:" URI scheme
    Sips = 0x003D,
    /// "sms:" URI scheme
    Sms = 0x003E,
    /// "snmp:" URI scheme
    Snmp = 0x003F,
    /// "soap.beep:" URI scheme
    SoapBeep = 0x0040,
    /// "soap.beeps:" URI scheme
    SoapBeeps = 0x0041,
    /// "stun:" URI scheme
    Stun = 0x0042,
    /// "stuns:" URI scheme
    Stuns = 0x0043,
    /// "tag:" URI scheme
    Tag = 0x0044,
    /// "tel:" URI scheme
    Tel = 0x0045,
    /// "telnet:" URI scheme
    Telnet = 0x0046,
    /// "tftp:" URI scheme
    Tftp = 0x0047,
    /// "thismessage:" URI scheme
    Thismessage = 0x0048,
    /// "tn3270:" URI scheme
    Tn3270 = 0x0049,
    /// "tip:" URI scheme
    Tip = 0x004A,
    /// "turn:" URI scheme
    Turn = 0x004B,
    /// "turns:" URI scheme
    Turns = 0x004C,
    /// "tv:" URI scheme
    Tv = 0x004D,
    /// "urn:" URI scheme
    Urn = 0x004E,
    /// "vemmi:" URI scheme
    Vemmi = 0x004F,
    /// "ws:" URI scheme
    Ws = 0x0050,
    /// "wss:" URI scheme
    Wss = 0x0051,
    /// "xcon:" URI scheme
    Xcon = 0x0052,
    /// "xcon-userid:" URI scheme
    XconUserid = 0x0053,
    /// "xmlrpc.beep:" URI scheme
    XmlrpcBeep = 0x0054,
    /// "xmlrpc.beeps:" URI scheme
    XmlrpcBeeps = 0x0055,
    /// "xmpp:" URI scheme
    Xmpp = 0x0056,
    /// "z39.50r:" URI scheme
    Z3950R = 0x0057,
    /// "z39.50s:" URI scheme
    Z3950S = 0x0058,
    /// "acr:" URI scheme
    Acr = 0x0059,
    /// "adiumxtra:" URI scheme
    Adiumxtra = 0x005A,
    /// "afp:" URI scheme
    Afp = 0x005B,
    /// "afs:" URI scheme
    Afs = 0x005C,
    /// "aim:" URI scheme
    Aim = 0x005D,
    /// "apt:" URI scheme
    Apt = 0x005E,
    /// "attachment:" URI scheme
    Attachment = 0x005F,
    /// "aw:" URI scheme
    Aw = 0x0060,
    /// "barion:" URI scheme
    Barion = 0x0061,
    /// "beshare:" URI scheme
    Beshare = 0x0062,
    /// "bitcoin:" URI scheme
    Bitcoin = 0x0063,
    /// "bolo:" URI scheme
    Bolo = 0x0064,
    /// "callto:" URI scheme
    Callto = 0x0065,
    /// "chrome:" URI scheme
    Chrome = 0x0066,
    /// "chrome-extension:" URI scheme
    ChromeExtension = 0x0067,
    /// "com-eventbrite-attendee:" URI scheme
    ComEventbriteAttendee = 0x0068,
    /// "content:" URI scheme
    Content = 0x0069,
    /// "cvs:" URI scheme
    Cvs = 0x006A,
    /// "dlna-playsingle:" URI scheme
    DlnaPlaysingle = 0x006B,
    /// "dlna-playcontainer:" URI scheme
    DlnaPlaycontainer = 0x006C,
    /// "dtn:" URI scheme
    Dtn = 0x006D,
    /// "dvb:" URI scheme
    Dvb = 0x006E,
    /// "ed2k:" URI scheme
    Ed2K = 0x006F,
    /// "facetime:" URI scheme
    Facetime = 0x0070,
    /// "feed:" URI scheme
    Feed = 0x0071,
    /// "feedready:" URI scheme
    Feedready = 0x0072,
    /// "finger:" URI scheme
    Finger = 0x0073,
    /// "fish:" URI scheme
    Fish = 0x0074,
    /// "gg:" URI scheme
    Gg = 0x0075,
    /// "git:" URI scheme
    Git = 0x0076,
    /// "gizmoproject:" URI scheme
    Gizmoproject = 0x0077,
    /// "gtalk:" URI scheme
    Gtalk = 0x0078,
    /// "ham:" URI scheme
    Ham = 0x0079,
    /// "hcp:" URI scheme
    Hcp = 0x007A,
    /// "icon:" URI scheme
    Icon = 0x007B,
    /// "ipn:" URI scheme
    Ipn = 0x007C,
    /// "irc:" URI scheme
    Irc = 0x007D,
    /// "irc6:" URI scheme
    Irc6 = 0x007E,
    /// "ircs:" URI scheme
    Ircs = 0x007F,
    /// "itms:" URI scheme
    Itms = 0x0080,
    /// "jar:" URI scheme
    Jar = 0x0081,
    /// "jms:" URI scheme
    Jms = 0x0082,
    /// "keyparc:" URI scheme
    Keyparc = 0x0083,
    /// "lastfm:" URI scheme
    Lastfm = 0x0084,
    /// "ldaps:" URI scheme
    Ldaps = 0x0085,
    /// "magnet:" URI scheme
    Magnet = 0x0086,
    /// "maps:" URI scheme
    Maps = 0x0087,
    /// "market:" URI scheme
    Market = 0x0088,
    /// "message:" URI scheme
    Message = 0x0089,
    /// "mms:" URI scheme
    Mms = 0x008A,
    /// "ms-help:" URI scheme
    MsHelp = 0x008B,
    /// "ms-settings-power:" URI scheme
    MsSettingsPower = 0x008C,
    /// "msnim:" URI scheme
    Msnim = 0x008D,
    /// "mumble:" URI scheme
    Mumble = 0x008E,
    /// "mvn:" URI scheme
    Mvn = 0x008F,
    /// "notes:" URI scheme
    Notes = 0x0090,
    /// "oid:" URI scheme
    Oid = 0x0091,
    /// "palm:" URI scheme
    Palm = 0x0092,
    /// "paparazzi:" URI scheme
    Paparazzi = 0x0093,
    /// "pkcs11:" URI scheme
    Pkcs11 = 0x0094,
    /// "platform:" URI scheme
    Platform = 0x0095,
    /// "proxy:" URI scheme
    Proxy = 0x0096,
    /// "psyc:" URI scheme
    Psyc = 0x0097,
    /// "query:" URI scheme
    Query = 0x0098,
    /// "res:" URI scheme
    Res = 0x0099,
    /// "resource:" URI scheme
    Resource = 0x009A,
    /// "rmi:" URI scheme
    Rmi = 0x009B,
    /// "rsync:" URI scheme
    Rsync = 0x009C,
    /// "rtmfp:" URI scheme
    Rtmfp = 0x009D,
    /// "rtmp:" URI scheme
    Rtmp = 0x009E,
    /// "secondlife:" URI scheme
    Secondlife = 0x009F,
    /// "sftp:" URI scheme
    Sftp = 0x00A0,
    /// "sgn:" URI scheme
    Sgn = 0x00A1,
    /// "skype:" URI scheme
    Skype = 0x00A2,
    /// "smb:" URI scheme
    Smb = 0x00A3,
    /// "smtp:" URI scheme
    Smtp = 0x00A4,
    /// "soldat:" URI scheme
    Soldat = 0x00A5,
    /// "spotify:" URI scheme
    Spotify = 0x00A6,
    /// "ssh:" URI scheme
    Ssh = 0x00A7,
    /// "steam:" URI scheme
    Steam = 0x00A8,
    /// "submit:" URI scheme
    Submit = 0x00A9,
    /// "svn:" URI scheme
    Svn = 0x00AA,
    /// "teamspeak:" URI scheme
    Teamspeak = 0x00AB,
    /// "teliaeid:" URI scheme
    Teliaeid = 0x00AC,
    /// "things:" URI scheme
    Things = 0x00AD,
    /// "udp:" URI scheme
    Udp = 0x00AE,
    /// "unreal:" URI scheme
    Unreal = 0x00AF,
    /// "ut2004:" URI scheme
    Ut2004 = 0x00B0,
    /// "ventrilo:" URI scheme
    Ventrilo = 0x00B1,
    /// "view-source:" URI scheme
    ViewSource = 0x00B2,
    /// "webcal:" URI scheme
    Webcal = 0x00B3,
    /// "wtai:" URI scheme
    Wtai = 0x00B4,
    /// "wyciwyg:" URI scheme
    Wyciwyg = 0x00B5,
    /// "xfire:" URI scheme
    Xfire = 0x00B6,
    /// "xri:" URI scheme
    Xri = 0x00B7,
    /// "ymsgr:" URI scheme
    Ymsgr = 0x00B8,
    /// "example:" URI scheme
    Example = 0x00B9,
    /// "ms-settings-cloudstorage:" URI scheme
    MsSettingsCloudstorage = 0x00BA,
}
