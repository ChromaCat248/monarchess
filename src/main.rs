// Monarchess Discord bot
// Main
// by ChromaCat248

use termion::color;
use futures::executor::block_on;
use std::{fs, env};
use yaml_rust::YamlLoader;
use serenity::*;

mod output;

static CONFIG_PATH : &str = "config.yaml";


struct EventHandler;
#[async_trait]
impl prelude::EventHandler for EventHandler {
	async fn message(&self, _ctx: prelude::Context, _msg: model::channel::Message) {

	}

	async fn ready(&self, _ctx: prelude::Context, rdy: model::gateway::Ready) {
		println!("{} is connected!", rdy.user.name);
	}
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

	let token : &str;
	let prefix : &str;

	if config["token"].as_str() != None {
		token = config["token"].as_str().unwrap();
	} else { output::error("Key \"token\" not found in config.yaml"); return }

	if config["prefix"].as_str() != None {
		prefix = config["prefix"].as_str().unwrap();
	} else { output::error("Key \"prefix\" not found in config.yaml"); return }

	if client::validate_token(token).is_err() {
		output::error("The provided token was invalid");
		return;
	}

	output::success("Config successfully loaded.");

	println!("Token: [not shown]\nPrefix: {}", prefix);
	println!("");


	output::action("Starting bot");

	block_on(async {
		let mut client = Client::builder(&token).event_handler(EventHandler).await.expect("Error creating client");
		if let Err(why) = client.start().await {
			println!("{}", why);
			output::error("Client error");
		}
	});
}
