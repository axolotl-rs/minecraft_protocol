use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAdvancements ; impl Packet for CbPacketAdvancements { type PacketIDType = i32 ; type PacketContent = PacketAdvancementsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 99 } } pub struct PacketAdvancementsContent { pub reset: bool ,

pub advancement_mapping: PacketAdvancementsContentArray ,

pub identifiers: PacketAdvancementsContentArray ,

pub progress_mapping: PacketAdvancementsContentArray ,

 } impl PacketContent for PacketAdvancementsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reset.write(writer)?;;

total_bytes += self.advancement_mapping.write(writer)?;;

total_bytes += self.identifiers.write(writer)?;;

total_bytes += self.progress_mapping.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let reset : bool = PacketContent :: read ( reader ) ?;;

let advancement_mapping : PacketAdvancementsContentArray = PacketContent :: read ( reader ) ?;;

let identifiers : PacketAdvancementsContentArray = PacketContent :: read ( reader ) ?;;

let progress_mapping : PacketAdvancementsContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reset, advancement_mapping, identifiers, progress_mapping } ) } } pub type PacketAdvancementsContentArray = Vec <PacketAdvancementsContentArrayContent >; pub struct PacketAdvancementsContentArrayContent { pub key: String ,

pub value: PacketAdvancementsContentArrayContentContent ,

 } impl PacketContent for PacketAdvancementsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.key.write(writer)?;;

total_bytes += self.value.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let key : String = PacketContent :: read ( reader ) ?;;

let value : PacketAdvancementsContentArrayContentContent = PacketContent :: read ( reader ) ?;;

 Ok ( Self { key, value } ) } } pub struct PacketAdvancementsContentArrayContentContent { pub parent_id: void ,

pub display_data: void ,

pub criteria: PacketAdvancementsContentArrayContentContentArray ,

pub requirements: PacketAdvancementsContentArrayContentContentArray ,

 } impl PacketContent for PacketAdvancementsContentArrayContentContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.parent_id.write(writer)?;;

total_bytes += self.display_data.write(writer)?;;

total_bytes += self.criteria.write(writer)?;;

total_bytes += self.requirements.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let parent_id : void = PacketContent :: read ( reader ) ?;;

let display_data : void = PacketContent :: read ( reader ) ?;;

let criteria : PacketAdvancementsContentArrayContentContentArray = PacketContent :: read ( reader ) ?;;

let requirements : PacketAdvancementsContentArrayContentContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { parent_id, display_data, criteria, requirements } ) } } pub type PacketAdvancementsContentArrayContentContentArray = Vec <PacketAdvancementsContentArrayContentContentArrayContent >; pub struct PacketAdvancementsContentArrayContentContentArrayContent { pub key: String ,

pub value: minecraft_protocol::data::Void ,

 } impl PacketContent for PacketAdvancementsContentArrayContentContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.key.write(writer)?;;

total_bytes += self.value.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let key : String = PacketContent :: read ( reader ) ?;;

let value : minecraft_protocol::data::Void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { key, value } ) } }

pub type PacketAdvancementsContentArrayContentContentArray = Vec <PacketAdvancementsContentArrayContentContentArrayArray >; pub type PacketAdvancementsContentArrayContentContentArrayArray = Vec <String >;

pub type PacketAdvancementsContentArray = Vec <String >;

pub type PacketAdvancementsContentArray = Vec <PacketAdvancementsContentArrayContent >; pub struct PacketAdvancementsContentArrayContent { pub key: String ,

pub value: PacketAdvancementsContentArrayContentArray ,

 } impl PacketContent for PacketAdvancementsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.key.write(writer)?;;

total_bytes += self.value.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let key : String = PacketContent :: read ( reader ) ?;;

let value : PacketAdvancementsContentArrayContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { key, value } ) } } pub type PacketAdvancementsContentArrayContentArray = Vec <PacketAdvancementsContentArrayContentArrayContent >; pub struct PacketAdvancementsContentArrayContentArrayContent { pub criterion_identifier: String ,

pub criterion_progress: void ,

 } impl PacketContent for PacketAdvancementsContentArrayContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.criterion_identifier.write(writer)?;;

total_bytes += self.criterion_progress.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let criterion_identifier : String = PacketContent :: read ( reader ) ?;;

let criterion_progress : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { criterion_identifier, criterion_progress } ) } }
