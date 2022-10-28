use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketQueryEntityNbt ; impl Packet for SbPacketQueryEntityNbt { type PacketIDType = i32 ; type PacketContent = PacketQueryEntityNbtContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 12 } } pub struct PacketQueryEntityNbtContent { pub transaction_id: minecraft_protocol::data::VarInt ,

pub entity_id: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketQueryEntityNbtContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let transaction_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, entity_id } ) } }
