# Project launcher(in rust)

Try to rewrite my project launcher wich is a rofi script in rust

```bash
#!/bin/bash

# Path to your project configuration files
PROJECTS_DIR="$HOME/Projects/.launch"

# Get list of project configuration files
PROJECTS_CONFIGS=$(ls $PROJECTS_DIR/*.yml)

# Loop through the tmuxp configuration files and read their descriptions
for CONFIG in $PROJECTS_CONFIGS; do
    NAME=$(grep -i '^Name:' ${CONFIG}.desc | cut -d ' ' -f 2-)
    DESCRIPTION=$(grep -i '^Description:' ${CONFIG}.desc | cut -d ' ' -f 2-)
    TAGS=$(grep -i '^Tags:' ${CONFIG}.desc | cut -d ' ' -f 2-)
    if [ -z "$MENU_ITEMS" ]; then
        MENU_ITEMS="$NAME | $DESCRIPTION | $TAGS | $CONFIG"
    else
        MENU_ITEMS="$MENU_ITEMS\n$NAME | $DESCRIPTION | $TAGS | $CONFIG"
    fi
done

# Rofi theme location
ROFITHEME="$HOME/.config/rofi/themes/dracula.rasi"
# Prompt user to select a configuration file using dmenu
SELECTED=$(echo -e "$MENU_ITEMS" | rofi -theme $ROFITHEME -dmenu  -i -l 10 -p "Select a tmuxp configuration:" | rev | cut -d '|' -f 1 | rev)

# Launch the selected configuration file with tmuxp
if [ ! -z "$SELECTED" ]; then
    # Launch the selected project
    tmuxp load $SELECTED
fi
```