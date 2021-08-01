use std::collections::HashMap;
use std::collections::BTreeMap;

use serde_derive::{Serialize, Deserialize};
use serde_json;
use structopt::StructOpt;
use regex::Regex;

mod args;
mod util;
mod setting;
mod functions;
mod config;

use args::Args;

fn main() {
    let args = Args::from_args();

    let config = functions::get_config();
	let registry = functions::get_registry(args);

	if registry.is_err() {
		return;
	}

	let registry = registry.unwrap();
}
