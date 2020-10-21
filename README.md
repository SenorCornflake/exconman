# The basic idea
assign regex pattern to keyword, then associate value with keyword, then replace all matches of the regex pattern with the value, the user never sees the regex because he uses the keyword assigned to the regex

The keywords should be changable from the command line, example: (exconman set bspwm_normal_border_color "#00ff00").

The keyword above could be set to find the regex "bspc config normal_border_color .+" and replace it with "bspc config normal_border_color "{{provided_value}}>""". "{{provided_value}}" will be replaced by the value associated with the keyword.
