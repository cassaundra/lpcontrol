use launchpad::*;
use clap::{App, crate_authors, crate_description, crate_version};
use std::net::TcpListener;
use std::io::{Read, stdout, Write};

use log::*;

use lpcontrol::protocol;
use lpcontrol::protocol::{LOCAL_ADDRESS, Message};
use rmp_serde::Deserializer;
use serde::Deserialize;

#[derive(Debug)]
struct Color(u8, u8, u8);

impl Color {
	fn lerp(a: Self, b: Self, t: f32) -> Color {
		let lerp_1d = |a: u8, b: u8, t: f32| -> u8 {
			let diff = (b - a) as f32;
			a + (diff * t) as u8
		};

		Color(lerp_1d(a.0, b.0, t), lerp_1d(a.1, b.1, t), lerp_1d(a.2, b.2, t))
	}
}

fn main() {
	// TODO custom port
	let matches = App::new("lpcontrol daemon")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.get_matches();

	env_logger::init();

	let mut device = LaunchpadMk2::guess();

	device.light_all(0);

	start_ipc(&mut device);
}

fn start_ipc(device: &mut LaunchpadMk2) {
	// open TCP socket for interprocess communication
	let listener = TcpListener::bind(LOCAL_ADDRESS).unwrap();

	info!("Listening on TCP port {}", protocol::PORT);

	let mut color = Color(0, 0, 0);
	let mut current_color = color;
	let duration: u32 = 0;

	for stream in listener.incoming() {
		match stream {
			Ok(mut stream) => {
				let mut buffer = [0; 256];
				stream.read(&mut buffer).unwrap();

				let cmd = rmp::decode::read_u8(&mut &buffer[..]).unwrap();
				let cmd = num_traits::FromPrimitive::from_u8(cmd);

				match cmd {
					Some(Message::Clear) => {
						device.light_all(0);
					},
					Some(Message::SetColor) => {
						// TODO validate input

						let mut de = Deserializer::new(&buffer[2..]);
						let packet: (u8, u8, u8, u32) = Deserialize::deserialize(&mut de).unwrap();

						color = Color(packet.0, packet.1, packet.2);

						let midi_color = nearest_palette(packet.0, packet.1, packet.2);
						device.light_all(midi_color);
					}
					_ => {}
				}
			}
			Err(e) => {
				eprintln!("Unable to connect: {}", e);
			}
		}
	}
}