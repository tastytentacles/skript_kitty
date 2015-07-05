use std::io::*;
use std::net::TcpStream;
// use std::path::Path;
// use std::fs::File;

fn main() {
	let mut stream :TcpStream = TcpStream::connect("irc.freenode.net:6667").unwrap();

	let name = "skript_kitty";
	let channel = "#testingsnoopyq";
	// let channel = "#pbsideachannel";

	let _ = stream.write_all(format!("NICK {n}\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("USER {n} {n} {n} :meow\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("JOIN {c}\r\n", c = channel).as_bytes());

	// let log_path = Path::new("log.txt");

	// let mut log_file = match File::create(&log_path) {
	// 	Err(why) => (panic!("error {}", why)),
	// 	Ok(log_file) => log_file,
	// };

	let read_stream = stream.try_clone().unwrap();
	let mut read_head = BufReader::new(read_stream);
	let mut write_head = BufWriter::new(stream);

	while true {
		let mut take = String::new();
		read_head.read_line(&mut take).unwrap();

		println!("{}", take);
		// let _ = log_file.write_all(take.as_bytes());

		if take.starts_with("PING") {
			write_head.write_all("PONG: nope".as_bytes());
			write_head.flush();
		}

		if take.contains("go to sleep") {
			msg_send(&mut write_head, "ok going to sleep", channel);
			break;
		}

		if msg_contains(take, "test") {
			println!("cats though");
		}
	}
}

fn msg_send(s: &mut BufWriter<TcpStream>, msg: &str, channel: &str) {
	s.write_all(format!("PRIVMSG {c} :{m}\r\n", c = channel, m = msg).as_bytes()).unwrap();
	s.flush().unwrap();
}

fn msg_contains(s: String, t: &str) -> bool {
	let mut out = false;
	let frac: Vec<&str> = s.splitn(4, " ").collect();
	// let msg: String = ;

	if frac.len() != 4 { return false; }
	if frac[1] == "PRIVMSG" &&
	 	String::from_utf8_lossy(frac[3].as_bytes()).contains(t){
		return true;
	}
	else { return false; }
}
