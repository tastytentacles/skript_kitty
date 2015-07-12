use std::io::*;
use std::net::TcpStream;


pub struct IRC {
	pub stream :TcpStream,
	pub read :BufReader<TcpStream>,
	pub write :BufWriter<TcpStream>
}

pub struct ClientData {
	pub name :&'static str,
	pub channel :Vec<&'static str>
}

pub fn irc_new(url :&str) -> IRC {
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

pub fn irc_join(irc :&mut IRC, irc_data :&ClientData) {
	irc.write.write_all(format!("NICK {n}\r\n", n = irc_data.name).as_bytes()).unwrap();
	irc.write.write_all(format!("USER {n} {n} {n} :meow\r\n", n = irc_data.name).as_bytes()).unwrap();
	irc.write.write_all(format!("JOIN {c}\r\n", c = irc_data.channel[0]).as_bytes()).unwrap();
	irc.write.flush().unwrap();
}
