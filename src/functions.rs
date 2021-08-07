use std::collections::HashMap;

use regex::Regex;

use crate::util;
use crate::setting::{Setting, Pattern, Replace};
use crate::config::Config;

// Get the config file, if it exists
pub fn get_config() -> Option<Config> {
    let config_metadata = std::fs::metadata(util::expand_home("~/.config/exconman/config.json"));

    if config_metadata.is_ok() {
        let config_metadata = config_metadata.unwrap();

        if config_metadata.is_dir() {
            eprintln!("Config path {}\"~/.config/exconman/config.json\"{} is a directory.", util::color("green", "fg"), util::color("white", "fg"));
            None
        } else {
            let config = std::fs::read_to_string(&util::expand_home("~/.config/exconman/config.json"));

            if config.is_ok() {
				let config: Result<Config, serde_json::Error> = serde_json::from_str(&config.unwrap());

                if config.is_err() {
                    eprintln!(
                        "Config Error: {}{}{}",
                        util::color("red", "fg"),
                        config.unwrap_err(),
                        util::color("white", "fg"),
                    );
                    return None
                }

                Some(config.unwrap())
            } else {
                None
            }
        }
    } else {
        None
    }
}

// Get the registry, whether it's a dir or file.
pub fn get_registry(registry: Option<String>) -> Result<Vec<Setting>, ()> {
    let registry_path: Result<String, ()> = if registry.is_some() {
        let registry = registry.unwrap();
        Ok(registry)
    } else {
        // Decide which default registry path to use, either the file or the directory
        let registry_file = util::expand_home("~/.config/exconman/registry.json");
        let registry_dir = util::expand_home("~/.config/exconman/registry");
        
        let file_results = std::fs::metadata(&registry_file);
        let dir_results = std::fs::metadata(&registry_dir);

        if file_results.is_err() && dir_results.is_err() {
            eprintln!(
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
		Err(error) => {
            eprintln!(
                "Failed to read {}\"{}\"{}: {}{}{}",
                util::color("green", "fg"),
                registry_path,
                util::color("white", "fg"),
                util::color("red", "fg"),
                error,
                util::color("white", "fg")
            );
			return Err(());
		}
		Ok(metadata) => {
			if metadata.is_file() {
				let registry = std::fs::read_to_string(&registry_path);

				if registry.is_err() {
                    eprintln!(
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

					if registry.is_err() {
                        eprintln!(
                            "Failed to read {}\"{}\"{}: {}{}{}",
                            util::color("green", "fg"),
                            registry_path,
                            util::color("white", "fg"),
                            util::color("red", "fg"),
                            registry.unwrap_err(),
                            util::color("white", "fg")
                        );
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

// Run a hook, does this by checking if the hook is a valid path, if it is, it runs it as a shell
// script, if it isn't a valid path, or the path is valid but it cannot execute it,
// it interprets it as a shell command and runs it
pub fn run_hook(hook_name: String, hook_command: String) {

    fn run(hook_type: &str, hook_name: &str, hook_command: &str) {
        let output: Result<std::process::Output, std::io::Error>;

        if hook_type == String::from("file") {
            output = std::process::Command::new("sh")
                .arg(hook_command)
                .output();
        } else {
            output = std::process::Command::new("sh")
                .arg("-c")
                .arg(hook_command)
                .output();
        }

        let stderr = String::from_utf8_lossy(&output.as_ref().unwrap().stderr);
        let stderr = stderr.trim_end();

        if output.is_err() {
            eprintln!(
                "Error running {}\"{}\"{} hook: {}Error occured while trying to run command \"{}\"{}",
                util::color("green", "fg"),
                hook_name,
                util::color("white", "fg"),
                util::color("red", "fg"),
                hook_command,
                util::color("white", "fg")
            );

            return;
        } else if stderr.len() > 0 {
            eprintln!(
                "Error running {}\"{}\"{} hook: {}{}{}",
                util::color("green", "fg"),
                hook_name,
                util::color("white", "fg"),
                util::color("red", "fg"),
                stderr,
                util::color("white", "fg")
            );

            return;
        }
    }

    match std::fs::metadata(util::expand_home(&hook_command)) {
        Err(_) => {
            run("command", &hook_name, &hook_command);
        },
        Ok(metadata) => {
            if metadata.is_dir() {
                run("command", &hook_name, &hook_command);
            } else {
                run("file", &hook_name, &hook_command);
            }
        }
    }
}

pub fn get_setting(setting_name: String, registry: &Vec<Setting>) -> Option<&Setting> {
    for setting in registry {
        if setting.name == setting_name {
            return Some(setting);
        }
    }
    return None;
}

pub fn set(name: String, value: String, config: &Option<Config>, registry: &Vec<Setting>) {

    if let Some(config) = config {
        if let Some(hook_before_set) = &config.hook_before_set {
            run_hook("hook_before_get".to_string(), hook_before_set.to_string());
        }
    }

    let setting = get_setting(name, registry);

    // TODO: Error message
    if setting.is_none() {
        return;
    }

    let setting = setting.unwrap();

    // Open the file   
    let file = std::fs::read_to_string(&util::expand_home(&setting.file));

    if file.is_err() {
        eprintln!(
            "Error opening file {}\"{}\"{} for setting {}\"{}\"{}: {}{}{}",
            util::color("green", "fg"),
            setting.file,
            util::color("white", "fg"),
            util::color("green", "fg"),
            setting.name,
            util::color("white", "fg"),
            util::color("red", "fg"),
            file.unwrap_err(),
            util::color("", "fg")
        );
        return;
    }

    let file = file.unwrap();
    // Split file into lines
    let mut file: Vec<&str> = file
        .split("\n")
        .collect();

    let substitute: String;

    if setting.read_value_path.is_some() && setting.read_value_path.unwrap() == true {
        let contents = std::fs::read_to_string(&util::expand_home(&value));

        if contents.is_err() {
            eprintln!(
                "Error opening file {}\"{}\"{} path provided in the value for setting {}\"{}\"{}: {}{}{}",
                util::color("green", "fg"),
                value,
                util::color("white", "fg"),
                util::color("green", "fg"),
                setting.name,
                util::color("white", "fg"),
                util::color("red", "fg"),
                contents.unwrap_err(),
                util::color("", "fg")
            );
            return;
        }

        let contents = contents.unwrap();
        substitute = setting.substitute.replace("{value}", &contents);
    } else {
        substitute = setting.substitute.replace("{value}", &value);
    }

    match &setting.pattern {
        Pattern::Line(pattern) => {
            let rgx = Regex::new(&pattern);

            if rgx.is_err() {
                eprintln!(
                    "Error occured while compiling regex for setting {}\"{}\"{}: {}{}{}",
                    util::color("green", "fg"),
                    setting.name,
                    util::color("white", "fg"),
                    util::color("red", "fg"),
                    rgx.unwrap_err(),
                    util::color("white", "fg"),
                );
                return;
            }

            let rgx = rgx.unwrap();

            for i in 0..file.len() {
                let line = file[i];

                if rgx.is_match(line) {
                    match &setting.replace {
                        None => {
                            file[i] = &substitute;
                        }
                        Some(replace) => {
                            match replace {
                                Replace::LineAbove => {
                                    if i != 0 {
                                        file[i - 1] = &substitute;
                                    }
                                }
                                Replace::MatchedText => {
                                    file[i] = &substitute;
                                }
                                Replace::LineBelow => {
                                    if i != file.len() - 1 {
                                        file[i + 1] = &substitute;
                                    }
                                }
                            }
                        }
                    }

                    if setting.multiple.is_none() || setting.multiple.unwrap() == false {
                        break;
                    }
                }
            }

        }
        Pattern::Region(region) => {
            let mut region_start: Option<usize> = None;
            let mut region_end: Option<usize> = None;
            
            let rgx_start = Regex::new(&region[0]);
            let rgx_end = Regex::new(&region[1]);

            if rgx_start.is_err() || rgx_end.is_err() {
                eprintln!(
                    "Error occured while compiling regex for setting {}\"{}\"{}: {}{}{}",
                    util::color("green", "fg"),
                    setting.name,
                    util::color("white", "fg"),
                    util::color("red", "fg"),
                    rgx_start.unwrap_err(),
                    util::color("white", "fg"),
                );
                return;
            }

            let rgx_start = rgx_start.unwrap();
            let rgx_end = rgx_end.unwrap();

            for i in 0..file.len() {
                let line = file[i];
                if rgx_start.is_match(line) {
                    region_start = Some(i);

                    if (setting.multiple.is_none() || setting.multiple.unwrap() == false) && region_end.is_some() {
                        break;
                    }
                }
                if rgx_end.is_match(line) {
                    region_end = Some(i);

                    if (setting.multiple.is_none() || setting.multiple.unwrap() == false) && region_start.is_some() {
                        break;
                    }
                }
            }

            if region_start.is_none() || region_end.is_none() || region_start.unwrap() >= region_end.unwrap() {
                return;
            }

            file.drain(region_start.unwrap() + 1..region_end.unwrap());
            file.insert(region_start.unwrap() + 1, &substitute);
        }
    }


    let file = file.join("\n");

    match std::fs::write(util::expand_home(&setting.file), file) {
        Err(error) => {
            eprintln!(
                "Failed to write to {}\"{}\"{}: {}{}{}",
                util::color("green", "fg"),
                setting.file,
                util::color("white", "fg"),
                util::color("red", "fg"),
                error,
                util::color("white", "fg"),
            );
        }
        _ => {}
    }

    if let Some(config) = config {
        if let Some(hook_after_set) = &config.hook_after_set {
            run_hook("hook_after_get".to_string(), hook_after_set.to_string());
        }
    }
}

pub fn get(name: String, print: bool, config: &Option<Config>, registry: &Vec<Setting>) -> Option<String> {
    // Don't run hooks when dumping
    if print {
        if let Some(config) = config {
            if let Some(hook_after_get) = &config.hook_after_get {
                run_hook("hook_after_get".to_string(), hook_after_get.to_string());
            }
        }
    }

    let setting = get_setting(name, registry);

    // TODO: Error message
    if setting.is_none() {
        return None;
    }

    let setting = setting.unwrap();

    // Open the file   
    let file = std::fs::read_to_string(&util::expand_home(&setting.file));

    if file.is_err() {
        eprintln!(
            "Error opening file {}\"{}\"{} for setting {}\"{}\"{}: {}{}{}",
            util::color("green", "fg"),
            setting.file,
            util::color("white", "fg"),
            util::color("green", "fg"),
            setting.name,
            util::color("white", "fg"),
            util::color("red", "fg"),
            file.unwrap_err(),
            util::color("", "fg")
        );
        return None;
    }

    let file = file.unwrap();
    // Split file into lines
    let file: Vec<&str> = file
        .split("\n")
        .collect();

    let mut text: String = String::new();

    match &setting.pattern {
        Pattern::Line(pattern) => {
            let rgx = Regex::new(&pattern);

            if rgx.is_err() {
                eprintln!(
                    "Error occured while compiling regex for setting {}\"{}\"{}: {}{}{}",
                    util::color("green", "fg"),
                    setting.name,
                    util::color("white", "fg"),
                    util::color("red", "fg"),
                    rgx.unwrap_err(),
                    util::color("white", "fg"),
                );
                return None;
            }

            let rgx = rgx.unwrap();

            for i in 0..file.len() {
                let line = file[i];

                if rgx.is_match(line) {
                    match &setting.replace {
                        None => {
                            text = file[i].to_string();
                        }
                        Some(replace) => {
                            match replace {
                                Replace::LineAbove => {
                                    if i != 0 {
                                        text = file[i - 1].to_string();
                                    }
                                }
                                Replace::MatchedText => {
                                    text = file[i].to_string();
                                }
                                Replace::LineBelow => {
                                    if i != file.len() - 1 {
                                        text = file[i + 1].to_string();
                                    }
                                }
                            }
                        }
                    }

                    if setting.multiple.is_none() || setting.multiple.unwrap() == false {
                        break;
                    }
                }
            }

        }
        Pattern::Region(region) => {
            let mut region_start: Option<usize> = None;
            let mut region_end: Option<usize> = None;
            
            let rgx_start = Regex::new(&region[0]);
            let rgx_end = Regex::new(&region[1]);

            if rgx_start.is_err() || rgx_end.is_err() {
                eprintln!(
                    "Error occured while compiling regex for setting {}\"{}\"{}: {}{}{}",
                    util::color("green", "fg"),
                    setting.name,
                    util::color("white", "fg"),
                    util::color("red", "fg"),
                    rgx_start.unwrap_err(),
                    util::color("white", "fg"),
                );
                return None;
            }

            let rgx_start = rgx_start.unwrap();
            let rgx_end = rgx_end.unwrap();

            for i in 0..file.len() {
                let line = file[i];
                if rgx_start.is_match(line) {
                    region_start = Some(i);

                    if (setting.multiple.is_none() || setting.multiple.unwrap() == false) && region_end.is_some() {
                        break;
                    }
                }
                if rgx_end.is_match(line) {
                    region_end = Some(i);

                    if (setting.multiple.is_none() || setting.multiple.unwrap() == false) && region_start.is_some() {
                        break;
                    }
                }
            }

            if region_start.is_none() || region_end.is_none() || region_start.unwrap() >= region_end.unwrap() {
                return None;
            }

            text = file[region_start.unwrap() + 1 .. region_end.unwrap()]
                .join("\n");
        }

    }

    // Now that we've extracted the text, extact the value from it.
    let built_rgx = &setting.substitute;
    let built_rgx = built_rgx.replace("\\", "\\\\");
    let built_rgx = built_rgx.replace("^", "\\^");
    let built_rgx = built_rgx.replace("$", "\\$");
    let built_rgx = built_rgx.replace("|", "\\|");
    let built_rgx = built_rgx.replace("?", "\\?");
    let built_rgx = built_rgx.replace(".", "\\.");
    let built_rgx = built_rgx.replace("*", "\\*");
    let built_rgx = built_rgx.replace("(", "\\(");
    let built_rgx = built_rgx.replace(")", "\\)");
    let built_rgx = built_rgx.replace("{value}", "(.|\n)*"); // Match all characters, even new lines
    let built_rgx = built_rgx.replace("+", "\\+");
    let built_rgx = built_rgx.replace("{", "\\{");
    let built_rgx = built_rgx.replace("[", "\\[");
    let built_rgx = Regex::new(&built_rgx);
    if built_rgx.is_err() {
        eprintln!(
            "Error occured while compiling auto generated regex for setting {}\"{}\"{}: {}{}{}",
            util::color("green", "fg"),
            setting.name,
            util::color("white", "fg"),
            util::color("red", "fg"),
            built_rgx.unwrap_err(),
            util::color("white", "fg"),
        );
        return None;
    }
    let built_rgx = built_rgx.unwrap();

    let start_of_text = built_rgx.find(&text);

    if start_of_text.is_none() {
        eprintln!(
            "Error occurred while extracting the value for setting {}\"{}\"{}: Could not find value",
            util::color("green", "fg"),
            setting.name,
            util::color("white", "fg"),
        );
        return None;
    }

    let start_of_text = start_of_text
        .unwrap()
        .start();

    let start_of_value = setting.substitute.find("{value}");
    if start_of_value.is_none() {
        eprintln!(
            "Error occurred while extracting the value for setting {}\"{}\"{}: Could not find value",
            util::color("green", "fg"),
            setting.name,
            util::color("white", "fg"),
        );
        return None;
    }
    let start_of_value = start_of_value.unwrap();

    let mut text: Vec<&str> = text
        .split("")
        .collect();

    // Remove chars before the value
    for i in 0..start_of_text + start_of_value + 1 {
        text[i] = "";
    }

    let amount_of_chars_after_value = setting.substitute[start_of_value + "{value}".len()..].len() + 1;

    // Remove chars after the value
    for i in 0..amount_of_chars_after_value {
        let end_of_text = text.len() - 1;
        text[end_of_text - i] = "";
    }

    let text = text.join("");

    if print {
        println!("{}", text);
    } else {
        return Some(text);
    }

    if print {
        if let Some(config) = config {
            if let Some(hook_after_get) = &config.hook_after_get {
                run_hook("hook_after_get".to_string(), hook_after_get.to_string());
            }
        }
    }

    return None
}

pub fn load(file: String, config: &Option<Config>, registry: &Vec<Setting>) {
    let settings = std::fs::read_to_string(&file);

    if settings.is_err() {
        eprintln!(
            "Failed to read file {}\"{}\"{}: {}{}{}",
            util::color("green", "fg"),
            file,
            util::color("white", "fg"),
            util::color("red", "fg"),
            settings.unwrap_err(),
            util::color("white", "fg"),
        );
        return;
    }

    let settings = settings.unwrap();
    let settings: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str(&settings);

    if settings.is_err() {
        eprintln!(
            "JSON error in file {}\"{}\"{}: {}{}{}",
            util::color("green", "fg"),
            file,
            util::color("white", "fg"),
            util::color("red", "fg"),
            settings.unwrap_err(),
            util::color("white", "fg"),
        );
        return;
    }

    let settings = settings.unwrap();

    for (name, value) in settings {
        set(name, value, config, registry);
    }
}

pub fn dump(config: &Option<Config>, registry: &Vec<Setting>) {
    let mut settings: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();

    for setting in registry {
        let name = &setting.name;
        let value = get(name.to_string(), false, config, registry);

        if let Some(value) = value {
            settings.insert(name.to_string(), value);
        }
    }
    
    let json = serde_json::to_string_pretty(&settings);

    if let Ok(json) = json {
        println!("{}", json);
    } else {
        eprintln!("Failed to generate JSON");
    }
}
