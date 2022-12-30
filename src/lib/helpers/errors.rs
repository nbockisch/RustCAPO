//! Provides error definitions for the application
use const_format::{
    concatcp,
    formatcp,
};
use super::defaults;

/// Encapsulates an error message with its associated status code
pub struct CapoError<'a> {
    status_code: i32,
    error_msg: &'a str,
}

impl CapoError<'_> {
    /// CapoError method that prints the associated CapoError's message to
    /// stderr and exits the process with its status code
    pub fn exit_on_error(&self) {
        eprintln!("{}", self.error_msg);
        std::process::exit(self.status_code);
    }
}

// Go-to error messages for common ways the program can fail

/// Default error for when the profile can't be determined
pub const PROFILE_ERROR: CapoError = CapoError {
    status_code: 2,
    error_msg: concatcp!("ERROR: RustCAPO can't deduce the 'profile', give it ",
            formatcp!{
                "the -P argument or set the {} environment variable!",
                defaults::CAPO_ENV_VAR
            }
        ), 
};

/// Default error for when an option argument is missing
pub const OPTION_ERROR: CapoError = CapoError {
    status_code: 3,
    error_msg: "ERROR: either -A or a list of settings is needed!",
};

/// Default error for when a setting is missing
pub const SETTING_ERROR: CapoError = CapoError {
    status_code: 4,
    error_msg: "ERROR: missing setting ",
};

/// Default error for when property files can't be found
pub const PATH_ERROR: CapoError = CapoError {
    status_code: 5,
    error_msg: concat!("Error: unable to locate CAPO files in the required ",
        "path using the current Pofile"),
};

/// Default error for when the user's home directory can't be found
pub const HOME_DIR_ERROR: CapoError = CapoError {
    status_code: 6,
    error_msg: "ERROR: Unable to find the user's home directory"
};
