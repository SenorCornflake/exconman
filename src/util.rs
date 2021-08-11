/// This file contains functions that assists development

use shellexpand; 

pub fn expand_env_vars(string: &str) -> String {
    let expanded = shellexpand::full(string);

    if expanded.is_ok() {
        return expanded
            .unwrap()
            .to_string();
    }
    
    string.to_string()
}

pub fn color<'a>(color: &'a str, ground: &'a str) -> &'a str {
    match ground {
		"clear" => "\x1b[0m",
        "fg" => {
            match color {
                "black"        => "\x1b[0;30m",
                "red"          => "\x1b[0;31m",
                "green"        => "\x1b[0;32m",
                "yellow"       => "\x1b[0;33m",
                "blue"         => "\x1b[0;34m",
                "magenta"      => "\x1b[0;35m",
                "cyan"         => "\x1b[0;36m",
                "white"        => "\x1b[0;37m",
                "dark_grey"    => "\x1b[0;90m",
                "light_red"    => "\x1b[0;91m",
                "light_green"  => "\x1b[0;92m",
                "light_yellow" => "\x1b[0;93m",
                "light_blue"   => "\x1b[0;94m",
                "light_purple" => "\x1b[0;95m",
                "turquoise"    => "\x1b[0;96m",
                "light_white"  => "\x1b[0;97m",
				_              => ""
            }
        }
        "bg" => {
            match color {
                "black"        => "\x1b[0;40m",
                "red"          => "\x1b[0;41m",
                "green"        => "\x1b[0;42m",
                "yellow"       => "\x1b[0;43m",
                "blue"         => "\x1b[0;44m",
                "magenta"      => "\x1b[0;45m",
                "cyan"         => "\x1b[0;46m",
                "white"        => "\x1b[0;47m",
                "dark_grey"    => "\x1b[0;100m",
                "light_red"    => "\x1b[0;101m",
                "light_green"  => "\x1b[0;102m",
                "light_yellow" => "\x1b[0;103m",
                "light_blue"   => "\x1b[0;104m",
                "light_purple" => "\x1b[0;105m",
                "turquoise"    => "\x1b[0;106m",
                "light_white"  => "\x1b[0;107m",
				_              => ""
            }
        }
        _ => ""
    }
}

