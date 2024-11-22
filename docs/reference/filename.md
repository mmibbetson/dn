# File Naming

## File Name Segments

The generated filename follows this pattern by default:

`[identifier]==[signature]--[title]__[keywords].[extension]`

Because the segment order is configurable, it's possible that the `identifier` may not be first. When this is the case, it will be prefixed with `@@`. Other segments, when first, will retain their prefixes:

`--[title]==[signature]__[keywords]@@[identifier].[extension]`
