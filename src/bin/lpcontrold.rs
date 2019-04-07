use std::time::Duration;
use std::thread;
use launchpad::*;
use clap::{App, crate_authors, crate_description, crate_version};
use std::fs::File;
use std::net::{TcpStream, TcpListener};
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
	let timeout = Duration::from_millis(1);

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
					Some(protocol::Message::SetColor) => {
						// TODO validate color
						device.light_all(buffer[1]);
					},
					_ => {}
				}
			}
			Err(e) => {
				eprintln!("Unable to connect: {}", e);
			}
		}
	}
}