use launchpad::*;
use clap::{App, crate_authors, crate_description, crate_version};
use std::net::TcpListener;
use std::io::Read;

use log::*;

use lpcontrol::protocol;
use lpcontrol::protocol::{LOCAL_ADDRESS, Message};

fn main() {
	let _ = App::new("lpcontrol daemon")
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

	for stream in listener.incoming() {
		match stream {
			Ok(mut stream) => {
				let mut buffer = [0; 8];
				stream.read(&mut buffer).unwrap();

				let cmd = num_traits::FromPrimitive::from_u8(buffer[0]);

				match cmd {
					Some(Message::Clear) => {
						device.light_all(0);
					},
					Some(Message::SetColorRaw) => {
						// TODO validate input
						device.light_all(buffer[1]);
					},
					Some(Message::SetColorRGB) => {
						// TODO validate input
						let color = nearest_palette(buffer[1], buffer[2], buffer[3]);
						device.light_all(color);
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