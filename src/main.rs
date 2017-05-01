extern crate termion;

use std::io::{Write, stdout, stdin};
use std::io::prelude::*;
use std::net::TcpStream;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

fn main()
{
	let mut stdout = MouseTerminal::from(
		stdout().into_raw_mode().unwrap()
	);

	loop
	{
		let mut stream = TcpStream::connect("127.0.0.1:9123").unwrap();
    	stream.write_all(b"Ring ring, it's the banana phone\r\n").unwrap();
		stream.flush().unwrap();

		let stdin = stdin();
		for c in stdin.events()
		{
			let evt = c.unwrap();
			write!(
				stdout,
				"{}{}Again? [Y/n]",
				termion::clear::All,
				termion::cursor::Goto(1,1)
			).unwrap();

			match evt
			{
				Event::Key(Key::Char('y')) => break,
				Event::Key(Key::Char('n')) => return,
				_ => {}
			}
			stdout.flush().unwrap();
		}
		stdout.flush().unwrap();
    }
}
