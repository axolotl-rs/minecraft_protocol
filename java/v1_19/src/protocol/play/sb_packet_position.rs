mod sb_packet_position { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketPosition ; impl Packet for SbPacketPosition { type PacketIDType = i32 ; type PacketContent = PacketPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 17 } } pub struct PacketPositionContent { x: f64 ,

y: f64 ,

z: f64 ,

on_ground: bool ,

 } impl PacketContent for PacketPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, y, z, on_ground } ) } }

 }

 pub use sb_packet_position ::*;