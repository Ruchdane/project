# Project launcher
Do you need to launch multiple things whenever you want to work on a new project ?
Can you remember everythings needed to get running on that project from three month ago ?

`project` aim to make starting working on your project a simple click.

The default configuration uses `fzf` as a project selector
And zellij as the project startup script

```yml
version: 0.1.0
projects_configs_path: /home/ruchdane/Projects/.projects
menu_command: [fzf]
default_project_command: [zellij, -l, $layout, attach, --create, $name]
```
`Config file located at ~/.config/project/config.yaml`

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
`Example Project config`
The command can be ovverided on a per project basis 

```yml
name: Project Laucher
description: Bridge between dmenu and tmux
path: /home/ruchdane/Projects/project
tags: [rust,cli]
command [code; $path]
env:
  path: /home/ruchdane/Projects/project
```
