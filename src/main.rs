use serde_derive::{Serialize, Deserialize};
use serde_json;
use structopt::StructOpt;
use regex::Regex;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
enum ReplaceType {
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
    replace_value: String,
    replace_type: ReplaceType,
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
    Set(Set),
    Get(Get),
    Load(Load)
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
    eprintln!("A setting named \"{}\" does not exist", setting_name);
    return None;
}

fn validate_setting(setting: &Setting) -> Result<(), ()> {
    let result = std::fs::metadata(expand_home(&setting.file));
    
    if result.is_err() {
        eprintln!("\"{}\" | {}",setting.file, result.unwrap_err());
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
    let mut replace_value;

    if setting.value_is_file {
        let result = std::fs::read_to_string(expand_home(&value));

        if result.is_err() {
            eprintln!("\"{}\" | {}", value, result.unwrap_err());
            return;
        }

        let result = result.unwrap();
        replace_value = setting.replace_value.replace("{value}", result.as_str());
    } else {
        replace_value = setting.replace_value.replace("{value}", value.as_str());
    }

    let replace_value = replace_value.as_str();

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
            modified_file.insert(region_start.unwrap() + 1, replace_value.to_string());
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
                    match &setting.replace_type {
                        ReplaceType::Above => {
                            if i != 0 {
                                modified_file[i - 1] = replace_value.to_string();
                            }
                        }
                        ReplaceType::Below => {
                            if i != file.len() {
                                modified_file[i + 1] = replace_value.to_string();
                            }
                        }
                        ReplaceType::Matched => {
                            modified_file[i] = regex.replace_all(line, replace_value).to_string();
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

fn get(setting_name: String, stop_at_first_match: bool, registry: &Vec<Setting>) {
    let setting = get_setting(setting_name, registry);
    if setting.is_none() { return }
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

            text = file[region_start.unwrap() + 1 .. region_end.unwrap()].join("\n");
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
                    match &setting.replace_type {
                        ReplaceType::Above => {
                            if i != 0 {
                                text = file[i - 1].to_string();
                            }
                        }
                        ReplaceType::Below => {
                            if i != file.len() {
                                text = file[i + 1].to_string();
                            }
                        }
                        ReplaceType::Matched => {
                            text = file[i].to_string();
                        }
                    }
                    if stop_at_first_match { break }
                }
            }

        }
    }

    if text.len() == 0 { return }

    // Escape special characters
    let regex = &setting.replace_value;
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
        return;
    }
    let regex = regex.unwrap();

    let starting_index = regex.find(text.as_str());

    if starting_index.is_none() {
        eprintln!("An error ocurred while trying to get the value for \"{}\"", setting.name);
        return
    }
    let starting_index = starting_index.unwrap().start();
    
    let value_index = setting.replace_value.find("{value}");
    if value_index.is_none() { return }
    let value_index = value_index.unwrap();

    let value_suffix_len = setting.replace_value[value_index + "{value}".len()..].len();
    
    let mut text: Vec<&str> = text
        .split("")
        .collect();
    
    // The starting index marks the beginning of the text
    // provided in the replace_value, for example, the replace_value might be "border_width = {value}",
    // but in the file there might be a tab or spaces before it, therefore being "\tborder_width = {value}".
    // The starting index is found by looking for what was given in the replace_value. That's what
    // the regex being constructed above is for. Hope it's not too daunting!
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

    println!("{}", text.join(""));
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

fn main() {
    let args = Args::from_args();

    let subcommand = args.sub;
    let registry_path = args.registry;
    let stop_at_first_match = args.stop_at_first_match;

    let registry_path: String = if registry_path.is_some() {
        registry_path.unwrap()
    } else {
        String::from("~/.config/exconman/registry.json")
    };
    
    let registry = std::fs::read_to_string(&expand_home(&registry_path));

    if registry.is_err() {
        eprintln!("{} {}", registry_path, registry.unwrap_err());
        return;
    }
    
    let registry: Result<Vec<Setting>, serde_json::Error> = serde_json::from_str(registry.unwrap().as_str());

    if registry.is_err() {
        eprintln!("Registry JSON Error: {}", registry.unwrap_err());
        return;
    }

    let mut error_occured = false;

    let registry = registry.unwrap();

    for setting in &registry {
        let result = validate_setting(&setting);
        if result.is_err() {
            error_occured = true;
       }
    }

    if error_occured { return }


    match subcommand {
        Subcommand::Get(Get { setting_name }) => {
            get(setting_name, stop_at_first_match, &registry);
        }
        Subcommand::Set(Set { setting_name, value }) => {
            set(setting_name, value, stop_at_first_match, &registry);
        }
        Subcommand::Load(Load { file }) => {
            load(file, stop_at_first_match, &registry);
        }
    }

}
