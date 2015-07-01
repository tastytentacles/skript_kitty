// use std::iter;
use std::io::*;
use std::net::TcpStream;

fn main() {
	let mut stream = TcpStream::connect("irc.freenode.net:6667").unwrap();

	let name = "skript_kitty";
	let channel = "#testingsnoopyq";
	// let channel = "#pbsideachannel";

	let _ = stream.write_all(format!("NICK {n}\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("USER {n} {n} {n} :meow\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("JOIN {c}\r\n", c = channel).as_bytes());

	let mut read_head = BufReader::new(stream);
	while true {
		let mut take = String::new();
		read_head.read_line(&mut take).unwrap();
		// println!("{}", take);

		if take.contains("go to sleep") { break; }

		let read_stack: Vec<&str> = take.splitn(3, ":").collect();
		if read_stack.len() == 3 {
			println!("{}", read_stack[2]);
		}
	}
}
