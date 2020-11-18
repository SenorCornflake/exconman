use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

mod registry;
mod modifier;
mod config;
mod utilities;

use structopt::StructOpt;

use registry as reg;
use modifier as modif;
use config as conf;

const CONFIG_PATH: &str = "~/.config/exconman/config.json";

fn load_registry(path: &str) -> Result<HashMap<String, reg::Setting>, ()> {
    let registry = fs::read_to_string(path);

    if registry.is_err() {
        let error = format!("Error reading registry file <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", path);
        utilities::color(error);

        return Err(());
    }

    let registry = reg::from_json(registry.unwrap().as_str());
    
    if registry.is_err() {
        let error = registry.unwrap_err().to_string();
        let error = error.replace(" `", " `<|R|>");
        let error = error.replace("` ", "<|N|>` ");
        let error = error.replace("`,", "<|N|>`,");
        let error = error.replace(" \"", " \"<|R|>");
        let error = error.replace("\" ", "<|N|>\" ");
        let error = error.replace("\",", "<|N|>\",");
        let error = format!("Registry <|Y|>\"{}\"<|N|> json error: {} <|M|><|N|>", path, error);
        utilities::color(error);

        return Err(());
    }

    let registry = registry.unwrap();

    for (name, setting) in &registry {
        match setting.match_type {
            reg::MatchType::Region => {
                if setting.match_pattern.find("<|SECOND_LINE|>").is_none() {
                    let error = format!("<|Y|>{}<|N|>: <|R|>\"<|SECOND_LINE|>\"<|N|> marker not found in <|R|>\"match_pattern\"<|N|> attribute of setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", path, name);
                    utilities::color(error);

                    return Err(());
                }                   
            }
            _ => {}
        }

        match setting.value_type {
            reg::ValueType::Bool => {
                if setting.replace_with.find("<|FALSE|>").is_none() {
                    let error = format!("<|Y|>{}<|N|>: <|R|>\"<|FALSE|>\"<|N|> marker not found in <|R|>\"replace_with\"<|N|> attribute of setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", path, name);
                    utilities::color(error);

                    return Err(());
                }
            }
            _ => {}
        }

        if !PathBuf::from(utilities::expand_home(setting.file.as_str())).exists() {
            let error = format!("<|Y|>{}<|N|>: File path <|R|>\"{}\"<|N|> does not exist in <|R|>\"file\"<|N|> attribute of setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", path, setting.file, name);
            utilities::color(error);

            return Err(());
        } else if PathBuf::from(utilities::expand_home(setting.file.as_str())).is_dir() {
            let error = format!("<|Y|>{}<|N|>: File path <|R|>\"{}\"<|N|> is a directory in <|R|>\"file\"<|N|> attribute of setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", path, setting.file, name);
            utilities::color(error);

            return Err(());
        }
    }

    return Ok(registry);
}

fn load_modifier(path: &str, registry: &HashMap<String, reg::Setting>) -> Result<HashMap<String, modif::ValueType>, ()> {
    let modifier = fs::read_to_string(path);

    if modifier.is_err() {
        let error = format!("Error reading modifier <|R|>\"{}\"<|N|>", path);
        utilities::color(error);

        return Err(());
    }

    let modifier = modif::from_str(modifier.unwrap().as_str());
    
    if modifier.is_err() {
        let error = modifier.unwrap_err().to_string();
        let error = error.replace(" `", " `<|R|>");
        let error = error.replace("` ", "<|N|>` ");
        let error = error.replace("`,", "<|N|>`,");
        let error = error.replace(" \"", " \"<|R|>");
        let error = error.replace("\" ", "<|N|>\" ");
        let error = error.replace("\",", "<|N|>\",");
        let error = format!("<|Y|>{}<|N|>: {}", path, error);
        utilities::color(error);

        return Err(());
    }

    let modifier = modifier.unwrap();

    if modifier.get("__NAME__").is_none() {
        let error = format!("<|Y|>{}<|N|>: Required setting <|R|>\"__NAME__\"<|N|> not found", path);
        utilities::color(error);

        return Err(());
    }

    for (name, value) in &modifier {
        if registry.get(name).is_none() && name.as_str() != "__NAME__" {
            let error = format!("<|Y|>{}<|N|>: Unregistered setting <|R|>\"{}\"<|N|>", path, name);
            utilities::color(error);

            return Err(());
        } else if name.as_str() != "__NAME__" {
            let registered_value_type = &registry.get(name).unwrap().value_type;
            
            // Check if provided value in the modifier is a boolean or a string and send error
            // based on what the setting was registered as
            match value {
                modif::ValueType::Bool(_) => {
                    if registered_value_type != &reg::ValueType::Bool {
                        let error = format!("<|Y|>{}<|N|>: Setting <|R|>\"{}\"<|N|> has boolean value but requires a string", path, name);
                        utilities::color(error);

                        return Err(());
                    }
                }
                modif::ValueType::String(value) => {
                    if registered_value_type == &reg::ValueType::Bool {
                        let error = format!("<|Y|>{}<|N|>: Setting <|R|>\"{}\"<|N|> has string value but requires a boolean", path, name);
                        utilities::color(error);

                        return Err(());
                    }
                    
                    // If the value provided is supposed to be a file path, check if it exists or is a
                    // directory
                    if registered_value_type == &reg::ValueType::File {
                        if !PathBuf::from(utilities::expand_home(value)).exists() {
                            let error = format!("<|Y|>{}<|N|>: The path <|R|>\"{}\"<|N|> provided for setting <|R|>\"{}\"<|N|> does not exist", path, value, name);
                            utilities::color(error);

                            return Err(());
                        } else if PathBuf::from(utilities::expand_home(value)).is_dir() {
                            let error = format!("<|Y|>{}<|N|>: The path <|R|>\"{}\"<|N|> provided for setting <|R|>\"{}\"<|N|> is a directory", path, value, name);
                            utilities::color(error);

                            return Err(());
                        }
                    }
                }
            }
        }
    }

    return Ok(modifier);
}

