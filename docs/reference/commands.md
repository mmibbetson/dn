# dn Commands

This document details the commands available with dn as well as their various options.

> NOTE: manpages are available with `man dn`

## new

Create a new note following the dn naming system. Basic usage is as follows:

```sh
dn new
```

By default, the note will be created in `~/Documents/notes` with the following characteristics:

- A timestamp identifier
- `.txt` extension
- No frontmatter

### `new` Options

#### Metadata Options

| Option        | Short | Argument  | Description        | Example                      |
| :------------ | :---: | :-------- | :----------------- | :--------------------------- |
| `--signature` | `-s`  | Signature | Add a signature    | `dn new -s 1a1`              |
| `--title`     | `-t`  | Title     | Add note title     | `dn new -t "My First Note!"` |
| `--keywords`  | `-k`  | Keywords  | Add keywords       | `dn new -k demo_example`     |
| `--extension` | `-e`  | Extension | Set file extension | `dn new -e md`               |

#### Content Options

| Option                   | Short | Argument | Description                                     | Example                   |
| :----------------------- | :---: | :------- | :---------------------------------------------- | :------------------------ |
| `--generate-frontmatter` | `-G`  | None     | Add frontmatter to note                         | `dn new -G`               |
| `--frontmatter-format`   | `-F`  | Format   | Set frontmatter format (text\|yaml\|toml\|json) | `dn new -F toml`          |
| `--template`             | `-T`  | Path     | Use template file for note content              | `dn new -T ./example.txt` |

#### Other Options

| Option        | Short | Argument | Description                         | Example                           |
| :------------ | :---: | :------- | :---------------------------------- | :-------------------------------- |
| `--directory` | `-d`  | Path     | Specify output directory            | `dn new -d ./docs/`               |
| `--config`    | `-c`  | Path     | Use custom config file              | `dn new -c ./special-config.toml` |
| `--print`     | `-p`  | None     | Print absolute path of created note | `dn new -p`                       |

### `new` Examples

#### Basic Note Creation

