# dn

`dn` is a CLI tool for creating, managing, and querying plaintext notes. It is inspired by the amazing Emacs package [Denote]() created by [Prot](). dn creates files with a predictable, timestamped naming format (you can specify file format, naming template details and more) that can be quried with dn itself, or through standard tools like `sed`, `grep`, `awk`, `ripgrep`, `fzf`, etc.

dn aims to reproduce much of the functionality of Denote, while being entirely editor agnostic and scriptable in any language. At will adhere to the defaults of Denote as much as possible unless there are justifiable reasons to deviate such as limitation of scope for a CLI or the rare strong opinion of the author.

@@IDENTIFIER
==SIGNATURE
--TITLE
__KEYWORDS
.EXTENSION

## Features

### Current

- 

### In-Progress

- Create single file (Support identifier, name, tag)
- Arbitrary file extension (default to .md but take a flag for others)
- Specify output directory

### Planned

- Convert old file to dn format
- Add/Remove tag(s) from existing file
- Edit name of existing file
- TOML customisation file to be read which can store persistent config like template, file format, etc.
- File creation templates (To add boilerplate to files like prot's examples in Denote)
- Cached scratch-note(s) for short-term non-saved note taking like temp to-do lists etc.
- Query dn note directory by tag, title, identifier
- Progressive query refinement filters
- Open in $EDITOR (root of notes dir if no path, something else if multiple query results)
- Delete file(s) returned by query
- Nice way to compose queries and pipes to create compounds, e.g. query all files before a certain date which contain a specific tag, and delete those
- Frontmatter template to go into every new file (excludable by a no_boilerplate flag of course) which can be different depending on which file extension is being used
- Linking between files (This requires editor integrations. Will probably only bother with nvim myself for the forseeable future)
- Beautiful, friendly error messages
- ignore file for dn
- directory and subdirectory management

## Inspirations

[Denote]()
[jq]()
[Zettelkasten]()

## Dependencies

I try to keep dependencies relatively minimal. Ideally, this will become a [cold-blooded]() project and the dependencies will be vendored for relative permanence.

## Other Stuff

- [Semver]()
- [POSIX Compliance]()
- [CommonMark]()
- [Djot]()
- [Org]()
- [KDL]()
