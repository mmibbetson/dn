# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_dn_global_optspecs
	string join \n h/help V/version
end

function __fish_dn_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_dn_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_dn_using_subcommand
	set -l cmd (__fish_dn_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c dn -n "__fish_dn_needs_command" -s h -l help -d 'Print help'
complete -c dn -n "__fish_dn_needs_command" -s V -l version -d 'Print version'
complete -c dn -n "__fish_dn_needs_command" -f -a "new" -d 'Create a new note'
complete -c dn -n "__fish_dn_needs_command" -f -a "rename" -d 'Rename an existing note'
complete -c dn -n "__fish_dn_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dn -n "__fish_dn_using_subcommand new" -s d -l directory -d 'Directory in which the note will be created' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s c -l config -d 'Configuration file path' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s T -l template -d 'Template file to add contents to new note' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s F -l frontmatter-format -d 'Frontmatter format' -r -f -a "{text\t'',yaml\t'',toml\t'',json\t''}"
complete -c dn -n "__fish_dn_using_subcommand new" -s s -l signature -d 'Signature for the note' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s t -l title -d 'Title for the note' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s e -l extension -d 'File extension for the note' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s k -l keywords -d 'Keywords for the note' -r
complete -c dn -n "__fish_dn_using_subcommand new" -s p -l print -d 'Print the absolute path of the created note'
complete -c dn -n "__fish_dn_using_subcommand new" -s G -l generate-frontmatter -d 'Generate frontmatter'
complete -c dn -n "__fish_dn_using_subcommand new" -s h -l help -d 'Print help'
complete -c dn -n "__fish_dn_using_subcommand rename" -s c -l config -d 'Configuration file path' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s F -l frontmatter-format -d 'Frontmatter format' -r -f -a "{text\t'',yaml\t'',toml\t'',json\t''}"
complete -c dn -n "__fish_dn_using_subcommand rename" -s s -l signature -d 'New signature for the note' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s t -l title -d 'New title for the note' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s k -l keywords -d 'New keywords for the note' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s A -l add-keywords -d 'Add keywords to the current or new keywords' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s R -l remove-keywords -d 'Remove keywords from the current or new keywords' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s e -l extension -d 'New file extension for the note' -r
complete -c dn -n "__fish_dn_using_subcommand rename" -s p -l print -d 'Print the absolute path of the created file'
complete -c dn -n "__fish_dn_using_subcommand rename" -s I -l regenerate-identifier -d 'Generate an identifier even if there is an existing one'
complete -c dn -n "__fish_dn_using_subcommand rename" -s f -l from-frontmatter -d 'Rename based on frontmatter values'
complete -c dn -n "__fish_dn_using_subcommand rename" -s G -l generate-frontmatter -d 'Generate or regenerate frontmatter'
complete -c dn -n "__fish_dn_using_subcommand rename" -s h -l help -d 'Print help'
complete -c dn -n "__fish_dn_using_subcommand help; and not __fish_seen_subcommand_from new rename help" -f -a "new" -d 'Create a new note'
complete -c dn -n "__fish_dn_using_subcommand help; and not __fish_seen_subcommand_from new rename help" -f -a "rename" -d 'Rename an existing note'
complete -c dn -n "__fish_dn_using_subcommand help; and not __fish_seen_subcommand_from new rename help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
