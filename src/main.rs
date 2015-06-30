use std::io::*;
use std::net::TcpStream;

fn main() {
	let mut stream = TcpStream::connect("irc.freenode.net:6667").unwrap();

	stream.write_all(b"NICK skript_kitty\r\n");
	stream.write_all(b"USER skript_kitty skript_kitty skript_kitty :meow\r\n");
	stream.write_all(b"JOIN #pbsideachannel\r\n");

	let mut read_head = BufReader::new(stream);
	// let mut take = String::new();
	while true {
		let mut take = String::new();
		read_head.read_line(&mut take);
		println!("{}", take);
	}
}