fn load_config(path: &str) -> Result<conf::Config, ()> {
    let config = fs::read_to_string(path);

    if config.is_err() {
        let error = format!("Error reading config file <|Y|>\"{}\"<|N|>", path);
        utilities::color(error);

        return Err(());
    }

    let config = conf::from_str(config.unwrap().as_str());
    
    if config.is_err() {
        let error = config.unwrap_err().to_string();
        let error = error.replace(" `", " `<|R|>");
        let error = error.replace("` ", "<|N|>` ");
        let error = error.replace("`,", "<|N|>`,");
        let error = error.replace(" \"", " \"<|R|>");
        let error = error.replace("\" ", "<|N|>\" ");
        let error = error.replace("\",", "<|N|>\",");
        let error = format!("<|Y|>{}<|N|>: {}", path, error);
        utilities::color(error);

        return Err(());
    }

    let config = config.unwrap();

    if config.modifier.len() == 0 {
        let error = format!("No modifiers specified in the configuration file");
        utilities::color(error);

        return Err(());
    }
    
    return Ok(config);
}

pub fn start_replacing(modifier: &HashMap<String, modif::ValueType>, registry: &HashMap<String, reg::Setting>, modifier_path: &str, registry_path: &str) {
    for (modifier_setting_name, modifier_setting_value) in modifier {
        if modifier_setting_name.as_str() != "__NAME__" {
            let registry_setting_attributes = registry.get(modifier_setting_name).unwrap();
            
            // Split the file of the setting into lines
            let file = fs::read_to_string(utilities::expand_home(registry_setting_attributes.file.as_str()));

            if file.is_err() {
                continue;
            }
            
            let file = file.unwrap();
            let mut file: Vec<String> = file
                .split("\n")
                .map(|s| s.to_string())
                .collect();
            
            // Get the "replace_with" value ready
            let mut replace_with = String::new();

            match registry_setting_attributes.value_type {
                reg::ValueType::Bool => {
                    if let modif::ValueType::Bool(modifier_setting_value) = modifier_setting_value {
                        if *modifier_setting_value == true {
                            replace_with = registry_setting_attributes
                                .replace_with
                                .split("<|FALSE|>")
                                .collect::<Vec<&str>>()[0]
                                .to_string();
                        } else {
                            replace_with = registry_setting_attributes
                                .replace_with
                                .split("<|FALSE|>")
                                .collect::<Vec<&str>>()[1]
                                .to_string();
                        }
                    }
                }
                reg::ValueType::File => {
                    if let modif::ValueType::String(modifier_setting_value) = modifier_setting_value {
                        let contents = fs::read_to_string(utilities::expand_home(modifier_setting_value.as_str()));
                        
                        if contents.is_err() {
                            continue;
                        }

                        replace_with = registry_setting_attributes
                            .replace_with
                            .replace("<|VALUE|>", contents.unwrap().as_str());
                    }
                }
                reg::ValueType::Text => {
                    if let modif::ValueType::String(modifier_setting_value) = modifier_setting_value {
                        replace_with = registry_setting_attributes.replace_with.replace("<|VALUE|>", modifier_setting_value.as_str());
                    }
                }
            }

            match registry_setting_attributes.match_type {
                reg::MatchType::Region => {
                    let match_pattern = registry_setting_attributes
                        .match_pattern
                        .split("<|SECOND_LINE|>")
                        .collect::<Vec<&str>>();

                    let first_line_regex: Result<regex::Regex, regex::Error> = regex::Regex::new(match_pattern[0]);
                    let second_line_regex: Result<regex::Regex, regex::Error> = regex::Regex::new(match_pattern[1]);
                    let mut first_line_match_index: Option<usize> = None;
                    let mut second_line_match_index: Option<usize> = None;

                    if first_line_regex.is_err() || second_line_regex.is_err() {
                        let error = format!("The match pattern of setting <|R|>\"{}\"<|N|> in modifier <|R|>\"{}\"<|N|> contains invalid regex", registry_path, modifier_setting_name);
                        utilities::color(error);

                        continue;
                    }

                    let first_line_regex = first_line_regex.unwrap();
                    let second_line_regex = second_line_regex.unwrap();

                    for (i, line) in file.iter().enumerate() {
                        if first_line_regex.is_match(line) {
                            first_line_match_index = Some(i);
                        }
                    }
                    for (i, line) in file.iter().enumerate() {
                        if second_line_regex.is_match(line) {
                            second_line_match_index = Some(i);
                            break;
                        }
                    }

                    if first_line_match_index.is_none() || second_line_match_index.is_none() {
                        let error = format!("Match not found for setting <|R|>\"{}\"<|N|>", modifier_setting_name);
                        utilities::color(error);

                        continue;
                    }

                    let first_line_match_index = first_line_match_index.unwrap();
                    let second_line_match_index = second_line_match_index.unwrap();

                    if second_line_match_index <= first_line_match_index {
                        let error = format!("The second line marking a region was found before the first, consider changing the regex pattern of the setting <|R|>\"{}\"<|N|>", modifier_setting_name);
                        utilities::color(error);

                        continue;
                    }

                    file.drain(first_line_match_index + 1..second_line_match_index);
                    file.insert(first_line_match_index + 1, replace_with);
                }
                reg::MatchType::Line => {
                    let line_regex: Result<regex::Regex, regex::Error> = regex::Regex::new(registry_setting_attributes.match_pattern.as_str());

                    if line_regex.is_err() {
                        let error = format!("<|Y|>{}<|N|>: Invalid regex in <|R|>\"match_pattern\"<|N|> attribute for setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", registry_path, modifier_setting_name);
                        utilities::color(error);

                        continue;
                    }

                    let line_regex = line_regex.unwrap();
                    let mut matches: Vec<usize> = Vec::new();

                    for (i, line) in file.iter_mut().enumerate() {
                        if line_regex.is_match(line) {
                            matches.push(i);
                        }
                    }

                    for i in &matches {
                        let i = *i;

                        match registry_setting_attributes.replace_line {
                            reg::ReplaceLine::Matched => {
                                let replace_line = line_regex.replace_all(file[i].as_str(), replace_with.as_str())
                                    .into_owned();
                                file[i] = replace_line;
                            }
                            reg::ReplaceLine::Above => {
                                if i == 0 {
                                    let error = format!("<|Y|>{}<|N|>: No line to replace above the first line. Consider changing the <|R|>\"replace_line\"<|N|> attribute for setting <|R|>\"{}\"<|N|> <|M|>-- REGISTRY ERROR<|N|>", registry_path, modifier_setting_name);
                                    utilities::color(error);

                                    continue;
                                }
                                let replace_line = line_regex.replace_all(file[i - 1].as_str(), replace_with.as_str())
                                    .into_owned();
                                file[i - 1] = replace_line;
                            }
                            reg::ReplaceLine::Below => {
                                let replace_line = line_regex.replace_all(file[i + 1].as_str(), replace_with.as_str())
                                    .into_owned();
                                file[i + 1] = replace_line;
                            }
                        }
                    }

                    if matches.len() == 0 {
                        let error = format!("<|Y|>{}<|N|>: Match not found for setting <|R|>\"{}\"<|N|>", modifier_path, modifier_setting_name);
                        utilities::color(error);
                    }

                }
            }

            match fs::write(utilities::expand_home(registry_setting_attributes.file.as_str()), file.join("\n")) {
                _ => {}
            };
        }
    }

    return;
}

