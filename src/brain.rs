use std::collections::HashMap;

use crate::config;

pub fn change_value(modifier_path: &str, setting: &str, value: &str, registry: &HashMap<String, HashMap<String, String>>) {
    let modifier = config::parse_modifier(modifier_path);

    if modifier.is_err() {
        return;
    }

    let modifier = config::check_modifier(modifier.unwrap(), modifier_path, &registry);

    if modifier.is_err() {
        return;
    }

}
