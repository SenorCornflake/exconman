use serde_derive::{Serialize, Deserialize};
use serde_json;
use structopt::StructOpt;
use regex::Regex;

use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
enum Replace {
    #[serde(rename = "above")]
    Above,
    #[serde(rename = "below")]
    Below,
    #[serde(rename = "matched")]
    Matched
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Pattern {
    Region([String; 2]),
    Line(String)
}

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    name: String,
    file: String,
    pattern: Pattern,
    substitute: String,
    replace: Replace,
    value_is_file: bool
}

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long, short)]
    /// Supply a registry to use
    registry: Option<String>,
    #[structopt(long, short)]
    /// Stop searching once a match is found
    stop_at_first_match: bool,
    #[structopt(subcommand)]
    sub: Subcommand
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    /// Set the value of a setting
    Set(Set),
    /// Get the value of a setting
    Get(Get),
    /// Load settings from a JSON file
    Load(Load),
    /// Print out all setting values in JSON format
    Dump
}

#[derive(StructOpt, Debug)]
struct Set {
    setting_name: String,
    value: String
}

#[derive(StructOpt, Debug)]
struct Get {
    setting_name: String
}

#[derive(StructOpt, Debug)]
struct Load {
    file: String
}

fn expand_home(path: &str) -> String {
    return path.replace("~", std::env::var("HOME").unwrap().as_str());

}

fn get_setting(setting_name: String, registry: &Vec<Setting>) -> Option<&Setting> {
    for setting in registry {
        if setting.name == setting_name {
            return Some(setting);
        }
    }
    eprintln!("Setting \"{}\" does not exist.", setting_name);
    return None;
}

fn validate_setting(setting: &Setting, registry_path: &str) -> Result<(), ()> {
    let result = std::fs::metadata(expand_home(&setting.file));
    
    if result.is_err() {
        eprintln!("Error in setting \"{}\" in registry \"{}\": \"{}\" | {}", setting.name, registry_path, setting.file, result.unwrap_err());
        return Err(());
    }
    return Ok(());
}

fn set(setting_name: String, value: String, stop_at_first_match: bool, registry: &Vec<Setting>) {
    let setting = get_setting(setting_name, registry);
    if setting.is_none() { return }
    let setting = setting.unwrap();

    let file = std::fs::read_to_string(&expand_home(&setting.file)).unwrap();
    let file: Vec<&str> = file.split("\n")
        .collect();
    let file: Vec<String> = file.iter().map(|s| s.to_string()).collect();
    let mut modified_file = file.clone();

    #[allow(unused_mut)]
    let mut substitute;

    if setting.value_is_file {
        let result = std::fs::read_to_string(expand_home(&value));

        if result.is_err() {
            eprintln!("\"{}\" | {}", value, result.unwrap_err());
            return;
        }

        let result = result.unwrap();
        substitute = setting.substitute.replace("{value}", result.as_str());
    } else {
        substitute = setting.substitute.replace("{value}", value.as_str());
    }

    let substitute = substitute.as_str();

    match &setting.pattern {
        Pattern::Region(region) => {
            let mut region_start = None;
            let mut region_end = None;

            let regex1 = Regex::new(region[0].as_str());
            if regex1.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return;
            }
            let regex1 = regex1.unwrap();

            let regex2 = Regex::new(region[1].as_str());
            if regex2.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return;
            }
            let regex2 = regex2.unwrap();

            for (i, line) in file.iter().enumerate() {
                if regex1.is_match(line) {
                    region_start = Some(i);
                    if stop_at_first_match { break }
                }
                if regex2.is_match(line) {
                    region_end = Some(i);
                    if stop_at_first_match { break }
                }
            }

            if region_start.is_none() || region_end.is_none() || region_end.unwrap() <= region_start.unwrap() { return }

            modified_file.drain(region_start.unwrap() + 1..region_end.unwrap());
            modified_file.insert(region_start.unwrap() + 1, substitute.to_string());
        },
        Pattern::Line(pattern) => {
            let regex = Regex::new(pattern.as_str());
            if regex.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return;
            }
            let regex = regex.unwrap();


            for (i, line) in file.iter().enumerate() {
                if regex.is_match(line) {
                    match &setting.replace {
                        Replace::Above => {
                            if i != 0 {
                                modified_file[i - 1] = substitute.to_string();
                            }
                        }
                        Replace::Below => {
                            if i != file.len() {
                                modified_file[i + 1] = substitute.to_string();
                            }
                        }
                        Replace::Matched => {
                            modified_file[i] = regex.replace_all(line, substitute).to_string();
                        }
                    }
                    if stop_at_first_match { break }
                }
            }

        }
    }

    let modified_file = modified_file.join("\n");
    match std::fs::write(expand_home(&setting.file), modified_file) { _ => {}}
}

