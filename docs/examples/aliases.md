# dn Shell Aliases & Functions

dn is designed to be composable in the shell. Sometimes, typing out long pipelines of commands with various configurable options is tedious; this is especially true for repeated actions. As such, this document contains a few examples of useful shell aliases & functions which can streamline this process.

## Bash

```bash
DN_DIRECTORY="$HOME/Documents/notes"
DN_TEMPLATES_DIRECTORY="$DN_DIRECTORY/templates"

alias notelist='rg --files "$DN_DIRECTORY" | fzf'
alias templist='rg --files "$DN_TEMPLATES_DIRECTORY" | fzf'
alias dnd='cd "$DN_DIRECTORY"'

dno() {
  local regexp="$1"
  rg --files "*$regexp*" "$DN_DIRECTORY" | fzf | xargs "$EDITOR"
}

dnt() {
  local template="$1"
  local title="$2"
  dn new --template "$DN_TEMPLATES_DIRECTORY/$template" --title "$title" --print | xargs "$EDITOR"
}

dnr() {
  local old=$(rg --files "$DN_DIRECTORY" | fzf)
  local new="$1"
  dn rename "$old" -t "$new"
}
```

## Zsh

```zsh
DN_DIRECTORY="~/Documents/notes"
DN_TEMPLATES_DIRECTORY="$DN_DIRECTORY/templates"

alias notelist="rg --files $DN_DIRECTORY | fzf"
alias templist="rg --files $DN_TEMPLATES_DIRECTORY | fzf"
alias dnd="cd $DN_DIRECTORY"

function dno() {
  local regexp="$1"
  rg --files "*$regexp*" $DN_DIRECTORY | fzf | xargs $EDITOR
}

function dnt() {
  local template="$1"
  local title="$2"
  dn new --template "$DN_TEMPLATES_DIRECTORY/$template" --title "$title" --print | xargs $EDITOR
}

function dnr() {
  local old=$(rg --files "$DN_DIRECTORY" | fzf)
  local new="$1"
  dn rename "$old" -t "$new"
}
```

## Fish

```fish
set -gx DN_DIRECTORY "$HOME/Documents/notes"
set -gx DN_TEMPLATES_DIRECTORY "$DN_DIRECTORY/templates"

alias notelist 'rg --files "$DN_DIRECTORY" | fzf'
alias templist 'rg --files "$DN_TEMPLATES_DIRECTORY" | fzf'
alias dnd 'cd "$DN_DIRECTORY"'

function dno --description "Open a note matching a regex"
  set -l regexp $argv[1]
  rg --files "*$regexp*" "$DN_DIRECTORY" | fzf | xargs $EDITOR
end

function dnt --description "Create a new note from a template"
  set -l template $argv[1]
  set -l title $argv[2]
  dn new --template "$DN_TEMPLATES_DIRECTORY/$template" --title "$title" --print | xargs $EDITOR
end

function dnr --description "Rename a note"
  set -l old (rg --files "$DN_DIRECTORY" | fzf)
  set -l new $argv[1]
  dn rename "$old" -t "$new"
end
```

## Nushell

```nushell

let DN_DIRECTORY = "~/Documents/notes"
let DN_TEMPLATES_DIRECTORY = $"$DN_DIRECTORY/templates"

alias notelist = {|| rg --files $DN_DIRECTORY | fzf }
alias templist = {|| rg --files $DN_TEMPLATES_DIRECTORY | fzf }
alias dnd = {|| cd $DN_DIRECTORY }

fn dno [regexp] {
  let file = (rg --files $"*$regexp*" $DN_DIRECTORY | fzf)
  $EDITOR $file
}

fn dnt [template title] {
  let new_file = (dn new --template $"$DN_TEMPLATES_DIRECTORY/$template" --title $title --print)
  $EDITOR $new_file
}

fn dnr [new] {
  let old = (rg --files $DN_DIRECTORY | fzf)
  dn rename $old -t $new
}
```

## Elvish

```elvish
let DN_DIRECTORY = $"$env:HOME/Documents/notes"
let DN_TEMPLATES_DIRECTORY = $"$DN_DIRECTORY/templates"

alias notelist = rg --files $DN_DIRECTORY | fzf
alias templist = rg --files $DN_TEMPLATES_DIRECTORY | fzf
alias dnd = cd $DN_DIRECTORY

def dno [regexp: string] {
  let file = (rg --files $"*$regexp*" $DN_DIRECTORY | fzf)
  $EDITOR $file
}

def dnt [template: string, title: string] {
  let new_file = (dn new --template $"$DN_TEMPLATES_DIRECTORY/$template" --title $title --print)
  $EDITOR $new_file
}

def dnr [new: string] {
  let old = (rg --files $DN_DIRECTORY | fzf)
  dn rename $old -t $new
}
```

## Powershell

```powershell
$DN_DIRECTORY = Join-Path $HOME "Documents\notes"
$DN_TEMPLATES_DIRECTORY = Join-Path $DN_DIRECTORY "templates"

function notelist {
  rg --files $DN_DIRECTORY | fzf
}

function templist {
  rg --files $DN_TEMPLATES_DIRECTORY | fzf
}

function dnd {
  cd $DN_DIRECTORY
}

function dno {
  param(
    [string]$regexp
  )
  rg --files "*$regexp*" $DN_DIRECTORY | fzf | & $EDITOR
}

function dnt {
  param(
    [string]$template,
    [string]$title
  )
  dn new --template (Join-Path $DN_TEMPLATES_DIRECTORY $template) --title $title --print | & $EDITOR

}

function dnr {
  param(
    [string]$new
  )
  $old = rg --files $DN_DIRECTORY | fzf
  dn rename $old -t $new
}
```
