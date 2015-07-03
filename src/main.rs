// use std::iter;
use std::io::*;
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

fn main() {
	let mut stream = TcpStream::connect("irc.freenode.net:6667").unwrap();

	let name = "skript_kitty";
	let channel = "#testingsnoopyq";
	// let channel = "#pbsideachannel";

	let _ = stream.write_all(format!("NICK {n}\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("USER {n} {n} {n} :meow\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("JOIN {c}\r\n", c = channel).as_bytes());

	let log_path = Path::new("log.txt");

	let mut log_file = match File::create(&log_path) {
		Err(why) => (panic!("error {}", why)),
		Ok(log_file) => log_file,
	};

	let mut read_head = BufReader::new(stream);

	let _ = stream.write_all("thsi is cats".as_bytes());

	while true {
		let mut take = String::new();
		// let _ = stream.read_to_string(&mut take);
		// let mut read_head = BufReader::new(take);

		// let take_line = String::new();
		// let _ = read_head.read_line(&mut take_line).unwrap();

		read_head.read_line(&mut take).unwrap();
		log_file.write_all(take.as_bytes());

		// if take.contains("go to sleep") { break; }

		// if check_pm(take, "test") {
		// 	send_msg(&stream, "yah that worked", channel);
		// }
	}
}

// fn check_pm(t: String, msg: &str) -> bool {
// 	let mut tick_list = false;

// 	let read_stack: Vec<&str> = t.splitn(4, " ").collect();
// 	if read_stack.len() == 4 {
// 		if read_stack[1].contains(" PRIVMSG ") { 
// 			if read_stack[3].contains(msg) {
// 				tick_list = true;
// 			}
// 		}
// 	}
// 	else {
// 		tick_list = false;
// 	}

// 	return tick_list;
// }

// fn send_msg(s: &TcpStream, msg: &str, channel: &str) {
// 	s.write_all(format!("PRIVMSG {c} :{m}\r\n", c = channel, m = msg).as_bytes());
// }