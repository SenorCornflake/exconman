use tilde_expand;

pub fn expand_home(path: &str) -> String {
    let path: Vec<u8> = tilde_expand::tilde_expand(path.as_bytes());
    let path: Result<String, std::string::FromUtf8Error> = String::from_utf8(path);

    return path.unwrap();
}

pub fn color(txt: String) {
    let txt = txt.replace("<|B|>", "\x1b[0;30m");
    let txt = txt.replace("<|R|>", "\x1b[0;31m");
    let txt = txt.replace("<|R|>", "\x1b[0;32m");
    let txt = txt.replace("<|Y|>", "\x1b[0;33m");
    let txt = txt.replace("<|BL|>", "\x1b[0;34m");
    let txt = txt.replace("<|M|>", "\x1b[0;35m");
    let txt = txt.replace("<|C|>", "\x1b[0;36m");
    let txt = txt.replace("<|W|>", "\x1b[0;37m");
    let txt = txt.replace("<|N|>", "\x1b[0m");
    println!("{}", txt);
}
