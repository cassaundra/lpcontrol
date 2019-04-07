#![feature(try_trait)]

use clap::{App, crate_authors, crate_description, crate_version, Arg, SubCommand};
use std::net::TcpStream;
use std::io::{Write, Error};

use log::*;

use lpcontrol::protocol::*;

#[macro_use]
extern crate derive_more;

#[derive(Debug, From)]
enum ClientError {
	NoneError(std::option::NoneError),
	ParseError(std::num::ParseIntError)
}

fn main() -> Result<(), ClientError> {
	let matches = App::new("lpcontrol client")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(SubCommand::with_name("off")
			.about("Turn off all LEDs"))
		.subcommand(SubCommand::with_name("raw")
			.arg(Arg::with_name("value")
				.help("The color value to send to the device")
				.index(1)
				.required(true)))
		.subcommand(SubCommand::with_name("rgb")
			.arg(Arg::with_name("red")
				.index(1)
				.required(true))
			.arg(Arg::with_name("green")
				.index(2)
				.required(true))
			.arg(Arg::with_name("blue")
				.index(3)
				.required(true)))
		.get_matches();

	env_logger::init();

	match matches.subcommand_name() {
		Some("off") => {
			send_to_daemon(Message::Clear as u8, vec![]);
		},
		Some("raw") => {
			let matches = matches.subcommand_matches("raw")?;
			let value: u8 = matches.value_of("value")?.parse()?;
			send_to_daemon(Message::SetColorRaw as u8, vec![value])
		},
		Some("rgb") => {
			let matches = matches.subcommand_matches("rgb").unwrap();
			let red = matches.value_of("red")?.parse()?;
			let green = matches.value_of("green")?.parse()?;
			let blue = matches.value_of("blue")?.parse()?;
			send_to_daemon(Message::SetColorRGB as u8, vec![red, green, blue]);
		},
		_ => unimplemented!()
	}

	Ok(())
}

fn send_to_daemon(msg_id: u8, payload: Vec<u8>) {
	let mut stream = TcpStream::connect(LOCAL_ADDRESS).unwrap();

	let mut payload = payload;
	let mut buf = vec![msg_id];
	buf.append(&mut payload);

	&[msg_id].to_vec().append(&mut payload.to_vec());
	stream.write(buf.as_ref()).unwrap();
	// stream is closed
}