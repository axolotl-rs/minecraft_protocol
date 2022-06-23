mod cb_packet_update_light { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateLight ; impl Packet for CbPacketUpdateLight { type PacketIDType = i32 ; type PacketContent = PacketUpdateLightContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 37 } } pub struct PacketUpdateLightContent { chunk_x: minecraft_data::common::data::VarInt ,

chunk_z: minecraft_data::common::data::VarInt ,

trust_edges: bool ,

sky_light_mask: PacketUpdateLightContentArray ,

block_light_mask: PacketUpdateLightContentArray ,

empty_sky_light_mask: PacketUpdateLightContentArray ,

empty_block_light_mask: PacketUpdateLightContentArray ,

sky_light: PacketUpdateLightContentArray ,

block_light: PacketUpdateLightContentArray ,

 } impl PacketContent for PacketUpdateLightContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_x.write(writer)?;;

total_bytes += self.chunk_z.write(writer)?;;

total_bytes += self.trust_edges.write(writer)?;;

total_bytes += self.sky_light_mask.write(writer)?;;

total_bytes += self.block_light_mask.write(writer)?;;

total_bytes += self.empty_sky_light_mask.write(writer)?;;

total_bytes += self.empty_block_light_mask.write(writer)?;;

total_bytes += self.sky_light.write(writer)?;;

total_bytes += self.block_light.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let chunk_x : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let chunk_z : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let trust_edges : bool = PacketContent :: read ( reader ) ?;;

let sky_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let block_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let empty_sky_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let empty_block_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let sky_light : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let block_light : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_x, chunk_z, trust_edges, sky_light_mask, block_light_mask, empty_sky_light_mask, empty_block_light_mask, sky_light, block_light } ) } } pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <PacketUpdateLightContentArrayArray >; pub type PacketUpdateLightContentArrayArray = Vec <u8 >;

pub type PacketUpdateLightContentArray = Vec <PacketUpdateLightContentArrayArray >; pub type PacketUpdateLightContentArrayArray = Vec <u8 >;

 }

 pub use cb_packet_update_light ::*;