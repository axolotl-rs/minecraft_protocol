mod sb_packet_held_item_slot { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketHeldItemSlot ; impl Packet for SbPacketHeldItemSlot { type PacketIDType = i32 ; type PacketContent = PacketHeldItemSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 37 } } pub struct PacketHeldItemSlotContent { slot_id: i16 ,

 } impl PacketContent for PacketHeldItemSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let slot_id : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot_id } ) } }

 }

 pub use sb_packet_held_item_slot ::*;
