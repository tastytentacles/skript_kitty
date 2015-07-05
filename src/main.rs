

// use std::iter;
use std::io::*;
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

fn main() {
	let mut stream :TcpStream = TcpStream::connect("irc.freenode.net:6667").unwrap();

	let name = "skript_kitty";
	let channel = "#testingsnoopyq";
	// let channel = "#pbsideachannel";

	let _ = stream.write_all(format!("NICK {n}\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("USER {n} {n} {n} :meow\r\n", n = name).as_bytes());
	let _ = stream.write_all(format!("JOIN {c}\r\n", c = channel).as_bytes());

	// let log_path = Path::new("log.txt");
	//
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

		// println!("{}", take);
		// let _ = log_file.write_all(take.as_bytes());
		info!("{}", take);

		if take.contains("go to sleep") {
			send_msg(&mut write_head, "ok going to sleep", channel);
			break;
		}

		// if check_pm(take, "test") {
		// 	// send_msg(stream, "yah that worked", channel);
		// 	println!("yah that worked");
		// }
	}
}

// fn parse_type() -> u16 {
//
// }

fn send_msg(s: &mut BufWriter<TcpStream>, msg: &str, channel: &str) {
	s.write_all(format!("PRIVMSG {c} :{m}\r\n", c = channel, m = msg).as_bytes()).unwrap();
	s.flush().unwrap();
}

// fn check_pm(t: String, msg: &str) -> bool {
// 	let mut tick_list = false;
//
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
//
// 	return tick_list;
// }
