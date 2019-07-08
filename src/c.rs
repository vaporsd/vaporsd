use termion::color;

pub fn head(version: &'static str) {
	println!(" _ _ ___ ___ ___ ___ ___");
	println!("| | | .'| . | . |  _|_ -|   /\\");
	println!(" \\_/|__,|  _|___|_| |___|  / ,\\");
	println!("        |_|                \\__/ v{}\n", version);
}

pub fn ok() -> String {
	format!("{}✔{}", color::Fg(color::Green), color::Fg(color::Reset))
}
pub fn ko() -> String {
	format!("{}✘{}", color::Fg(color::Red), color::Fg(color::Reset))
}
pub fn wait() -> String {
	format!("{}↻{}", color::Fg(color::Yellow), color::Fg(color::Reset))
}
