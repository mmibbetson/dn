const std = @import("std");

pub fn main() !void {
    // Parse CLI commands & flags.
    // Create file.
    // Log status to user.
}

const PREFIX_IDENTIFIER = "@@";
const PREFIX_SIGNATURE = "==";
const PREFIX_TITLE = "--";
const PREFIX_KEYWORDS = "__";

const SEPARATOR_TITLE = '-';
const SEPARATOR_KEYWORDS = '_';

const DEFAULT_FILETYPE = ".txt";

const ComponentNames = enum { identifier, signature, title, keywords };
const DEFAULT_COMPONENT_ORDER = [4]ComponentNames{ ComponentNames.identifier, ComponentNames.signature, ComponentNames.title, ComponentNames.keywords };

const FilenameComponents = struct { identifier: []const u8, signature: []const u8, title: []const u8, keywords: []const u8, filetype: []const u8 };

// Handle CLI flags. For now, only --help/-h, --version/-v --date, --signature, --title, --keywords, --filetype

// Create identifier. Either use provided date or generate one.
// Format: yyyyMMdd ++ T ++ hhmmss
// fn create_identifier(date: ?[]const u8) []const u8 {}

// Create signature if present.
// fn create_signature(sig_string: ?[]const u8) ?[]const u8 {}

// Create title if present. Joining list by separator. Assuming they come from cli, is it an ArrayList or a string?
// fn create_title(title_string: ?[]const u8) ?[]const u8 {}

// Create keywords. Joining list by separator. Assuming they come from cli, is it an ArrayList or a string?
// fn create_keywords(keys_string: ?[]const u8) ?[]const u8 {}

// Join components. Enforce lowercase. Add file extension, default to .txt if not present.
// fn create_filename(filename_components: FilenameComponents, component_order: []ComponentNames) []const u8 {}

// fn create_frontmatter(format: FrontmatterFormat) ?[]const u8 {}

// fn create_file(filename: []const u8, frontmatter: ?[]const u8) !void {}

test "simple test" {}
