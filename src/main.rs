// Monarchess Discord bot
// Main
// by ChromaCat248

use termion::color;
use std::{fs, env, thread, time};
use yaml_rust::YamlLoader;
use serenity::*;

mod output;

static CONFIG_PATH : &str = "config.yaml";

pub static mut TOKEN : &str = "";
pub static mut PREFIX : &str = "";

struct event_handler;
impl client::EventHandler for event_handler {
	/*fn message(&self, context: serenity::Context, message: serenity::Message) {
		unimplemented!();
	}*/
}

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



	output::action(
			format!("Loading config from {}/{}..",
				env::current_exe().unwrap().parent().unwrap().to_str().unwrap(),
				CONFIG_PATH
			).as_str()
	);

	let config_text = fs::read_to_string(
			format!( "{}/{}",
				env::current_exe().unwrap().parent().unwrap().to_str().unwrap(),
				CONFIG_PATH
			)
	);

	if config_text.is_err()
	{
		println!("");
		output::error(
				format!( "Failed to load config: {}",
					config_text.unwrap_err()
				).as_str()
		);
		println!("");
		return;
	}


	output::action("Parsing config..");

	let config_arr = YamlLoader::load_from_str(config_text.unwrap().as_str()).unwrap();
	let config = &config_arr[0];

	let token = config["token"].as_str().unwrap();
	let prefix = config["prefix"].as_str().unwrap();

	output::success("Config successfully loaded.");

	println!("Token: [not shown]\nPrefix: {}", prefix);
	println!("");


	output::action("Establishing connection..");
	let mut client = client::ClientBuilder::new(token);


	thread::sleep( time::Duration::from_secs(10) );


	println!("");
	output::action("Exiting");
	println!("");

}