fn main () {
    #[derive(StructOpt, Debug)]
    enum Args {
        /// Start replacing settings
        Start,
        /// List all settings in the registry
        List,
        /// Query a value
        Query(Query),
        /// Edit a value
        Edit(Edit)
    }

    #[derive(StructOpt, Debug)]
    enum Query {
        Modifier {
            path: String,
            setting: String
        },
        Config {
            setting: String
        }
    }

    #[derive(StructOpt, Debug)]
    enum Edit {
        Modifier {
            path: String,
            setting: String,
            value: String
        },
        Config {
            setting: String,
            value: String
        }
    }

    let config = load_config(utilities::expand_home(CONFIG_PATH).as_str());
    if config.is_err() {
        return;
    }
    let mut config = config.unwrap();

    let registry = load_registry(utilities::expand_home(config.registry.as_str()).as_str());
    if registry.is_err() {
        return;
    }
    let registry = registry.unwrap();

    let args: Args = Args::from_args();

    match args {
        Args::Start => {
            let modifier = load_modifier(utilities::expand_home(config.modifier[0].as_str()).as_str(), &registry);
            if modifier.is_err() {
                return;
            }
            let mut modifier = modifier.unwrap();

            let mut index = 1;
            while index < config.modifier.len() {
                let merge_modifier = load_modifier(utilities::expand_home(config.modifier[index].as_str()).as_str(), &registry);
                if merge_modifier.is_err() {
                    return;
                }
                let merge_modifier = merge_modifier.unwrap();

                for (key, value) in merge_modifier {
                    if key != "__NAME__" {
                        modifier.insert(key, value);
                    }
                }

                index += 1;
            }

            start_replacing(&modifier, &registry, config.modifier[0].as_str(), config.registry.as_str());
        }
        Args::Query(query) => {
            match query {
                Query::Modifier { path, setting } => {
                    let modifier = load_modifier(utilities::expand_home(path.as_str()).as_str(), &registry);
                    if modifier.is_err() {
                        return;
                    }
                    let modifier = modifier.unwrap();

                    let value = modifier.get(setting.as_str());

                    if value.is_none() {
                        let error = format!("Modifier <|R|>\"{}\"<|N|> does not contain setting <|R|>\"{}\"<|N|>", path, setting);
                        utilities::color(error);

                        return;
                    }

                    match value.unwrap() {
                        modif::ValueType::Bool(value) => {
                            println!("{}", value);
                        }
                        modif::ValueType::String(value) => {
                            println!("{}", value);
                        }
                    }
                }
                Query::Config { setting } => {
                    match setting.as_str() {
                        "registry" => {
                            println!("{}", config.registry);
                        }
                        "modifier" => {
                            println!("{:?}", config.modifier);
                        }
                        _ => {
                            let error = format!("Config does not contain setting <|R|>\"{}\"<|N|>", setting);
                            utilities::color(error);
                        }
                    }
                }
            }
        }
        Args::Edit(edit) => {
            match edit {
                Edit::Modifier { path, setting, value } => {
                    let modifier = load_modifier(utilities::expand_home(path.as_str()).as_str(), &registry);
                    if modifier.is_err() {
                        return;
                    }
                    let mut modifier = modifier.unwrap();

                    if registry.get(&setting).is_none() {
                        let error = format!("<|Y|>{}<|N|>: Unregistered setting <|R|>\"{}\"<|N|> <|M|>-- CMD ERROR<|N|>", path, setting);
                        utilities::color(error);

                        return;
                    }

                    match registry.get(&setting).unwrap().value_type {
                        reg::ValueType::Bool => {
                            if value == "true" {
                                modifier.insert(setting, modif::ValueType::Bool(true));
                            } else if value == "false" {
                                modifier.insert(setting, modif::ValueType::Bool(false));
                            } else {
                                let error = format!("<|Y|>{}<|N|>: Setting <|R|>\"{}\"<|N|> requires boolean value <|M|>-- CMD ERROR<|N|>", path, setting);
                                utilities::color(error);

                                return;
                            }
                        }
                        _ => {
                            modifier.insert(setting, modif::ValueType::String(value));
                        }
                    }
                    
                    let modifier = serde_json::to_string(&modifier).unwrap();

                    match fs::write(path, modifier) {
                        _ => {}
                    }
                }
                Edit::Config { setting, value } => {
                    match setting.as_str() {
                        "registry" => {
                            config.registry = value;
                        }
                        "modifier" => {
                            let modifier: Result<Vec<String>, serde_json::Error> = serde_json::from_str(value.as_str());

                            if modifier.is_err() || (modifier.is_ok() && modifier.as_ref().unwrap().len() == 0) {
                                let error = format!("An array containing at least one string is required <|M|>-- CMD ERROR<|N|>");
                                utilities::color(error);

                                return;
                            }

                            config.modifier = modifier.unwrap();
                        }
                        _ => {
                            let error = format!("Config does not contain setting <|R|>\"{}\"<|N|>", setting);
                            utilities::color(error);
                        }
                    }
                }
            }   
        }
        Args::List => {
            for (key, value) in registry {
                let value_type = match value.value_type {
                    reg::ValueType::File => "file",
                    reg::ValueType::Bool => "bool",
                    reg::ValueType::Text => "text"
                };
                let setting_signature = format!("<|G|>{}<|N|>: <|Y|>{}<|N|>", key, value_type);
                utilities::color(setting_signature);
            }
        }
    }
}
