use std::path::PathBuf;

use serde_derive::{Serialize, Deserialize};
use serde_json;
use structopt::StructOpt;
use regex::Regex;

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
    registry: String,
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
    let result = std::fs::read_to_string(&setting.file);
    
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

    let file = std::fs::read_to_string(&setting.file).unwrap();
    let file: Vec<&str> = file.split("\n")
        .collect();
    let file: Vec<String> = file.iter().map(|s| s.to_string()).collect();
    let mut modified_file = file.clone();

    let mut replace_value = String::new();

    if setting.value_is_file {
        let result = std::fs::read_to_string(&value);

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
    match std::fs::write(&setting.file, modified_file) { _ => {}}
}

fn get(setting_name: String, stop_at_first_match: bool, registry: &Vec<Setting>) {
    let setting = get_setting(setting_name, registry);
    if setting.is_none() { return }
    let setting = setting.unwrap();

    let file = std::fs::read_to_string(&setting.file).unwrap();
    let file: Vec<&str> = file.split("\n")
        .collect();
    let file: Vec<String> = file.iter().map(|s| s.to_string()).collect();

    let mut text: Vec<&str> = Vec::new();
    let mut temp: String = String::new();

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

            temp = file[region_start.unwrap() + 1 .. region_end.unwrap()].join("\n");
            text = temp.split("").collect();
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
                                text = file[i - 1]
                                    .split("")
                                    .collect();
                            }
                        }
                        ReplaceType::Below => {
                            if i != file.len() {
                                text = file[i + 1]
                                    .split("")
                                    .collect();
                            }
                        }
                        ReplaceType::Matched => {
                            text = file[i]
                                .split("")
                                .collect();
                        }
                    }
                    if stop_at_first_match { break }
                }
            }

        }
    }

    if text.len() == 0 { return }

    // Here we remove the non-value text. For example, if the replace_value was "x = {value};",
    // then "x = " and ";" will be removed because it's not part of the provided value
    let value_index = setting.replace_value.find("{value}");
    if value_index.is_none() { return }

    let value_prefix_len = setting.replace_value[0 .. value_index.unwrap()].len();
    let value_suffix_len = setting.replace_value[value_index.unwrap() + "{value}".len() .. setting.replace_value.len()].len();

    for i in 0 .. value_prefix_len + 1 {
        if i < text.len() {
            text[i] = "";
        }
    }

    for i in text.len() - value_suffix_len - 1 .. text.len() {
        if i < text.len() {
            text[i] = "";
        }
    }

    println!("{}", text.join(""));
}

fn main() {
    let args = Args::from_args();
    let subcommand = args.sub;
    let registry = args.registry;
    let stop_at_first_match = args.stop_at_first_match;
    
    let registry = std::fs::read_to_string(registry);

    if registry.is_err() {
        eprintln!("{}", registry.unwrap_err());
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
    }
}
