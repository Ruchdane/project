# Project launcher
Do you need to launch multiple command whenever you want to work on a project ?

`project` aim to make starting working on your project a simple click.

## Config
Global config is located at ~/.config/project/config.yaml

```yml
version: 0.1.0
projects_configs_path: /home/ruchdane/Projects/.projects
menu_command: [fzf]
default_project_command: [zellij, -l, $layout, attach, --create, $name]
``` 

The default configuration uses `fzf` as a project selector

And `zellij` as the project startup script

It looks for projects config in `~/Projects/.projects`


Any value specified in the env section of a project config can be referenced here by prefixing it with the `$` sign

```yml
name: Project Laucher
description: Bridge between dmenu and tmux
path: /home/ruchdane/Projects/project
tags: [rust,cli]
env: 
  layout: rust
  name: project
```

startup command can be overridden in a project

```yml
name: Project Laucher
description: Bridge between dmenu and tmux
path: ~/Projects/project
tags: [rust, cli]
command: [code, $path]
```

## Exemple
### fzf + zellij
### dmenu + tmux
### rofi + code
