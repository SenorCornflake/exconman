
pub fn expand_home(path: &str) -> String {
    return path.replace("~", std::env::var("HOME").unwrap().as_str());
}
