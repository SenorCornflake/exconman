use crate::util;
use crate::setting::Setting;
use crate::args::Args;

fn validate_setting(setting: &Setting, registry_path: &str) -> Result<(), ()> {
    let file = &util::expand_home(&setting.file);
    let name = &setting.name;

    match std::fs::metadata(file) {
        Err(err) => {
            println!(
                "Error occured while validating setting {}\"{}\"{} in {}\"{}\"{}: {}\"{}\" - {}{}",
                util::color("green", "fg"),
                name,
                util::color("white", "fg"),
                util::color("green", "fg"),
                registry_path,
                util::color("white", "fg"),
                util::color("red", "fg"),
                file,
                err.to_string() ,
                util::color("white", "fg")
            );
            return Err(());
        }
        Ok(metadata) => {
            if metadata.is_dir() {
                println!(
                    "Error occured while validating setting {}\"{}\"{} in {}\"{}\"{}: {}{} is a directory{}",
                    util::color("green", "fg"),
                    name,
                    util::color("white", "fg"),
                    util::color("green", "fg"),
                    registry_path,
                    util::color("white", "fg"),
                    util::color("red", "fg"),
                    file,
                    util::color("white", "fg")
                );
                return Err(());
            }

            return Ok(());
        }
    }
}

// Get the config file, if it exists
pub fn get_config() -> Option<String> {
    let config_metadata = std::fs::metadata(util::expand_home("~/.config/exconman/config.json"));

    if config_metadata.is_ok() {
        let config_metadata = config_metadata.unwrap();

        if config_metadata.is_dir() {
            println!("Config path {}\"~/.config/exconman/config.json\"{} is a directory.", util::color("green", "fg"), util::color("white", "fg"));
            None
        } else {
            let config_contents = std::fs::read_to_string(&util::expand_home("~/.config/exconman/config.json"));

            if config_contents.is_ok() {
                Some(config_contents.unwrap())
            } else {
                None
            }
        }
    } else {
        None
    }
}

pub fn get_registry(args: Args) -> Result<Vec<Setting>, ()> {
    let registry_path: Result<String, ()> = if args.registry.is_some() {
        let registry = args.registry.unwrap();
        Ok(registry)
    } else {
        // Decide which default registry path to use, either the file or the directory
        let registry_file = util::expand_home("~/.config/exconman/registry.json");
        let registry_dir = util::expand_home("~/.config/exconman/registry");
        
        let file_results = std::fs::metadata(&registry_file);
        let dir_results = std::fs::metadata(&registry_dir);

        if file_results.is_err() && dir_results.is_err() {
            println!(
                "Default registry paths \"{}{}{}\" and \"{}{}{}\" do not exist.\n\nCreate one of them or provide a custom path using {}--registry{}",
                util::color("green", "fg"),
                registry_file,
                util::color("white", "fg"),
                util::color("green", "fg"),
                registry_dir,
                util::color("white", "fg"),
                util::color("blue", "fg"),
                util::color("white", "fg"),
            );
            Err(())
        } else if file_results.is_ok() {
            Ok(registry_file)
        } else {
            Ok(registry_dir)
        }
    };

	if registry_path.is_err() {
		return Err(());
	}

	let registry_path = registry_path.unwrap();

	let registry: Result<Vec<Setting>, ()> = match std::fs::metadata(&registry_path) {
		Err(_) => {
			return Err(());
		}
		Ok(metadata) => {
			if metadata.is_file() {
				let registry = std::fs::read_to_string(&registry_path);

				if registry.is_err() {
                    println!(
                        "Failed to read {}\"{}\"{}",
                        util::color("green", "fg"),
                        registry_path,
                        util::color("white", "fg")
                    );
					return Err(());
				}

				let registry = registry.unwrap();
				let registry: Result<Vec<Setting>, serde_json::Error> = serde_json::from_str(&registry);

				if registry.is_err() {
					eprintln!(
						"JSON Error in {}\"{}\"{}: {}{}{}",
						util::color("green", "fg"),
						registry_path,
						util::color("white", "fg"),
						util::color("red", "fg"),
						registry.unwrap_err(),
						util::color("white", "fg"),
					);
					return Err(());
				}
				
				let registry = registry.unwrap();

                let mut error_occured = false;

                for setting in &registry {
                    let result = validate_setting(setting, &registry_path);
                    if result.is_err() {
                        error_occured = true;
                    }
                }

                if error_occured {
                    return Err(());
                }

				Ok(registry)
			} else {
                let files: Vec<Result<std::fs::DirEntry, _>> = std::fs::read_dir(registry_path)
					.unwrap()
					.collect();
                let mut joined_registry: Vec<Setting> = Vec::new();

				for file in files {
                    let registry_path = file.unwrap().path();
                    let registry_path = registry_path.display();
                    let registry_path = registry_path.to_string();

                    let registry = std::fs::read_to_string(&registry_path);

					// TODO: Put error message here
					if registry.is_err() {
						return Err(())
					}

					let registry = registry.unwrap();
                    let registry: Result<Vec<Setting>, serde_json::Error> = serde_json::from_str(&registry);

					if registry.is_err() {
						eprintln!(
							"JSON Error in {}\"{}\"{}: {}{}{}",
							util::color("green", "fg"),
							registry_path,
							util::color("white", "fg"),
							util::color("red", "fg"),
							registry.unwrap_err(),
							util::color("white", "fg"),
						);
						return Err(());
					}

					let mut registry = registry.unwrap();

                    // Validate registry settings
                    let mut error_occured = false;

                    for setting in &registry {
                        let result = validate_setting(setting, &registry_path);
                        if result.is_err() {
                            error_occured = true;
                        }
                    }

                    if error_occured {
                        return Err(());
                    }

					joined_registry.append(&mut registry);
				}
				
				Ok(joined_registry)
			}
		}
	};

	if registry.is_err() {
		return Err(())
	}

	Ok(registry.unwrap())
}