fn get(setting_name: String, stop_at_first_match: bool, print: bool, registry: &Vec<Setting>) -> Result<String, ()>{
    let setting = get_setting(setting_name, registry);
    if setting.is_none() { return Err(()) }
    let setting = setting.unwrap();

    let file = std::fs::read_to_string(expand_home(&setting.file)).unwrap();
    let file: Vec<&str> = file.split("\n")
        .collect();
    //let file: Vec<String> = file.iter().map(|s| s.to_string()).collect();

    let mut text: String = String::new();

    match &setting.pattern {
        Pattern::Region(region) => {
            let mut region_start = None;
            let mut region_end = None;

            let regex1 = Regex::new(region[0].as_str());
            if regex1.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return Err(());
            }
            let regex1 = regex1.unwrap();

            let regex2 = Regex::new(region[1].as_str());
            if regex2.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return Err(());
            }
            let regex2 = regex2.unwrap();

            for (i, line) in file.iter().enumerate() {
                if regex1.is_match(line) {
                    region_start = Some(i);
                    if stop_at_first_match { break }
                }
                if regex2.is_match(line) {
                    region_end = Some(i);
                    if stop_at_first_match { break }
                }
            }

            if region_start.is_none() || region_end.is_none() || region_end.unwrap() <= region_start.unwrap() { return Err(()) }

            text = file[region_start.unwrap() + 1 .. region_end.unwrap()].join("\n");
        },
        Pattern::Line(pattern) => {
            let regex = Regex::new(pattern.as_str());
            if regex.is_err() {
                eprintln!("Error compiling regex pattern for setting \"{}\"", setting.name);
                return Err(());
            }
            let regex = regex.unwrap();

            for (i, line) in file.iter().enumerate() {
                if regex.is_match(line) {
                    match &setting.replace {
                        Replace::Above => {
                            if i != 0 {
                                text = file[i - 1].to_string();
                            }
                        }
                        Replace::Below => {
                            if i != file.len() {
                                text = file[i + 1].to_string();
                            }
                        }
                        Replace::Matched => {
                            text = file[i].to_string();
                        }
                    }
                    if stop_at_first_match { break }
                }
            }

        }
    }

    if text.len() == 0 { return Err(()) }

    // Escape special characters
    let regex = &setting.substitute;
    let regex = regex.replace("\\", "\\\\");
    let regex = regex.replace("^", "\\^");
    let regex = regex.replace("$", "\\$");
    let regex = regex.replace("|", "\\|");
    let regex = regex.replace("?", "\\?");
    let regex = regex.replace(".", "\\.");
    let regex = regex.replace("*", "\\*");
    let regex = regex.replace("(", "\\(");
    let regex = regex.replace(")", "\\)");
    let regex = regex.replace("{value}", "(.|\n)*"); // Match all characters, even new lines
    let regex = regex.replace("+", "\\+");
    let regex = regex.replace("{", "\\{");
    let regex = regex.replace("[", "\\[");
    let regex = Regex::new(&regex);
    if regex.is_err() {
        eprintln!("Error compiling hardcoded (not your fault) regex for setting \"{}\"", setting.name);
        return Err(());
    }
    let regex = regex.unwrap();

    let starting_index = regex.find(text.as_str());

    if starting_index.is_none() {
        eprintln!("An error ocurred while trying to get the value for \"{}\"", setting.name);
        return Err(());
    }
    let starting_index = starting_index.unwrap().start();
    
    let value_index = setting.substitute.find("{value}");
    if value_index.is_none() { return Err(()) }
    let value_index = value_index.unwrap();

    let value_suffix_len = setting.substitute[value_index + "{value}".len()..].len();
    
    let mut text: Vec<&str> = text
        .split("")
        .collect();
    
    // The starting index marks the beginning of the text
    // provided in the substitute, for example, the substitute might be "border_width = {value}",
    // but in the file there might be a tab or spaces before it, therefore being "\tborder_width = {value}".
    // Replacing
    // The starting index is found by looking for what was given in the substitute. That's what
    // the regex being constructed above is for.
    //
    // - Past me

    // Remove characters before value
    for i in 0 .. starting_index + value_index + 1 {
        text[i] = "";
    }
    
    // Remove characters after the value (done using reverse loop)
    let mut count = 0;
    for i in (0 .. text.len()).rev() {
        if count <= value_suffix_len {
            text[i] = "";
        }
        count += 1;
    }

    if print {
        println!("{}", text.join(""));
    }

    return Ok(text.join(""))
}

fn load(file_name: String, stop_at_first_match: bool, registry: &Vec<Setting>) {
    let settings = std::fs::read_to_string(&file_name);

    if settings.is_err() {
        eprintln!("\"{}\" | {}", file_name, settings.unwrap_err());
        return;
    }
    
    let settings: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str(&settings.unwrap());

    if settings.is_err() {
        eprintln!("\"{}\" | {}", file_name, settings.unwrap_err());
        return;
    }

    let settings = settings.unwrap();

    for (setting, value) in settings.iter() {
        let setting = setting.as_str();
        let value = value.as_str();

        set(setting.to_string(), value.to_string(), stop_at_first_match, registry)
    }
}

