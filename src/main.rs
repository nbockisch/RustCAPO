use clap::Parser;
use helpers::errors;
use config::config::CapoConfig;

mod helpers;
mod config;
mod tests;

/// Arguments to read CAPO properties
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CapoArgs {
    /// Path of directories to search
    #[arg(long)]
    path: Option<String>,

    /// Display all settings
    #[arg(long, short='A')]
    all: bool,

    /// quiet mode; only display the value
    #[arg(long, short)]
    quiet: bool,

    /// one or more settings to query, ignored if -A
    #[arg(long)]
    settings: Vec<String>,

    /// profile name to use, e.g. test, production
    #[arg(long, short='P')]
    profile: Option<String>,
}

/// This function is called when you type `rustcapo`
fn main() {
    let mut args = CapoArgs::parse();

    // This will exit the program with an error code
    if !args.all && args.settings.is_empty() {
        errors::OPTION_ERROR.exit_on_error();

    }

    // CAPO properties are stored as uppercase, so the arguments must match
    for setting in &mut args.settings { *setting = setting.to_uppercase(); }

    let config = match CapoConfig::new(args.profile, args.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };

    let op_list: Vec<String> = match args.all {
        true => config.options.keys().map(|k| k.clone()).collect(),
        _ => args.settings.clone(),
    };

    for key in op_list {
        let key = key.to_uppercase();
        let (setting, location) = match (config.get(&key),
            config.get_location(&key)) {

            (Some(s), Some(l)) => (s, l),
            _ => {
                errors::SETTING_ERROR.exit_on_error();
                return;
            }
        };

        let format = match args.quiet {
            true => format!("{}", setting),
            _ => format!("{}='{}' # {}",
                fix_key(&key),
                setting,
                location
            ),
        };

        println!("{}", format);
    };
}

/// Clean up the option name for use in shell scripts
///
/// # Arguments
/// * `key` - A ToString with the option name
///
/// # Return
/// A String with the reformatted option name
fn fix_key<T: ToString>(key: &T) -> String {
    key.to_string().replace(".", "_").replace("-", "_")
}
