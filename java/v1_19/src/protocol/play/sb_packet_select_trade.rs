use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSelectTrade ; impl Packet for SbPacketSelectTrade { type PacketIDType = i32 ; type PacketContent = PacketSelectTradeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 35 } } pub struct PacketSelectTradeContent { slot: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketSelectTradeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let slot : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot } ) } }
