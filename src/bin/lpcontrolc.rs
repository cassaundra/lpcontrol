use clap::{App, crate_authors, crate_description, crate_version, Arg, SubCommand};
use std::net::TcpStream;
use std::io::Write;

use log::*;

use lpcontrol::protocol::*;

fn main() {
	let matches = App::new("lpcontrol client")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(SubCommand::with_name("color")
			.arg(Arg::with_name("color")
			.help("The color to set the Launchpad to")
			.index(1)
			.required(true)))
		.get_matches();

	env_logger::init();

	if let Some(matches) = matches.subcommand_matches("color") {
		let color = matches.value_of("color").unwrap();
		send_to_daemon(Message::SetColor as u8, color.parse().unwrap());
	}
}

fn send_to_daemon(msg_id: u8, payload: u8) {
	let mut stream = TcpStream::connect(LOCAL_ADDRESS).unwrap();
	stream.write(&[msg_id, payload]).unwrap();
	// stream is closed
}