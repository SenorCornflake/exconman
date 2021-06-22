# What?
Exconman is a program that allows you to edit configuration files for any program from the command line.
Essentially, it's a glorified search and replace tool.

# Why?
It makes it easier to programmatically manage your dotfiles.

# How?
You create a registry file at `~/.config/exconman/registry.json` containing an array of settings. Each setting has a few options:

1. `name` - The name of the setting, this has to be unique.
2. `file` - The path to the file in which the setting is located.
3. `pattern` - The pattern used to match the setting, possible values are:
	1. A string containing a regex pattern to match one line which the substitute will replace
	2. An array with two strings, both containing a regex pattern to each match one line. If a match is found for both, the substitute replaces all lines between them.
4. `substitute` - A string that replaces whatever the pattern matched. If you put `{value}` in it, whatever value was provided by the user will replace it.
5. `replace` - Specify what you want to replace. This option is ignored if the pattern is an array. Possible values are:
	1. `matched` - As you'd expect, replace what ever matched the pattern
	2. `above` - Replace the entire line above whatever matched the pattern
	2. `below` - Replace the entire line below whatever matched the pattern
6. `value_is_file` - A boolean. If true, it will treat the value provided by the user as a file path, and make it's contents the value which will replace the placeholder `{value}` in the substitute.

# Practical Example
Let's use a shortened BSPWM config file as a example.

bspwmrc:
```
bspc config window_gap 10
bspc config normal_border_color "#000000"
bspc config focused_border_color "#ffffff"
```

registry.json:
```
[
	{
		"name": "bspwm.window_gap",
		"file": "~/.config/bspwm/bspwmrc",
		"pattern": "bspc config window_gap .*",
		"substitute": "bspc config window_gap \"{value}\"",
		"replace": "matched",
		"value_is_file": false
	},
	{
		"name": "bspwm.normal_border_color",
		"file": "~/.config/bspwm/bspwmrc",
		"pattern": "bspc config normal_border_color .*",
		"substitute": "bspc normal_border_color \"{value}\"",
		"replace": "matched",
		"value_is_file": false
	},
	{
		"name": "bspwm.focused_border_color",
		"file": "~/.config/bspwm/bspwmrc",
		"pattern": "bspc config focused_border_color .*",
		"substitute": "bspc config focused_border_color \"{value}\"",
		"replace": "matched",
		"value_is_file": false
	}
]
```

Running `exconman get bspwm.window_gap` will print out `10`.

Running `exconman set bspwm.normal_border_color "#333333"` will modify the line to `bspc config normal_border_color "#333333"`

Running `exconman dump` will print out all the settings and their values in JSON format:
```
{
	"bspwm.window_gap": "10",
	"bspwm.normal_border_color": "#333333",
	"bspwm.focused_border_color": "#ffffff",
}
```
You can also set a bunch of settings in bulk if you have a JSON file in the same format like above using `exconman load path/to/json/file.json`.
