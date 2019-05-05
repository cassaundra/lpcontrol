#![feature(try_trait)]

#[macro_use]
extern crate derive_more;

use std::io::Write;
use std::net::{TcpStream, SocketAddr};

use clap::{App, Arg, crate_authors, crate_description, crate_version, SubCommand, value_t, value_t_or_exit};
use log::*;
use rmp_serde::Serializer;
use serde::Serialize;

use lpcontrol::protocol::*;


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
		.arg(Arg::with_name("address")
			.short("addr"))
		.arg(Arg::with_name("port")
			.short("p"))
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
				.required(true)))
		.subcommand(SubCommand::with_name("flash")
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

	let port = value_t!(matches, "port", u16).unwrap_or(DEFAULT_PORT);

	env_logger::init();

	let address = SocketAddr::from((LOCAL_ADDRESS, port));

	match matches.subcommand() {
		("off", _) => {
			send_to_daemon(address, Command::Clear as u8, vec![])?;
		},
		("set", Some(set_matches)) => {
			// TODO clean
			// should panic
			let red = value_t_or_exit!(set_matches, "red", u16);
			let green = value_t_or_exit!(set_matches, "green", u16);
			let blue = value_t_or_exit!(set_matches, "blue", u16);

			send_to_daemon(address, Command::SetColor as u8, vec![red, green, blue])?;
		},
		("flash", Some(set_matches)) => {
			let red = value_t_or_exit!(set_matches, "red", u16);
			let green = value_t_or_exit!(set_matches, "green", u16);
			let blue = value_t_or_exit!(set_matches, "blue", u16);

			send_to_daemon(address, Command::FlashColor as u8, vec![red, green, blue])?;
		}
		("", None) => println!("No subcommand specified, try --help."),
		_ => unreachable!()
	}

	Ok(())
}

fn send_to_daemon(address: SocketAddr, msg_id: u8, payload: Vec<u16>) -> Result<(), std::io::Error> {
	// quick one-shot connection
	let mut stream = TcpStream::connect(address).unwrap();

	let mut buffer = Vec::new();

	rmp::encode::write_u8(&mut buffer, msg_id)?;

	payload.serialize(&mut Serializer::new(&mut buffer)).unwrap();

	stream.write(&buffer[..])?;
	stream.flush()?;

	info!("Successfully sent command of ID {}", msg_id);

	Ok(())
} // stream is closed