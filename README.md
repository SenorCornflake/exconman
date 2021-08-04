# What
Exconman is a program that will edit your config files for you so you don't have to.

# Why
You can have multiple themes for each program without having more than one config file for the program itself. 

# How
There are two configuration files for exconman located in `~/.config/exconman`, one is `registry.json` and the other is `config.json`.

`registry.json` contains the information required to locate the text in a file that will be changed. For example:

```
[
	{
		"name": "bspwm.normal_border_color",
		"file": "~/.config/bspmwm/bspwmrc",
		"pattern": "bspc config normal_border_color .+",
		"substitute": "bspc config normal_border_color {value}",
	}
]
```
Considering the above registry, if we wanted to set the normal border color for bspwm to `#333333` we'd run `exconman set bspwm.normal_border_color "#333333"`.

This registry contains ONE setting identified by the **name** `bspwm.normal_border_color`.
It will open the **file** `~/.config/bspwm/bspwmrc`, then it will look for any text that matches a regex **pattern**, in this case, the pattern is `bspc config normal_border_color .+`,
after it has found a match, it will replace whatever text was matched with the **substitute**, the `{value}` placeholder will be replaced by whatever text is provided in the exconman command.

# Documentation (Work in progress)
## Registry Settings
1. `name`: The name of the setting, should be unique.
2. `file`: The file where the pattern is searched for.
3. `pattern`: The pattern to search for, any valid rust regex will work here. If this is a pattern, it will search for a line, but if it is an array containing two patterns, it will match any two lines that match those patterns then it will replace all text between them with the substitute.
4. `substitute`: The substitute that will replace the text that matches the pattern. `{value}` will be replaced by the value given to the exconman command.
5. `replace`: This is optional. You can choose which line you are replacing. `line_above` will replace the entire line above the line that matched the pattern with the substitute. `line_below` does the same as `line_above` but for the line below. `matched_text` is the default behaviour, it only replaces the text that matched the pattern.
6. `read_value_path`: This is optional. The value will be interpreted as a file path and the file's contents will be used as the value instead. The default is `false`.
7. `multiple`: This is optional. If this setting is set to true, it will replace all matches instead of just the first one.

