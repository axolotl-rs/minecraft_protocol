mod cb_packet_disconnect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketDisconnect ; impl Packet for CbPacketDisconnect { type PacketIDType = i32 ; type PacketContent = PacketDisconnectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketDisconnectContent { reason: String ,

 } impl PacketContent for PacketDisconnectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reason.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let reason : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reason } ) } }

 }

 pub use cb_packet_disconnect ::*;