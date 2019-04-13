use num_derive::FromPrimitive;

pub const DEFAULT_PORT: u16 = 35502;
pub const LOCAL_ADDRESS: [u8; 4] = [127, 0, 0, 1];

#[derive(Debug, FromPrimitive)]
pub enum Command {
	Clear = 0x00,
	SetColor = 0x01
}