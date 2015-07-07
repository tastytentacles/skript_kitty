use std::io::*;
use std::net::TcpStream;


struct IRC {
	stream :TcpStream,
	read :BufReader<TcpStream>,
	write :BufWriter<TcpStream>
}

static NAME :&'static str = "skript_kitty";
static CHANNEL :&'static str = "#testingsnoopyq";

fn main() {
	let mut client = irc_new("irc.freenode.net:6667");
	irc_join(&mut client);

	loop {
		let mut take = String::new();
		client.read.read_line(&mut take).unwrap();

		println!("{}", take);

		if take.starts_with("PING") {
			let _ = client.write.write_all("PONG: nope".as_bytes());
			let _ = client.write.flush();
		}

		// ---------------------------------- # kill command #
		if msg_contains(&take, "go to sleep") {
			msg_send(&mut client.write, "ok I am off to sleep");
			break; }

		// ---------------------------------- # simple hello responce #
		if msg_contains(&take, "hello")
			{ msg_send(&mut client.write, "hello"); }
	}
}

fn msg_send(s: &mut BufWriter<TcpStream>, msg: &str) {
	s.write_all(format!("PRIVMSG {c} :{m}\r\n",
		c = CHANNEL, m = msg).as_bytes()).unwrap();
	s.flush().unwrap();
}

fn msg_contains(s: &String, t: &str) -> bool {
	let frac: Vec<&str> = s.splitn(4, " ").collect();

	if frac.len() != 4 { return false; }
	if frac[1] == "PRIVMSG" &&
	 	String::from_utf8_lossy(frac[3].as_bytes()).contains(t){
		return true;
	}
	else { return false; }
}

fn irc_new(url :&str) -> IRC {
	let tcp = TcpStream::connect(url).unwrap();
	let br = BufReader::new(tcp.try_clone().unwrap());
	let bw = BufWriter::new(tcp.try_clone().unwrap());

	let irc_out = IRC {
		stream: tcp,
		read: br,
		write: bw
	};

	irc_out
}

fn irc_join(irc :&mut IRC) {
	irc.write.write_all(format!("NICK {n}\r\n", n = NAME).as_bytes()).unwrap();
	irc.write.write_all(format!("USER {n} {n} {n} :meow\r\n", n = NAME).as_bytes()).unwrap();
	irc.write.write_all(format!("JOIN {c}\r\n", c = CHANNEL).as_bytes()).unwrap();
	irc.write.flush();
}
