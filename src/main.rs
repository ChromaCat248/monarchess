// Monarchess Discord bot
// Main
// by ChromaCat248

use termion::color;

mod output;

fn main()
{

	println!("");
	println!("");
	println!("{}     Monarchess Discord bot{}",
				color::Fg(color::LightWhite),
				color::Fg(color::Reset)
	);
	println!("{}     Project repository: {}https://github.com/ChromaCat248/monarchess{}",
				color::Fg(color::LightWhite),
				color::Fg(color::LightCyan),
				color::Fg(color::Reset)
	);
	println!("");

	println!("Output color cheat sheet:");
	output::info("Info");
	output::action("Action");
	output::success("Success");
	output::warning("Warning");
	output::error("Error");
	println!("");
	println!("");


	output::action("Doing bot stuff");

	println!("");

}
