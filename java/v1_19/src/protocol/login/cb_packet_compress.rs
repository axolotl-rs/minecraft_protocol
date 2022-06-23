mod cb_packet_compress { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCompress ; impl Packet for CbPacketCompress { type PacketIDType = i32 ; type PacketContent = PacketCompressContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 3 } } pub struct PacketCompressContent { threshold: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketCompressContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.threshold.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let threshold : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { threshold } ) } }

 }

 pub use cb_packet_compress ::*;