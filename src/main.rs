use std::process::exit;

use cargo_ver::cmd::Cmd;

fn main() {
	if let Err(e) = Cmd::from_args().and_then(Cmd::run) {
		eprintln!("error: {}", e);
		exit(2);
	}
}
