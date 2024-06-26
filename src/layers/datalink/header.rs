use super::MacAddr;

pub type TypeLen = u16;

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum EtherType {
    IPv4 = 0x0800,
    Arp = 0x0806,
    IPv6 = 0x86DD,
}

impl From<u16> for EtherType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EtherType::IPv4,
            0x0806 => EtherType::Arp,
            0x86DD => EtherType::IPv6,
            _ => panic!("Unknown EtherType: {:x}", value),
        }
    }
}

pub(super) struct EthernetHeader {
    destination: MacAddr,
    source: MacAddr,
    type_len: TypeLen,
}

impl EthernetHeader {
    pub fn new(source: &MacAddr, destination: &MacAddr, type_len: TypeLen) -> Self {
        EthernetHeader {
            destination: destination.clone(),
            source: source.clone(),
            type_len,
        }
    }

    pub fn src(&self) -> &MacAddr {
        &self.source
    }

    pub fn dest(&self) -> &MacAddr {
        &self.destination
    }

    /// Returns a byte array representation of the EthernetHeader in network byte order
    pub fn to_be_bytes(self) -> [u8; 14] {
        let mut bytes = [0; 14];
        bytes[0..6].copy_from_slice(&self.destination.0);
        bytes[6..12].copy_from_slice(&self.source.0);
        bytes[12..14].copy_from_slice(&self.type_len.to_be_bytes());
        bytes
    }

    pub fn from_be_bytes(bytes: &[u8; 14]) -> Option<Self> {
        let mut destination = [0; 6];
        let mut source = [0; 6];
        destination.copy_from_slice(&bytes[0..6]);
        source.copy_from_slice(&bytes[6..12]);
        let type_len = u16::from_be_bytes([bytes[12], bytes[13]]);
        Some(EthernetHeader {
            destination: MacAddr(destination),
            source: MacAddr(source),
            type_len,
        })
    }
}
