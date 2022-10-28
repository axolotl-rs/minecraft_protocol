use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct CbPacketPing;
impl Packet for CbPacketPing {
    type PacketIDType = i32;
    type PacketContent = PacketPingContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        48
    }
}
pub struct PacketPingContent {
    pub id: i32,
}
impl PacketContent for PacketPingContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.id.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let id: i32 = PacketContent::read(reader)?;

        Ok(Self { id })
    }
}
