module completions {

  # A command to manage notes following the Denote naming scheme
  export extern dn [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Create a new note
  export extern "dn new" [
    --print(-p)               # Print the absolute path of the created note
    --directory(-d): string   # Directory in which the note will be created
    --config(-c): string      # Configuration file path
    --template(-T): string    # Template file to add contents to new note
    --signature(-s): string   # Signature for the note
    --title(-t): string       # Title for the note
    --extension(-e): string   # File extension for the note
    --keywords(-k): string    # Keywords for the note
    --help(-h)                # Print help
  ]

  # Rename an existing note
  export extern "dn rename" [
    input: string             # Path to the input file to be renamed
    --print(-p)               # Print the absolute path of the created file
    --regenerate-identifier(-I) # Generate an identifier even if there is an existing one
    --config(-c): string      # Configuration file path
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
