// When getting --add-keywords or --delete-keywords we want to modify the keywords_arg
// 1. Take existing keywords_arg (-k>existing>None)
// 2. format!("{}_{}", keywords_arg, add_keywords_arg)
// 3. split words in remove_keywords_arg
// 4. iterate over formatted string, remove instances of remove keywords arg words from [3]
// 5. collect properly into nice single string, to be used as final keywords_arg value
