#![feature(try_trait)]

use clap::{App, crate_authors, crate_description, crate_version, Arg, SubCommand};
use std::net::TcpStream;
use std::io::Write;

use log::*;

use lpcontrol::protocol::*;
use rmp_serde::Serializer;
use serde::Serialize;

#[macro_use]
extern crate derive_more;

#[derive(Debug, From)]
enum ClientError {
	NoneError(std::option::NoneError),
	ParseError(std::num::ParseIntError),
	IoError(std::io::Error)
}

fn main() -> Result<(), ClientError> {
	let matches = App::new("lpcontrol client")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(SubCommand::with_name("off")
			.about("Turn off all LEDs"))
		.subcommand(SubCommand::with_name("set")
			.about("Set all LEDs to an RGB color (0-255)")
			.arg(Arg::with_name("red")
				.index(1)
				.required(true))
			.arg(Arg::with_name("green")
				.index(2)
				.required(true))
			.arg(Arg::with_name("blue")
				.index(3)
				.required(true))
			.arg(Arg::with_name("duration")
				.index(4)
				.required(true)))
		.get_matches();

	env_logger::init();

	match matches.subcommand() {
		("off", _) => {
			send_to_daemon(Message::Clear as u8, vec![])?;
		},
		("set", Some(set_matches)) => {
			let red = set_matches.value_of("red")?.parse()?;
			let green = set_matches.value_of("green")?.parse()?;
			let blue = set_matches.value_of("blue")?.parse()?;
			let duration = set_matches.value_of("duration")?.parse()?;

			send_to_daemon(Message::SetColor as u8, vec![red, green, blue, duration])?;
		}
		("", None) => println!("No subcommand specified, try --help."),
		_ => unreachable!()
	}

	Ok(())
}

fn send_to_daemon(msg_id: u8, payload: Vec<u16>) -> Result<(), std::io::Error> {
	let mut stream = TcpStream::connect(LOCAL_ADDRESS).unwrap();

	let mut buffer = Vec::new();

	rmp::encode::write_u8(&mut buffer, msg_id)?;

	payload.serialize(&mut Serializer::new(&mut buffer)).unwrap();

	stream.write(&buffer[..])?;
	stream.flush()?;

	info!("Successfully sent command of ID {}", msg_id);

	Ok(())
} // stream is closed