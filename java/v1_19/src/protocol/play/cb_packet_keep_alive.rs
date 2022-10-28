use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketKeepAlive ; impl Packet for CbPacketKeepAlive { type PacketIDType = i32 ; type PacketContent = PacketKeepAliveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 33 } } pub struct PacketKeepAliveContent { pub keep_alive_id: i64 ,

 } impl PacketContent for PacketKeepAliveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.keep_alive_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let keep_alive_id : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { keep_alive_id } ) } }
