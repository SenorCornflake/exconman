use std::collections::HashMap;
use std::fs;

fn parse(contents: &str) -> Result<HashMap<String, String>, Vec<u32>> {

    let contents: Vec<&str> = contents
        .split("\n")
        .collect();

    let mut parsed_contents: HashMap<String, String> = HashMap::new();

    let mut errors: Vec<u32> = Vec::new();

    for (line_number, line) in contents.iter().enumerate() {
        if line.trim_start().trim_end() == "" {
            continue;
        }

        if line.find("=").is_none() {
            errors.push(line_number as u32);
            continue;
        }
        
        let line: Vec<&str> = line
            .splitn(2, "=")
            .collect();

        let mut line = vec![
            line[0].to_string(),
            line[1].to_string()
        ];
        line[0] = line[0]
            .trim_start()
            .trim_end()
            .to_string();
        line[1] = line[1]
            .trim_start()
            .trim_end()
            .to_string();

        if line[1].len() > 0
           && line[1].chars().nth(0).unwrap() == '"'
           && line[1].chars().last().unwrap() == '"' {
            line[1].remove(0);
            let line1_len = line[1].len();
            line[1].remove(line1_len - 1);
        }
        
        parsed_contents.insert(String::from(&line[0]), String::from(&line[1]));
    }

    if errors.len() > 0 {
        return Err(errors);
    } else {
        return Ok(parsed_contents);
    }
}

pub fn from_hash(hash: HashMap<String, String>) -> String {
    let mut string = String::new();

    for (key, value) in hash {
        string += format!("{} = {}\n", key, value).as_str();
    }

    return string;
}

pub fn merge(mut settings: HashMap<String, String>, merge_with: HashMap<String, String>) -> HashMap<String, String>{
    for (key, value) in merge_with {
        settings.insert(key, value);
    }

    return settings;
}

pub fn parse_file(file: &str) -> Result<HashMap<String, String>, ()> {
    let file_contents = fs::read_to_string(file);

    if file_contents.is_err() {
        eprintln!("Error reading {}", file);
        return Err(());
    }

    let file_contents = file_contents.unwrap();
    let parsed_contents = parse(file_contents.as_str());

    match parsed_contents {
        Err(errors) => {
            for line_number in errors {
                eprintln!("Missing assignment symbol (=) on line {} in file \"{}\"", line_number, file);
            }
        }
        Ok(parsed) => {
            return Ok(parsed);
        }
    };

    return Err(());
}

pub fn parse_registry(file: &str) {
    let file_contents = fs::read_to_string(file);

    if file_contents.is_err() {
        eprintln!("Error reading {}", file);
        return;
    }
    
    let file_contents = file_contents.unwrap();

    let file_contents: Vec<&str> = file_contents
        .split("\n")
        .collect();

    let mut section_indicators: Vec<usize> = Vec::new();

    for (i, line) in file_contents.iter().enumerate() {
        let line = line
            .trim_end()
            .trim_start();

        if line == "--" {
            section_indicators.push(i);
        }
    }

    let mut sections: Vec<String> = Vec::new();
    
    for (i, indicator) in section_indicators.iter().enumerate() {
        if i != section_indicators.len() - 1 {
            sections.push(file_contents[*indicator + 1..section_indicators[i + 1]].to_vec().join("\n"));
        }
    }
    
    let mut parsed_registry: HashMap<String, HashMap<String, String>> = HashMap::new();

    for (i, s) in sections.iter().enumerate() {
        let s = parse(s);

        match s {
            Err(errors) => {
                for line_number in errors {
                    println!("Missing assignment symbol (=) in line {}", (line_number + section_indicators[i] as u32) + 2);
                }
            }
            Ok(setting) => {
                let result = evaluate_setting(setting, section_indicators[i] + 1);

                if result.is_ok() {
                    println!("{:#?}", result.unwrap());
                }
            }
        }
    }
}

// Check if a setting's attributes are correct
fn evaluate_setting(setting: HashMap<String, String>, setting_location: usize) -> Result<HashMap<String, String>, ()> {
    let required_attributes = vec![
        "name",
        "file",
        "value_type",
        "replace_at",
        "replace_with",
        "match_pattern",
        "match_type"
    ];

    let mut error = false;

    for attr in required_attributes {
        if setting.get(attr).is_none() {
            println!("Attribute \"\x1b[0;31m{}\x1b[0m\" not found for setting located at line \x1b[0;31m{}\x1b[0m in the registry", attr, setting_location);
            error = true;
        }
    }

    if error {
        return Err(());
    }

    match setting.get("value_type").unwrap().as_str() {
        "boolean" => {
            let replace_with = setting.get("replace_with").unwrap();

            if replace_with.find("<|true_begin|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|true_begin|>\x1b[0m\" marker in \"\x1b[0;31mreplace_with\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if replace_with.find("<|true_end|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|true_end|>\x1b[0m\" marker in \"\x1b[0;31mreplace_with\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if replace_with.find("<|false_begin|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|false_begin|>\x1b[0m\" marker in \"\x1b[0;31mreplace_with\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if replace_with.find("<|false_end|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|false_end|>\x1b[0m\" marker in \"\x1b[0;31mreplace_with\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
        }
        "text" => {}
        _ => {
            eprintln!("Invalid value for \"\x1b[0;31mvalue_type\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
            error = true;
        }
    }

    let replace_at = setting.get("replace_at").unwrap().parse::<i32>();

    if replace_at.is_err() {
        eprintln!("Invalid value for \"\x1b[0;31mreplace_at\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
        error = true;
    }

    match setting.get("match_type").unwrap().as_str() {
        "region" => {
            let match_pattern = setting.get("match_pattern").unwrap();
            if match_pattern.find("<|first_pattern_begin|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|first_pattern_begin|>\x1b[0m\" marker in \"\x1b[0;31mmatch_pattern\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if match_pattern.find("<|first_pattern_end|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|first_pattern_end|>\x1b[0m\" marker in \"\x1b[0;31mmatch_pattern\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if match_pattern.find("<|second_pattern_begin|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|second_pattern_begin|>\x1b[0m\" marker in \"\x1b[0;31mmatch_pattern\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
            if match_pattern.find("<|second_pattern_end|>").is_none() {
                eprintln!("Missing \"\x1b[0;31m<|second_pattern_end|>\x1b[0m\" marker in \"\x1b[0;31mmatch_pattern\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
                error = true;
            }
        }
        "line" => {}
        _ => {
            eprintln!("Invalid value for \"\x1b[0;31mmatch_type\x1b[0m\" attribute in setting located at line \x1b[0;31m{}\x1b[0m", setting_location);
            error = true;
        }
    }

    if error {
        return Err(());
    }

    return Ok(setting);
}
