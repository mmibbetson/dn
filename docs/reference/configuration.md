# Configuration File

This document contains the details of every configuration option available in the dn configuration file. All values that can be set with a command-line option will override those set within the configuration file, with the exception of boolean values that have been set to true in the configuration file (they must necessarily be considered to be true even when they are not passed as an option in the command line).

## File Section

### Default Extension

The _default extension_ determines the value of the `Extension` segment of a note when not explicitly provided. If this value is not set in the configuration file, it will be "txt". If you tend to take all of your notes in a particular plaintext format, such as [djot](https://djot.net/) or [Markdown](https://commonmark.org/), it can be more convenient to specify this here than to repeatedly specify it each time a new note is created.

```toml
[file]
default_extension = "dj"
```

### Directory

The _directory_ value determines where notes will be created by default when no output path is specified with the `dn new` command. It expects an **absolute** path as its value, and not a relative path. When _directory_ is not set, dn will attempt to write files to $HOME/Documents/notes or $USERPROFILE/Documents/notes. If neither $HOME nor $USERPROFILE are able to be acquired from the environment, it will instead write in the current working directory.

```toml
[file]
directory = "~/Directory/notes"
```

### Illegal Characters

_Illegal characters_ is a list of characters which are not permitted to appear in any segment of the file name. Where they do appear in provided values, they will be removed and the letters around them will be concatenated. For example, if '[' is llegal and a title value is provided as "new[[no[te", it will be sanitised into "newnote".

This following example represents the default value for illegal characters:

```toml
[file]
illegal_characters = [
    '[',
    ']',
    '{',
    '}',
    '(',
    ')',
    '!',
    '#',
    '$',
    '%',
    '^',
    '&',
    '*',
    '+',
    '\',
    '"',
    "'",
    '?',
    ',',
    '|',
    ';',
    ':',
    '~',
    '`',
    '‘',
    '’',
    '“',
    '”',
    '/',
    '*',
    ' ',
    '@',
    '=',
    '-',
    '_',
    '.',
]
```

> NOTE: The segment prefix/separator characters are implicitly illegal outside of their own segments. So you can provide a '-' in a title argument but not in a keywords argument.

### Segment Order

The _segment order_ determines the order in which file name segments appear in newly created or renamed files. **All segments are required** - although all segments must be defined for the order, this does not mean they will all be present in every file name. They will only appear in files which have corresponding metadata provided, as expected.

```toml
[file]
segment_order = [
    "identifier",
    "signature",
    "title",
    "keywords",
    "extension",
]
```

### Template Path

The _template path_ determines which file to use as template content in a new note by default if none is explicitly provided. This will populate the new note with the contents of the specified file, and so is most useful in configurations specifically intended for a workflow that requires repeated structure.

```toml
[file]
template = "~/Directory/notes/templates/journal.txt"
```
