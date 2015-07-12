mod irc_h;

use std::io::*;
use std::net::TcpStream;


fn main() {
	let mut client_data_v = Vec::new();
	client_data_v.push("#testingsnoopyq");
	let client_data = irc_h::ClientData { name :"skript_kitty", channel :client_data_v };

	let mut client = irc_h::irc_new("irc.freenode.net:6667");
	irc_h::irc_join(&mut client, &client_data);

	loop {
		let mut take = String::new();
		client.read.read_line(&mut take).unwrap();

		println!("{}", take);

		// ---------------------------------- # PING #
		if take.starts_with("PING") {
			let _ = client.write.write_all("PONG: nope".as_bytes());
			let _ = client.write.flush();
		}

		// ---------------------------------- # kill command #
		if msg_contains(&take, "go to sleep") {
			msg_send(&mut client.write, &client_data, "ok I am off to sleep");
			break; }

		// ---------------------------------- # simple hello responce #
		if msg_contains(&take, "hello")
			{ msg_send(&mut client.write, &client_data, "hello"); }
	}
}



fn msg_send(s: &mut BufWriter<TcpStream>, cd :&irc_h::ClientData, msg: &str) {
	s.write_all(format!("PRIVMSG {c} :{m}\r\n",
		c = cd.channel[0], m = msg).as_bytes()).unwrap();
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
