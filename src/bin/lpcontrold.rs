use std::io::Read;
use std::net::{SocketAddr, TcpListener};

use clap::{App, Arg, crate_authors, crate_description, crate_version, value_t};
use launchpad::*;
use log::*;
use rmp_serde::Deserializer;
use serde::Deserialize;

use lpcontrol::protocol;
use lpcontrol::protocol::{Command, DEFAULT_PORT, LOCAL_ADDRESS};

fn main() {
	// TODO custom port
	let matches = App::new("lpcontrol daemon")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(Arg::with_name("port"))
		.get_matches();

	let port = value_t!(matches, "port", u16).unwrap_or(DEFAULT_PORT);

	env_logger::init();

	let mut device = LaunchpadMk2::guess();

	device.light_all(0);

	start_ipc(&mut device, port);
}

fn start_ipc(device: &mut LaunchpadMk2, port: u16) {
	// open TCP socket for interprocess communication
	let listener = TcpListener::bind(SocketAddr::from((LOCAL_ADDRESS, port))).unwrap();

	info!("Listening on TCP port {}", protocol::DEFAULT_PORT);
	for stream in listener.incoming() {
		match stream {
			Ok(mut stream) => {
				let mut buffer = [0; 256];
				stream.read(&mut buffer).unwrap();

				let cmd = rmp::decode::read_u8(&mut &buffer[..]).unwrap();
				let cmd = num_traits::FromPrimitive::from_u8(cmd);

				// omit first two bytes indicating type
				execute_command(device, cmd, &buffer[2..]);
			}
			Err(e) => {
				eprintln!("Unable to connect: {}", e);
			}
		}
	}
}

fn execute_command(device: &mut LaunchpadMk2, command: Option<Command>, buffer: &[u8]) {
	match command {
		Some(Command::Clear) => {
			device.light_all(0);
		},
		Some(Command::SetColor) => {
			// TODO validate input

			let mut de = Deserializer::new(&buffer[..]);
			let packet: (u8, u8, u8, u32) = Deserialize::deserialize(&mut de).unwrap();

			let midi_color = nearest_palette(packet.0, packet.1, packet.2);
			device.light_all(midi_color);
		}
		_ => {}
	}
}