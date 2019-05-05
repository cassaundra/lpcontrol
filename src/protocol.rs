use num_derive::FromPrimitive;

pub const DEFAULT_PORT: u16 = 35502;
pub const LOCAL_ADDRESS: [u8; 4] = [127, 0, 0, 1];

#[derive(Debug, FromPrimitive)]
pub enum Command {
	// Set all pads to off
	Clear = 0x00,
	// Set all pads to a color
	SetColor = 0x01,
	// Flash all pads
	FlashColor = 0x02,
	PulseColor = 0x03
}