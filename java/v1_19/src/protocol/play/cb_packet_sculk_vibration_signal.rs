use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSculkVibrationSignal ; impl Packet for CbPacketSculkVibrationSignal { type PacketIDType = i32 ; type PacketContent = PacketSculkVibrationSignalContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 5 } } pub struct PacketSculkVibrationSignalContent { pub source_position: minecraft_protocol::data::position::Position ,

pub destination_identifier: String ,

pub destination: PacketSculkVibrationSignalContentContent ,

pub arrival_ticks: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketSculkVibrationSignalContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.source_position.write(writer)?;;

total_bytes += self.destination_identifier.write(writer)?;;

total_bytes += self.destination.switch_write(false,writer)?;;

total_bytes += self.arrival_ticks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let source_position : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let destination_identifier : String = PacketContent :: read ( reader ) ?;;

let destination : PacketSculkVibrationSignalContentContent = PacketSwitch::switch_read(&destinationIdentifier,reader)?;;

let arrival_ticks : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { source_position, destination_identifier, destination, arrival_ticks } ) } } pub enum PacketSculkVibrationSignalContentContent { /// This switch variant requires a value block in the compare_to field

 Block (minecraft_protocol::data::position::Position ) ,

/// This switch variant requires a value entityId in the compare_to field

 EntityId (minecraft_protocol::data::VarInt ) ,

 } impl PacketSwitch for PacketSculkVibrationSignalContentContent { type CompareType = String ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
