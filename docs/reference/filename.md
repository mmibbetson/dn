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

Note that the identifier is not prefixed with its indicator in the default position; because the segment order is configurable, it's possible that the `identifier` may not be first. If and only if this is the case, it will be prefixed with `@@`. Other segments, when first, will retain their prefixes:

`--[title]==[signature]__[keywords]@@[identifier].[extension]`
