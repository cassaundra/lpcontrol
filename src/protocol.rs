pub const DEFAULT_PORT: u16 = 35502;
pub const LOCAL_ADDRESS: [usize; 4] = [127, 0, 0, 1];

use num_derive::FromPrimitive;
use std::net::Ipv4Addr;

#[derive(Debug, FromPrimitive)]
pub enum Command {
	Clear = 0x00,
	SetColor = 0x01
}