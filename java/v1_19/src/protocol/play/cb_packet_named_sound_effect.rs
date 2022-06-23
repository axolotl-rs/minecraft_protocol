use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketNamedSoundEffect ; impl Packet for CbPacketNamedSoundEffect { type PacketIDType = i32 ; type PacketContent = PacketNamedSoundEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 25 } } pub struct PacketNamedSoundEffectContent { sound_name: String ,

sound_category: minecraft_data::data::VarInt ,

x: i32 ,

y: i32 ,

z: i32 ,

volume: f32 ,

pitch: f32 ,

 } impl PacketContent for PacketNamedSoundEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sound_name.write(writer)?;;

total_bytes += self.sound_category.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.volume.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let sound_name : String = PacketContent :: read ( reader ) ?;;

let sound_category : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : i32 = PacketContent :: read ( reader ) ?;;

let y : i32 = PacketContent :: read ( reader ) ?;;

let z : i32 = PacketContent :: read ( reader ) ?;;

let volume : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sound_name, sound_category, x, y, z, volume, pitch } ) } }
