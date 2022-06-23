mod sb_packet_close_window { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketCloseWindow ; impl Packet for SbPacketCloseWindow { type PacketIDType = i32 ; type PacketContent = PacketCloseWindowContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 9 } } pub struct PacketCloseWindowContent { window_id: u8 ,

 } impl PacketContent for PacketCloseWindowContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id } ) } }

 }

 pub use sb_packet_close_window ::*;
