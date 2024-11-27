module completions {

  # A command to manage notes following the Denote naming scheme
  export extern dn [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  def "nu-complete dn new cli_frontmatter_format" [] {
    [ "text" "yaml" "toml" "json" ]
  }

  # Create a new note
  export extern "dn new" [
    --print(-p)               # Print the absolute path of the created note
    --generate-frontmatter(-G) # Generate frontmatter
    --directory(-d): string   # Directory in which the note will be created
    --config(-c): string      # Configuration file path
    --template(-T): string    # Template file to add contents to new note
    --frontmatter-format(-F): string@"nu-complete dn new cli_frontmatter_format" # Frontmatter format
    --signature(-s): string   # Signature for the note
    --title(-t): string       # Title for the note
    --extension(-e): string   # File extension for the note
    --keywords(-k): string    # Keywords for the note
    --help(-h)                # Print help
  ]

  def "nu-complete dn rename cli_frontmatter_format" [] {
    [ "text" "yaml" "toml" "json" ]
  }

  # Rename an existing note
  export extern "dn rename" [
    input: string             # Path to the input file to be renamed
    --print(-p)               # Print the absolute path of the created file
    --regenerate-identifier(-I) # Generate an identifier even if there is an existing one
    --from-frontmatter(-f)    # Rename based on frontmatter values
    --generate-frontmatter(-G) # Generate or regenerate frontmatter
    --config(-c): string      # Configuration file path
    --frontmatter-format(-F): string@"nu-complete dn rename cli_frontmatter_format" # Frontmatter format
    --signature(-s): string   # New signature for the note
    --title(-t): string       # New title for the note
    --keywords(-k): string    # New keywords for the note
    --add-keywords(-A): string # Add keywords to the current or new keywords
    --remove-keywords(-R): string # Remove keywords from the current or new keywords
    --extension(-e): string   # New file extension for the note
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "dn help" [
  ]

  # Create a new note
  export extern "dn help new" [
  ]

  # Rename an existing note
  export extern "dn help rename" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "dn help help" [
  ]

}

export use completions *
