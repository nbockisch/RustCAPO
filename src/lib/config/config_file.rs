//! This file contains the struct and methods to handle CAPO configuration files

use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{
        Path,
        PathBuf,
    },
};

/// The struct for containing information from the required properties file
pub struct CapoConfigurationFile {
    pub filename: String,
    _path_to_file: PathBuf,
    _last_read: i32,
    pub options: HashMap<String, String>,
}

impl CapoConfigurationFile {
    /// Collect the properties of a CAPO configuration file and return them in
    /// a CapoConfigurationFile struct
    ///
    /// # Arguments
    /// * `profile` - A ToString with the name of the profile
    /// * `path` - An `impl AsRef<Path>` type with the path to the config file
    ///
    /// # Return
    /// An Option containing a CapoConfigurationFile struct with the
    /// configuration file properties or None if they couldn't be read
    pub fn new<S: ToString>(profile: S, path: impl AsRef<Path>)
        -> Option<Self> {

        let mut filename = profile.to_string();
        filename.push_str(".properties");

        let mut path_to_file: PathBuf = path.as_ref().to_owned();
        path_to_file.push(filename.clone());
        let last_read = 0;

        let config_file = match File::open(&path_to_file).ok() {
            Some(c) => c,
            None => {
                eprintln!("WARNING: Couldn't open file: {:?}", path_to_file);
                return None;
            }
        };

        // Read the options into the hashmap or die trying
        let options = match java_properties::read(BufReader::new(config_file)){
            Ok(o) => o,
            Err(e) => {
                eprintln!("WARNING: Couldn't read properties from file: {e}");
                return None;
            },
        };

        Some(Self {
            filename,
            _path_to_file: path_to_file,
            _last_read: last_read,
            options
        })
    }
}
