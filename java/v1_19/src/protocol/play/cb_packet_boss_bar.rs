use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBossBar ; impl Packet for CbPacketBossBar { type PacketIDType = i32 ; type PacketContent = void ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 13 } }
