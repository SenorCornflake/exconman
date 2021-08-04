use structopt::StructOpt;

mod args;
mod util;
mod setting;
mod functions;
mod config;

use args::{Args, SubCommands, Set, Get, Load};

fn main() {
    let args = Args::from_args();
    let subcommand = args.sub;

    let config = functions::get_config();
	let registry = functions::get_registry(args.registry);

	if registry.is_err() {
		return;
	}

	let registry = registry.unwrap();

    match subcommand {
        SubCommands::Set(Set {name, value}) => {
            functions::set(name, value, &config, &registry);
        }
        SubCommands::Get(Get { name }) => {
            functions::get(name, true, &config, &registry);
        }
        SubCommands::Load(Load { path }) => {
            functions::load(path, &config, &registry);
        }
        SubCommands::Dump => {
            functions::dump(&config, &registry);
        }
    }
}
