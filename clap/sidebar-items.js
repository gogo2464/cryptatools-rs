window.SIDEBAR_ITEMS = {"derive":[["ArgEnum","Deprecated, replaced with [`ValueEnum`] Generates the `ValueEnum` impl."]],"enum":[["AppSettings","Application level settings, which affect how `Command` operates"],["ArgAction","Behavior of arguments when they are encountered while parsing"],["ArgSettings","Various settings that apply to arguments and may be set, unset, and checked via getter/setter methods `Arg::setting`, `Arg::unset_setting`, and `Arg::is_set`. This is what the `Arg` methods which accept a `bool` use internally."],["ColorChoice","Represents the color preferences for program output"],["ErrorKind","Command line argument parser kind of error"],["ValueHint","Provide shell with hint on how to complete an argument."],["ValueSource","Origin of the argument’s value"]],"macro":[["app_from_crate","Deprecated, replaced with [`clap::command!`][crate::command]"],["arg","Create an `Arg` from a usage string."],["command","Allows you to build the `Command` instance from your Cargo.toml at compile time."],["crate_authors","Allows you to pull the authors for the command from your Cargo.toml at compile time in the form: `\"author1 lastname <author1@example.com>:author2 lastname <author2@example.com>\"`"],["crate_description","Allows you to pull the description from your Cargo.toml at compile time."],["crate_name","Allows you to pull the name from your Cargo.toml at compile time."],["crate_version","Allows you to pull the version from your Cargo.toml at compile time as `MAJOR.MINOR.PATCH_PKGVERSION_PRE`"],["value_parser","Select a [`ValueParser`] implementation from the intended type"]],"mod":[["builder","Define [`Command`] line [arguments][`Arg`]"],["error","Error reporting"],["parser","[`Command`][crate::Command] line argument parser"]],"struct":[["App","Deprecated, replaced with [`Command`]"],["Arg","The abstract representation of a command line argument. Used to set all the options and relationships that define a valid argument for the program."],["ArgGroup","Family of related arguments."],["ArgMatches","Container for parse results."],["Indices","Iterate over indices for where an argument appeared when parsing, via `ArgMatches::indices_of`"],["OsValues","Deprecated, replaced with [`ArgMatches::get_many()`]"],["PossibleValue","A possible value of an argument."],["Values","Deprecated, replaced with [`ArgMatches::get_many()`]"]],"trait":[["ArgEnum","Parse arguments into enums."],["Args","Parse a set of arguments into a user-defined container."],["CommandFactory","Create a [`Command`] relevant for a user-defined container."],["FromArgMatches","Converts an instance of [`ArgMatches`] to a user-defined container."],["IntoApp","Create a [`Command`] relevant for a user-defined container."],["Parser","Parse command-line arguments into `Self`."],["Subcommand","Parse a sub-command into a user-defined enum."],["ValueEnum","Parse arguments into enums."]],"type":[["Command","Build a command-line interface."]]};