fn get_registry(registry_path: &str) -> Result<Vec<Setting>, ()>{
    match std::fs::metadata(registry_path) {
        Err(_) => {
            return Err(());
        }
        Ok(metadata) => {
            if metadata.is_file() {
                let registry = std::fs::read_to_string(&registry_path);

                if registry.is_err() {
                    eprintln!("Failed to read registry \"{}\"", registry_path);
                }
                let registry = registry.unwrap();

                let registry: Result<Vec<Setting>, serde_json::Error> = serde_json::from_str(registry.as_str());

                if registry.is_err() {
                    eprintln!("JSON Error in registry \"{}\": {}", registry_path, registry.unwrap_err());
                    return Err(());
                }

                let mut error_occured = false;

                let registry = registry.unwrap();

                for setting in &registry {
                    let result = validate_setting(&setting, &registry_path);
                    if result.is_err() {
                        error_occured = true;
                   }
                }

                if error_occured { return Err(()) }
                
                return Ok(registry);
            } else if metadata.is_dir() {
                // Join all registries in the registry directory
                let files: Vec<Result<std::fs::DirEntry, _>> = std::fs::read_dir(registry_path).unwrap().collect();
                let mut joined_registry: Vec<Setting> = Vec::new();

                for file in files {
                    let registry_path = file.unwrap().path();
                    let registry_path = registry_path.display();
                    let registry_path = registry_path.to_string();

                    let registry = std::fs::read_to_string(&registry_path);

                    if registry.is_err() {
                        eprintln!("Failed to read registry \"{}\"", registry_path);
                    }
                    let registry = registry.unwrap();

                    let registry: Result<Vec<Setting>, serde_json::Error> = serde_json::from_str(registry.as_str());

                    if registry.is_err() {
                        eprintln!("JSON Error in registry \"{}\": {}", registry_path, registry.unwrap_err());
                        return Err(());
                    }

                    let mut error_occured = false;

                    let mut registry = registry.unwrap();

                    for setting in &registry {
                        let result = validate_setting(&setting, &registry_path);
                        if result.is_err() {
                            error_occured = true;
                       }
                    }

                    if error_occured { continue; }

                    joined_registry.append(&mut registry);
                }

                return Ok(joined_registry);
            }
        }
    }

    return Err(());
}

fn main() {
    let args = Args::from_args();

    let subcommand = args.sub;
    let registry_path = args.registry;
    let stop_at_first_match = args.stop_at_first_match;
    
    let registry_path: Result<String, ()> = if registry_path.is_some() {
        Ok(registry_path.unwrap())
    } else {
        // The following code chooses whichever registry paths exists as the registry path
        let registry_file = "~/.config/exconman/registry.json";
        let registry_file: String = expand_home(registry_file);

        let registry_dir = "~/.config/exconman/registry";
        let registry_dir: String = expand_home(registry_dir);
        
        let file_metadata = std::fs::metadata(&registry_file);
        let dir_metadata = std::fs::metadata(&registry_dir);

        if file_metadata.is_err() && dir_metadata.is_err() {
            eprintln!("Both default registry paths \"{}\" and \"{}\" do not exist, create them or provide a location to a registry using --registry", registry_file, registry_dir);
            Err(())
        } else {
            if file_metadata.is_ok() {
                Ok(registry_file)
            } else {
                Ok(registry_dir)
            }
        }
    };

    if registry_path.is_err() {
        return;
    }

    let registry_path = registry_path.unwrap();

    let registry = get_registry(&registry_path);

    if registry.is_err() { return; }

    let registry = registry.unwrap();

    match subcommand {
        Subcommand::Get(Get { setting_name }) => {
            match get(setting_name, stop_at_first_match, true, &registry) {
                _ => {}
            };
        }
        Subcommand::Set(Set { setting_name, value }) => {
            set(setting_name, value, stop_at_first_match, &registry);
        }
        Subcommand::Load(Load { file }) => {
            load(file, stop_at_first_match, &registry);
        }
        Subcommand::Dump => {
            let mut settings: BTreeMap<String, String> = BTreeMap::new();

            for setting in &registry {
                let setting_name = &setting.name.as_str().to_string();
                let setting_value = get(setting_name.to_string(), stop_at_first_match, false, &registry);

                match setting_value {
                    Ok(value) => {
                        settings.insert(setting_name.to_string(), value);
                    }
                    Err(_) => {}
                }
            }

            let json = serde_json::to_string_pretty(&settings);

            match json {
                Ok(value) => {
                    println!("{}", value);
                },
                Err(_) => {
                    eprintln!("Failed to generate JSON");
                }
            }
        }
    }
}
