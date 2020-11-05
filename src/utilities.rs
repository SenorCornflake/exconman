use tilde_expand;

pub fn expand_home(path: &str) -> String {
    let path: Vec<u8> = tilde_expand::tilde_expand(path.as_bytes());
    let path: Result<String, std::string::FromUtf8Error> = String::from_utf8(path);

    return path.unwrap();
}