By providing values for the various segments of a file name, they will be inserted into the appropriate positions of the new file name. The text will be lowercased and sanitised to remove illegal characters [illegal characters are configurable](../reference/configuration.md#illegal-characters).

```sh
# Simple note with title and keywords
dn new --title 'My First Note!' \
       --keywords 'demo example' \
       --extension md

# 20241117T105000--my-first-note__demo_example.md
```

#### Working with Separators

Instead of quoting the values like `'My First Note!'`, you can also use the corresponding segment separator to provide multiple words for a segment. Since the separator for **title** is `-`, the prior could be rewritten as `My-First-Note!`.

```sh
# Separators in metadata are handled automatically
dn new --signature 1a1 \
       --title My-First-Note! \
       --keywords demo_example \
       --extension md.bak

# 20241117T105000==1a1--my-first-note__demo_example.md.bak
```

#### Using Front Matter

If you want to generate front matter for a note you can pass the `--generate-frontmatter` flag and information provided will be used to fill the front matter segments. If no front matter format is provided with `--frontmatter-format`, the default format of `text` will be used.

```sh
# Generate YAML frontmatter
dn new --generate-frontmatter \
       --frontmatter-format yaml \
       -t 'My Note 2: This Time with Front Matter' \
       -k example_kwrds

# 20241117T105000--my-note-2-this-time-with-front-matter__example_kwrds.txt
```

The front matter generated in this new file would look something like the following:

```yaml
---
title: "My Note 2: This Time with Front Matter"
date: 2024-11-20T21:57:00+02:00
tags: ["example", "kwrds"]
identifier: "20241120T215700"
---
```

#### Templates

Sometimes, you may want to repeatedly create notes with the same initial structure. The `--template` flag is a convenient way to fill a new note with text from another file.

```sh
# Use template file
dn new --template ./journal-template.md \
       --keywords journal \
       --extension md

# 20241117T105000__journal.md
```

Assuming the contents of _journal-template.md_ were:

```md
# Journal Entry No. ID

Captain's log, -DATE-.

BODY

Signing out.
```

Then the newly created note would also contain this same content upon creation.

If you also want to add front matter while dealing with templates, the template will be inserted after a line break below the frontmatter. Continuing with the prior example, this command:

```sh
# Combine template with front matter
dn new --generate-frontmatter \
       --template ./journal-template.md \
       --keywords journal \
       --extension md

# 20241117T105000__journal.md
```

Would create a note with the following contents:

```md
date: 2024-11-17
tags: journal
identifier: 20241117T105000
---------------------------

# Journal Entry No. ID

Captain's log, -DATE-.

BODY

Signing out.
```

#### Location and Output

It's useful to be able to pipe the location of a newly created note into other command line programs. Therefore, the `--print` option is provided to print out the absolute path of the newly created file when dn finishes executing. This can be leveraged to automate workflows and integrate dn into the wider ecosystem of terminal tooling.

```sh
# Print absolute path
dn new --print \
       --title show-my-location

# 20241117T105000--show-my-location.txt
```

If you're on a Unix-like system and using the default notes directory, this will print something like `/home/[username]/Documents/notes/20241117T105000--show-my-location.txt`.

You can also specify the directory in which you'd like to create the new note. This is useful if you have reason to make a note outside of your general notes context.

```sh
# Custom output directory
dn new --directory ./private/notes
```

Quite often this will involve wanting to create a new note in your current working directory, which is achieved as you might expect:

```sh
# Use current directory
dn new --directory .
```

#### Configuration

If you don't want to store your configuration file in the standard location, or perhaps have distinct configurations for different note contexts, you can pass in a path to specify which configuration file to use with `--config`.

```sh
# Use custom config file
dn new --config ../dn-configs/special.toml
```

## rename

Rename existing notes following the Denote naming scheme, with options to modify metadata and content. Basic usage is as follows:

```bash
dn rename path/to/note
```

By default, the renamed note will preserve the existing segments if it was previously in valid dn format. If it wasn't, it will attempt to retain the file extension (falling back to .txt if there is none) and treat the previous file name as the **title** segment; an identifier will always be included. The renamed file will always remain in the directory it was in prior to the rename.

### `rename` Options

#### Metadata Renaming Options

| Option                    | Short | Argument  | Description                       | Example                                  |
| :------------------------ | :---: | :-------- | :-------------------------------- | :--------------------------------------- |
| `--regenerate-identifier` | `-I`  | None      | Generate new timestamp identifier | `dn rename ./demo.md -I`                 |
| `--signature`             | `-s`  | Signature | Set new signature                 | `dn rename ./demo.md -s 1a2`             |
| `--title`                 | `-t`  | Title     | Set new title                     | `dn rename ./demo.md -t "Updated Title"` |
| `--keywords`              | `-k`  | Keywords  | Replace all keywords              | `dn rename ./demo.md -k 'new renamed'`   |
| `--add-keywords`          | `-A`  | Keywords  | Add keywords to existing set      | `dn rename ./demo.md -A more_added`      |
| `--remove-keywords`       | `-R`  | Keywords  | Remove keywords from set          | `dn rename ./demo.md -R added`           |
| `--extension`             | `-e`  | Extension | Change file extension             | `dn rename ./demo.md -e md`              |

#### Frontmatter Renaming Options

| Option                   | Short | Argument | Description                         | Example                       |
| :----------------------- | :---: | :------- | :---------------------------------- | :---------------------------- |
| `--from-frontmatter`     | `-f`  | None     | Use frontmatter values for renaming | `dn rename ./demo.md -f`      |
| `--generate-frontmatter` | `-G`  | None     | Generate/regenerate frontmatter     | `dn rename ./demo.md -G`      |
| `--frontmatter-format`   | `-F`  | Format   | Set format (text\|yaml\|toml\|json) | `dn rename ./demo.md -f json` |

#### Other Renaming Options

| Option     | Short | Argument | Description                         | Example                                        |
| :--------- | :---: | :------- | :---------------------------------- | :--------------------------------------------- |
| `input`    |  N/A  | Path     | Path to the note to rename          | `dn rename ./demo.md`                          |
| `--print`  | `-p`  | None     | Print absolute path of renamed note | `dn rename ./demo.md -p`                       |
| `--config` | `-c`  | Path     | Use custom config file              | `dn rename ./demo.md -c ./special-config.toml` |

> NOTE: `input` is a required positional argument, the first value after the `rename` command.

### `rename` Examples

#### Basic Renaming

It's a common occurence when managing notes to need to migrate a file into the dn format or to update an existing valid dn-formatted file with new metadata. The `rename` command can be used for this purpose.

New metadata can be provided with the same metadata options that are present on the `new` command.

```sh
# Rename with new title
dn rename old-note.txt \
          --title "New Title"

# 20241117T105000--new-title.txt
```

For files that already fit the dn format, if there is existing metadata for which a new value is not provided, the old metadata will be preserved.

```sh
# Rename with new signature and extension but preserve title and keywords
dn rename 20241117T105000--old-note__preserved.txt \
          --signature "2b2" \
          --extension "md"

# 20241117T105000==2b2--old-note__preserved.md
```

Because the identifier is generated from a timestamp, an option is provided to generate a new one to ensure the format is correct when one wants to create a new identifier for an existing note.

```sh
# Regenerate identifier with new metadata
dn rename 20241117T105000--note.dj \
          --regenerate-identifier \
          --title "Fresh Title" \
          --keywords "new keywords"

# 20241122T085100--fresh-title__new_keywords.dj
```

#### Working with Keywords

Perhaps the most common use for adjusting existing dn notes is to update their keywords. The three options provided for this are `--keywords`, `--add-keywords`, and `--remove-keywords`.

`--keywords` overwrites the existing metadata the way that the other standard options do.

```sh
# Replace all keywords
dn rename note.txt \
          --keywords "tag1 tag2"

# 20241117T105000--note__tag1_tag2.txt
```

`--add-keywords` extends the value of the existing metadata, or the value of `--keywords` if provided instead. If there are duplicates, they will be ignored.

```sh
# Add additional keywords
dn rename 20241117T105000--note__oldtag.txt \
          --add-keywords "newtag"

# 20241117T105000--note__oldtag_newtag.txt
```

`--remove-keywords` will reduce the existing metadata value or the value of `--keywords` if provided instead. If a keyword is provided which is not present, it will be silently ignored.

```sh
# Remove specific keywords
dn rename 20241117T105000--note__oldtag.txt \
          --remove-keywords "oldtag"

# 20241117T105000--note.txt
```

Finally, these keyword operations can be combined for more complicated adjustments:

```sh
dn rename 20241117T105000--note.txt \
          --keywords oldtag \
          --add-keywords "newtag moretag" \
          --remove-keywords "oldtag moretag"

# 20241117T105000--note__newtag.txt
```

#### Front Matter Operations

The front matter values that dn can generate interact with the `rename` command in various ways. One useful option is to rename a file based on the contents of its front matter. If one were to adjust the contents of an existing note and then change its tags, for example, in the front matter, it may be most convenient to run a command to just adjust the existing file name to match.

```sh
# Rename based on frontmatter values
dn rename note.txt \
          --from-frontmatter
```

> NOTE: When there are multiple sources of metadata they follow a precedence hierarchy: `CLI Options > Frontmatter > Existing Metadata > Defaults`.

Sometimes you may not have existing front matter for a note and want to add it programmatically. This can be achieved with the `--generate-frontmatter` option. Like all other options, it can be combined trivially with the others.

```sh
# Generate new frontmatter during rename
dn rename note.txt \
          --generate-frontmatter \
          --frontmatter-format toml \
          --title "New Title"

# 20241122T085100--new-title.txt
```

This renamed file will now have the following front matter:

```md
+++
title      = "New Title"
date       = 2024-11-22T08:51:00+02:00
identifier = "20241122T085100"
+++
```

#### Other Usage

As with the `new` command, the new path of the renamed note can be printed for use with other CLI tooling.

```sh
# Print new path after renaming
dn rename ~/Documents/notes/note.txt \
          --print \
          --title "Find Me"

# 20241122T085100--find-me.txt
```

If you're on a Unix-like system and using the default notes directory, this will print something like `/home/[username]/Documents/notes/20241122T085100--find-me.txt`.
