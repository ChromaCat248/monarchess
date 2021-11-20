// Monarchess Discord Bot
// Command line output
// by ChromaCat248

use termion::color;

pub fn info(text: &str) {
	println!("{}==>{} {}{}",
				color::Fg(color::LightMagenta),
				color::Fg(color::LightWhite),
				text,
				color::Fg(color::Reset)
	);
}

pub fn action(text: &str) {
	println!("{}==>{} {}{}",
				color::Fg(color::LightBlue),
				color::Fg(color::LightWhite),
				text,
				color::Fg(color::Reset)
	);
}

pub fn success(text: &str) {
	println!("{}==>{} {}{}",
				color::Fg(color::LightGreen),
				color::Fg(color::LightWhite),
				text,
				color::Fg(color::Reset)
	);
}

pub fn warning(text: &str) {
	println!("{}==>{} {}{}",
				color::Fg(color::LightYellow),
				color::Fg(color::LightWhite),
				text,
				color::Fg(color::Reset)
	);
}

pub fn error(text: &str) {
	println!("{}==>{} {}{}",
				color::Fg(color::LightRed),
				color::Fg(color::LightWhite),
				text,
				color::Fg(color::Reset)
	);
}
