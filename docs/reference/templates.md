# Templates

Templates are simply files which contain content that will be injected into any new files which they are applied to. By applying a template when creating a new note, the note will be created with the content of that template file in it. This can be used to make repeated workflows more ergonomic --- for example, tracking a daily checklist of responsibilities.

## Template Files

A template file can be any file with plain text content. Because templates are applied by providing a path, you can organise them however is most convenient to you. One simple example of this is to maintain a `templates/` directory inside of your notes directory, and to create a simple shell function to easily create new notes with a template applied:

```sh
# Example zsh function
function dnt() {
    local template="$1"
    local title="$2"
    
    dn new --template "$template" --title "$title" --print | xargs $EDITOR
}
```
