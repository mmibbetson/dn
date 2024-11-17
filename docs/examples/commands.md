<!--
SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
SPDX-FileContributor: Matthew Mark Ibbetson

SPDX-License-Identifier: GPL-3.0-or-later
-->

# dn Commands

> Full manual pages available with `man dn`

## new

Create a new note.

### Basic Usage

```sh
dn new
```

Creates a new note with default settings:

- Timestamp identifier
- `.txt` extension
- No frontmatter
- Default file location is `~/Documents/notes`

### File Naming Components

The generated filename follows this pattern:

`[identifier]==[signature]--[title]__[keywords].[extension]`

Because the segment order is configurable, it's possible that the `identifier` may not be first. When this is the case, it will be prefixed with `@@`. Other segments, when first, will retain their prefixes:

`--[title]==[signature]__[keywords]@@[identifier].[extension]`

### Options

#### Metadata Options

| Option        | Short | Description        | Example                      |
| ------------- | ----- | ------------------ | ---------------------------- |
| `--signature` | `-s`  | Add a signature    | `dn new -s 1a1`              |
| `--title`     | `-t`  | Add note title     | `dn new -t "My First Note!"` |
| `--keywords`  | `-k`  | Add keywords       | `dn new -k demo_example`     |
| `--extension` | `-e`  | Set file extension | `dn new -e md`               |

#### Content Options

| Option                   | Short | Description                                    |
| ------------------------ | ----- | ---------------------------------------------- |
| `--generate-frontmatter` | `-G`  | Add frontmatter to note                        |
| `--frontmatter-format`   | `-F`  | Set frontmatter format (text\|yaml\|toml\|org) |
| `--template`             | `-T`  | Use template file for note content             |

#### Other Options

| Option        | Short | Description                         |
| ------------- | ----- | ----------------------------------- |
| `--directory` | `-d`  | Specify output directory            |
| `--config`    | `-c`  | Use custom config file              |
| `--print`     | `-p`  | Print absolute path of created note |

### Examples

#### Basic Note Creation

```sh
# Simple note with title and keywords
dn new --title 'My First Note!' --keywords 'demo example' --extension md
```

Result: 20241117T105000--my-first-note\_\_demo_example.md

#### Working with Separators

```sh
# Separators in metadata are handled automatically
dn new --signature 1a1 --title My-First-Note! --keywords demo_example --extension md.bak
```

Result: 20241117T105000==1a1--my-first-note\_\_demo_example.md.bak

#### Using Frontmatter

```sh
# Generate YAML frontmatter
dn new --generate-frontmatter --frontmatter-format yaml \
       -t 'My Note 2: This Time with Front Matter' -k example_kwrds
```

Result: 20241117T105000--my-note-2-this-time-with-front-matter\_\_example_kwrds.txt

#### Templates and Content

```sh
# Use template file
dn new --template ./journal-template.md --keywords journal --extension md
```

Result: 20241117T105000\_\_journal.md (with template content)

```sh
# Combine template with frontmatter
dn new --template ./journal-template.md --generate-frontmatter
```

Result: 20241117T105000\_\_journal.md (frontmatter + template content)

#### Location and Output

```sh
# Print absolute path
dn new --print --title show-my-location
# Creates file and prints its absolute path

# Custom output directory
dn new --directory ./private/notes
# Creates note in specified directory

# Use current directory
dn new --directory .
```

#### Configuration

```sh
# Use custom config file
dn new --config ../dn-configs/special.toml
```

## rename

Rename existing notes following the Denote naming scheme, with options to modify metadata and content.

### Basic Usage

```bash
dn rename path/to/note.txt
```

### Options

#### Core Options

| Option                    | Short | Description                         |
| ------------------------- | ----- | ----------------------------------- |
| `input`                   | N/A   | Path to the note to rename          |
| `--print`                 | `-p`  | Print absolute path of renamed note |
| `--regenerate-identifier` | `-I`  | Generate new timestamp identifier   |
| `--config`                | `-c`  | Use custom config file              |

> NOTE: `input` is a required positional argument, the first value after the `rename` command.

#### Metadata Options

| Option              | Short | Description                  | Example               |
| ------------------- | ----- | ---------------------------- | --------------------- |
| `--signature`       | `-s`  | Set new signature            | `--s 1a2`             |
| `--title`           | `-t`  | Set new title                | `--t "Updated Title"` |
| `--keywords`        | `-k`  | Replace all keywords         | `--k new_renamed`     |
| `--add-keywords`    | `-A`  | Add keywords to existing set | `--A "more added"`    |
| `--remove-keywords` | `-R`  | Remove keywords from set     | `--R added`           |
| `--extension`       | `-e`  | Change file extension        | `--e md`              |

#### Content Options

| Option                   | Short | Description                         |
| ------------------------ | ----- | ----------------------------------- |
| `--frontmatter`          | `-f`  | Use frontmatter values for renaming |
| `--generate-frontmatter` | `-G`  | Generate/regenerate frontmatter     |
| `--frontmatter-format`   | `-F`  | Set format (text\|yaml\|toml\|org)  |

### Examples

#### Basic Renaming

```sh
# Rename with new title
dn rename old-note.txt --title "New Title"
```

Result: [timestamp]--new-title.[ext]

```sh
# Rename with new signature and extension
dn rename old-note.txt --signature "2b2" --extension "md"
```

Result: [timestamp]==2b2--[title].md

#### Working with Keywords

```sh
# Replace all keywords
dn rename note.txt --keywords "tag1 tag2"
```

Result: 20241117T105000--note\_\_tag1_tag2.txt

```sh
# Add additional keywords
dn rename 20241117T105000--note__oldtag.txt --add-keywords "newtag"
```

Result: 20241117T105000--note\_\_oldtag_newtag.txt

```sh
# Remove specific keywords
dn rename 20241117T105000--note__oldtag.txt --remove-keywords "oldtag"
```

Result: 20241117T105000--note.txt

Keyword operations can be combined:

```sh
dn rename 20241117T105000--note__oldtag.txt --add-keywords "newtag moretag" --remove-keywords "oldtag moretag"
```

Result: 20241117T105000--note\_\_newtag.txt

#### Frontmatter Operations

```sh
# Rename based on frontmatter values
dn rename note.txt --from-frontmatter
```

Uses title, keywords, etc. from file's frontmatter

```sh
# Generate new frontmatter during rename
dn rename note.txt --generate-frontmatter --frontmatter-format yaml --title "New Title"
```

Renames file and updates/adds YAML frontmatter

#### Advanced Usage

```sh
# Regenerate identifier with new metadata
dn rename note.txt --regenerate-identifier --title "Fresh Title" --keywords "new keywords"
```

Result: [new-timestamp]--fresh-title\_\_new_keywords.[ext]

```sh
# Print new path after renaming
dn rename note.txt --print --title "Find Me"
```

Renames file and prints its new absolute path
