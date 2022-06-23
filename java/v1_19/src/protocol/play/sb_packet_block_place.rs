mod sb_packet_block_place { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketBlockPlace ; impl Packet for SbPacketBlockPlace { type PacketIDType = i32 ; type PacketContent = PacketBlockPlaceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 46 } } pub struct PacketBlockPlaceContent { hand: minecraft_data::common::data::VarInt ,

location: minecraft_data::common::data::position::Position ,

direction: minecraft_data::common::data::VarInt ,

cursor_x: f32 ,

cursor_y: f32 ,

cursor_z: f32 ,

inside_block: bool ,

 } impl PacketContent for PacketBlockPlaceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.direction.write(writer)?;;

total_bytes += self.cursor_x.write(writer)?;;

total_bytes += self.cursor_y.write(writer)?;;

total_bytes += self.cursor_z.write(writer)?;;

total_bytes += self.inside_block.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let hand : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let direction : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let cursor_x : f32 = PacketContent :: read ( reader ) ?;;

let cursor_y : f32 = PacketContent :: read ( reader ) ?;;

let cursor_z : f32 = PacketContent :: read ( reader ) ?;;

let inside_block : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand, location, direction, cursor_x, cursor_y, cursor_z, inside_block } ) } }

 }

 pub use sb_packet_block_place ::*;
