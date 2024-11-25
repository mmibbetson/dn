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

> NOTE: The segment prefix/separator characters are implicitly illegal outside of their own segments. So you can provide a '-' in a title argument but not in a keywords argument.

```toml
[file]
illegal_characters = [
    '[',
    ']',
    '{',
    '}',
    '(',
    ')',
    '“',
    '”',
]
```

### Segment Order

The _segment order_ determines the order in which file name segments appear in newly created or renamed files. **All segments are required** - although all segments must be defined for the order, this does not mean they will all be present in every file name. They will only appear in files which have corresponding metadata provided, as expected.

```toml
[file]
segment_order = [
    "Identifier",
    "Signature",
    "Title",
    "Keywords",
    "Extension",
]
```

### Template Path

The _template path_ determines which file to use as template content in a new note by default if none is explicitly provided. This will populate the new note with the contents of the specified file, and so is most useful in configurations specifically intended for a workflow that requires repeated structure. Perhaps you want to us a very particular custom front matter format, or you want all of your journal entries to follow the same initial preamble, etc.

```toml
[file]
template = "~/Directory/notes/templates/journal.txt"
```

## Front Matter Section

### Enabled

The _enabled_ value determines whether or not to generate front matter by default in new notes. If you are intending to always use dn's front matter values and use it across all notes, this is a convenient way to opt into that.

```toml
[frontmatter]
enabled = true
```

### Format

_Format_ determines how metadata will be serialised into front matter. Its allowed values are: "text", "yaml", "toml", and "json". When none is provided, it defaults to "text".

```toml
[frontmatter]
format = "text"
```

### Order

The _order_ determines which front matter segments are generated and in what order. Each segment is optional, so by leaving some out they will not be generated but the others will. If an empty array is provided, no front matter will be generated even when enabled. If this value is not set, then when front matter is enabled, it will default to the value `["title", "date", "keywords", "identifier"]`

```toml
[frontmatter]
order = [
    "title",
    "date",
    "keywords",
    "identifier",
]
```

### Time Style

The _time style_ determines the way that the date and time will be formatted in dn-generated front matter. This can be used to discard time zone information, or only output the date and not the time, for example. The value is expected to be a valid format string in accordance with [chrono's strftime formatting rules](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

```toml
[frontmatter]
time_style = "%FT%T%:z"
```
