
use builtin;
use str;

set edit:completion:arg-completer[dn] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'dn'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'dn'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand new 'Create a new note'
            cand rename 'Rename an existing note'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'dn;new'= {
            cand -d 'Directory in which the note will be created'
            cand --directory 'Directory in which the note will be created'
            cand -c 'Configuration file path'
            cand --config 'Configuration file path'
            cand -T 'Template file to add contents to new note'
            cand --template 'Template file to add contents to new note'
            cand -F 'Frontmatter format'
            cand --frontmatter-format 'Frontmatter format'
            cand -s 'Signature for the note'
            cand --signature 'Signature for the note'
            cand -t 'Title for the note'
            cand --title 'Title for the note'
            cand -e 'File extension for the note'
            cand --extension 'File extension for the note'
            cand -k 'Keywords for the note'
            cand --keywords 'Keywords for the note'
            cand -p 'Print the absolute path of the created note'
            cand --print 'Print the absolute path of the created note'
            cand -G 'Generate frontmatter'
            cand --generate-frontmatter 'Generate frontmatter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'dn;rename'= {
            cand -c 'Configuration file path'
            cand --config 'Configuration file path'
            cand -F 'Frontmatter format'
            cand --frontmatter-format 'Frontmatter format'
            cand -s 'New signature for the note'
            cand --signature 'New signature for the note'
            cand -t 'New title for the note'
            cand --title 'New title for the note'
            cand -k 'New keywords for the note'
            cand --keywords 'New keywords for the note'
            cand -A 'Add keywords to the current or new keywords'
            cand --add-keywords 'Add keywords to the current or new keywords'
            cand -R 'Remove keywords from the current or new keywords'
            cand --remove-keywords 'Remove keywords from the current or new keywords'
            cand -e 'New file extension for the note'
            cand --extension 'New file extension for the note'
            cand -p 'Print the absolute path of the created file'
            cand --print 'Print the absolute path of the created file'
            cand -I 'Generate an identifier even if there is an existing one'
            cand --regenerate-identifier 'Generate an identifier even if there is an existing one'
            cand -f 'Rename based on frontmatter values'
            cand --from-frontmatter 'Rename based on frontmatter values'
            cand -G 'Generate or regenerate frontmatter'
            cand --generate-frontmatter 'Generate or regenerate frontmatter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'dn;help'= {
            cand new 'Create a new note'
            cand rename 'Rename an existing note'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'dn;help;new'= {
        }
        &'dn;help;rename'= {
        }
        &'dn;help;help'= {
        }
    ]
    $completions[$command]
}
