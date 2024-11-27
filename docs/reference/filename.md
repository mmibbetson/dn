# File Naming

The file name is the central concern of dn. By imposing a regular structure on file naming, and encoding metadata into the file name itself, notes are naturally integrated with regular expressions (or other parsing mechanisms).

## File Name Segments

There are five segments that comprise the naming scheme for dn:

| Segment    | Purpose                           | Indicator | Separator |
| :--------- | :-------------------------------- | :-------: | :-------: |
| Identifier | Unique identification & timestamp |    @@     |    N/A    |
| Signature  | Sequential file relationships     |    ==     |    N/A    |
| Title      | Standard title                    |    --     |     -     |
| Keywords   | Organisation and tagging          |   \_\_    |    \_     |
| Extension  | Standard file extension           |     .     |     .     |

The generated file names follow this pattern by default:

`[identifier]==[signature]--[title]__[keywords].[extension]`

### Identifier

_Identifier_ is the segment which provides a unique key for each note. It allows precise searching, linking, and ordering. An identifier is **always** required to be present in a note file name. Note that the identifier is not prefixed with its indicator in the default position; because the segment order is configurable, it's possible that the `identifier` may not be first. If and only if this is the case, it will be prefixed with `@@`. Other segments, when first, will retain their prefixes:

`--[title]==[signature]__[keywords]@@[identifier].[extension]`

### Signature

_Signature_ is intended to be used to indicate a sequential relationship between notes, for use in things like the [Zettelkasten]() system. Because it is intended to be a kind of index, there is no facility for multi-part signatures. This is why there is no separator for the segment, the entire signature must be one continuous token after the `==`. In the event that a faulty signature such as `==foo=bar` is provided, the `=` will be removed and the rest will be concatenated into `foobar`.

### Title

_Title_ is a relatively self-explanatory segment. It is what one would conventionally think of as the actual note name, representing the contents much as the title of an essay, or poem, or any other document would. These can have multiple fragments, such as `--my-example-title`.

### Keywords

_Keywords_ is the segment concerned with tagging notes and associating them with different subject matter or general domains. As with the title, there may be multiple fragments in the keywords segment, allowing multiple keywords to be associated with a note, e.g. `__example_metanote`. Keywords will always be placed alphabetically in the file name, according to [tertiary-strength icu4x Unicode Collation](https://docs.rs/icu_collator/1.5.0/icu_collator/index.html).

### Extension

_Extension_ is also somewhat self-evident in its purpose. As with any other file, the extension indicates the format of its contents. It is possible to chain multiple extensions, such as `.temp.dj`, if that provides utility in your use case. Notes must **always** have an extension, and if one isn't provided a default will be generated based on either the configuration file or the program default (".txt").
