# The basic idea
assign regex pattern to keyword, then associate value with keyword, then replace all matches of the regex pattern with the value, the user never sees the regex because he uses the keyword assigned to the regex

The keywords should be changable from the command line, example: (exconman set bspwm_normal_border_color "#00ff00").

The keyword above could be set to find the regex "bspc config normal_border_color .+" and replace it with "bspc config normal_border_color "{{provided_value}}>""". "{{provided_value}}" will be replaced by the value associated with the keyword.

## File Locations
The file storing the last values associated with the keywords with be located in .local/exconman/values (NOTE: This will not be necessary if I can find a way to get values from the application config file itself)
The file storing the possible keywords and their settings such as the regex and the various settings related to the keywords with be located at .config/exconman/settings

The extensions of the files are not decided yet.

## Basic CLI
"exconman set <key> <value>" should replace regex matches associated to key with value
"exconman list" should list all keys
"exconman load <file>" should load key/value pairs from a file
"exconman get <key>" get last value key was associated with.

## Features
should be able to allow value to be a path, and then make the value the contents of that file.
should be able to match two lines and replace whatever is in between.
should be able to replace the line above or below the one containing the match
should be able to allow user to allow changing all matches, the first match or the last match to the value associated with the key
