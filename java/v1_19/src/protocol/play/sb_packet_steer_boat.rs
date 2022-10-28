use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSteerBoat ; impl Packet for SbPacketSteerBoat { type PacketIDType = i32 ; type PacketContent = PacketSteerBoatContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 22 } } pub struct PacketSteerBoatContent { pub left_paddle: bool ,

pub right_paddle: bool ,

 } impl PacketContent for PacketSteerBoatContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.left_paddle.write(writer)?;;

total_bytes += self.right_paddle.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let left_paddle : bool = PacketContent :: read ( reader ) ?;;

let right_paddle : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { left_paddle, right_paddle } ) } }
