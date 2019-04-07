use std::net::SocketAddr;

pub const PORT: &str = "35502";
pub const LOCAL_ADDRESS: &str = "127.0.0.1:35502";

use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum Message {
	Clear = 0x00,
	SetColorRaw = 0x01,
	SetColorRGB = 0x02
}