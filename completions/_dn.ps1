
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'dn' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'dn'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'dn' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create a new note')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename an existing note')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dn;new' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Directory in which the note will be created')
            [CompletionResult]::new('--directory', '--directory', [CompletionResultType]::ParameterName, 'Directory in which the note will be created')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Configuration file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Configuration file path')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Template file to add contents to new note')
            [CompletionResult]::new('--template', '--template', [CompletionResultType]::ParameterName, 'Template file to add contents to new note')
            [CompletionResult]::new('-F', '-F ', [CompletionResultType]::ParameterName, 'Frontmatter format')
            [CompletionResult]::new('--frontmatter-format', '--frontmatter-format', [CompletionResultType]::ParameterName, 'Frontmatter format')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Signature for the note')
            [CompletionResult]::new('--signature', '--signature', [CompletionResultType]::ParameterName, 'Signature for the note')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Title for the note')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'Title for the note')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'File extension for the note')
            [CompletionResult]::new('--extension', '--extension', [CompletionResultType]::ParameterName, 'File extension for the note')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'Keywords for the note')
            [CompletionResult]::new('--keywords', '--keywords', [CompletionResultType]::ParameterName, 'Keywords for the note')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Print the absolute path of the created note')
            [CompletionResult]::new('--print', '--print', [CompletionResultType]::ParameterName, 'Print the absolute path of the created note')
            [CompletionResult]::new('-G', '-G ', [CompletionResultType]::ParameterName, 'Generate frontmatter')
            [CompletionResult]::new('--generate-frontmatter', '--generate-frontmatter', [CompletionResultType]::ParameterName, 'Generate frontmatter')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'dn;rename' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Configuration file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Configuration file path')
            [CompletionResult]::new('-F', '-F ', [CompletionResultType]::ParameterName, 'Frontmatter format')
            [CompletionResult]::new('--frontmatter-format', '--frontmatter-format', [CompletionResultType]::ParameterName, 'Frontmatter format')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'New signature for the note')
            [CompletionResult]::new('--signature', '--signature', [CompletionResultType]::ParameterName, 'New signature for the note')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'New title for the note')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'New title for the note')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'New keywords for the note')
            [CompletionResult]::new('--keywords', '--keywords', [CompletionResultType]::ParameterName, 'New keywords for the note')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'Add keywords to the current or new keywords')
            [CompletionResult]::new('--add-keywords', '--add-keywords', [CompletionResultType]::ParameterName, 'Add keywords to the current or new keywords')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'Remove keywords from the current or new keywords')
            [CompletionResult]::new('--remove-keywords', '--remove-keywords', [CompletionResultType]::ParameterName, 'Remove keywords from the current or new keywords')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'New file extension for the note')
            [CompletionResult]::new('--extension', '--extension', [CompletionResultType]::ParameterName, 'New file extension for the note')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Print the absolute path of the created file')
            [CompletionResult]::new('--print', '--print', [CompletionResultType]::ParameterName, 'Print the absolute path of the created file')
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Generate an identifier even if there is an existing one')
            [CompletionResult]::new('--regenerate-identifier', '--regenerate-identifier', [CompletionResultType]::ParameterName, 'Generate an identifier even if there is an existing one')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Rename based on frontmatter values')
            [CompletionResult]::new('--from-frontmatter', '--from-frontmatter', [CompletionResultType]::ParameterName, 'Rename based on frontmatter values')
            [CompletionResult]::new('-G', '-G ', [CompletionResultType]::ParameterName, 'Generate or regenerate frontmatter')
            [CompletionResult]::new('--generate-frontmatter', '--generate-frontmatter', [CompletionResultType]::ParameterName, 'Generate or regenerate frontmatter')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'dn;help' {
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create a new note')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename an existing note')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dn;help;new' {
            break
        }
        'dn;help;rename' {
            break
        }
        'dn;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
