pub mod config;
mod github;

/// Command line Arguments.
#[derive(Clone, Debug, clap::Parser)]
pub struct Options {
	/// The command to execute.
	#[clap(subcommand)]
	command: Command,
}

fn main() {
	if let Err(()) = do_main(clap::Parser::parse()) {
		std::process::exit(1)
	}
}

/// Available commands.
#[derive(Debug, Clone, clap::Parser)]
enum Command {
	/// Retrieve the existing comments of a PR and print them.
	GetComments(GetCommentsCommand),
}

#[derive(Debug, Clone, clap::Parser)]
pub struct GetCommentsCommand {
	#[clap(short, long)]
	/// Pull request number.
	pub pull_number: u64,
}


fn do_main(options: Options) -> Result<(), ()> {
	let config = config::parse("config.json").map_err(|e| println!("Error while parsing: {}", e))?;
	let github_client = make_github_client(config.clone())?;

	// Delegate commands to appropriate handlers.
	match &options.command {
		Command::GetComments(command) => github::get_pull_request_comments(command, github_client, config)?,
	}
	Ok(())
}



fn make_github_client(config: config::Config) -> Result<github::GitHubClient, ()> {
	github::GitHubClient::new(config)
}
