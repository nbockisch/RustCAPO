//! This file contains the struct and methods to handle the CAPO configuration
use dirs;
use std::{
    collections::HashMap,
    env,
    path::PathBuf, error::Error,
};

use super::config_file::CapoConfigurationFile;
use crate::helpers::{
    errors,
    defaults,
};

/// The struct for fetching CAPO configuration settings
pub struct CapoConfig {
    pub profile: String,
    pub path: String,
    pub options: HashMap<String, String>,
    pub locations: HashMap<String, String>,
    pub cfg_files: Vec<CapoConfigurationFile>,
}

impl CapoConfig {
    /// Configure a new CapoConfig with a profile and path
    ///
    /// # Arguments
    /// * `profile` - An `Option<ToString>` with a provided profile or None if the
    /// environment variable is to be used
    /// * `path` - An `Option<ToString>` With a provided path or None if the
    /// environment variable is to be used
    ///
    /// # Return
    /// A CapoProfile object constructed from the path and profile
    pub fn new<S: ToString>(profile: Option<S>, path: Option<S>)
        -> Result<Self, Box<dyn Error>> {

        // Use the provided profile, otherwise the environment variable,
        // otherwise error
        let profile = match profile {
            Some(p) => p.to_string(),
            None => match env::var(defaults::CAPO_ENV_VAR) {
                Ok(p) => p,
                Err(_) => {
                    errors::PROFILE_ERROR.exit_on_error();
                    String::new()
                },
            },
        };

        // Use the provided path, otherwise the environment variable, otherwise
        // the default
        let mut path = match path {
            Some(p) => p.to_string(),
            None => match env::var(defaults::CAPO_PATH_VAR) {
                Ok(p) => p,
                Err(_) => defaults::DEFAULT_CAPO_PATH.to_string(),
            }
        };

        // Append the user's capo path
        let home_dir = match dirs::home_dir() {
            Some(h) => h,
            None => {
                errors::HOME_DIR_ERROR.exit_on_error();
                PathBuf::new()
            },
        };

        path.push_str(format!(":{}.capo", home_dir.display()).as_str());

        let mut config = Self {
            profile,
            path,
            options: HashMap::new(),
            locations: HashMap::new(),
            cfg_files: Vec::new(),
        };

        // Get config files and properties
        config.get_files();
        if config.cfg_files.len() <= 0 {
            errors::PATH_ERROR.exit_on_error();
        }

        config.load_merged_config();

        Ok(config)
    }

    /// Create a single properties list from all loaded configuration files
    fn load_merged_config(&mut self) {
        for file in &self.cfg_files {
            for (key, val) in &file.options {
                self.options.insert(key.clone().to_uppercase(), val.clone());
                self.locations.insert(key.clone().to_uppercase(),
                    file.filename.clone());
            }
        }
    }

    /// Populate the CapoConfig object's configuration file list
    fn get_files(&mut self) {
        for path in self.path.split(':') {
            match CapoConfigurationFile::new(self.profile.clone(),
                path.clone()) {

                Some(c) => self.cfg_files.push(c),
                None => continue,
            };
        }
    }

    // Methods for retrieving properties

    /// Get the value of a CAPO property as a String
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the property value as a String or None if the property
    /// isn't found
    pub fn get<S: ToString>(&self, key: &S) -> Option<String> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Get the value of a CAPO property as a u128
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the u128 value if the property can be represented as a u128
    /// and None otherwise
    pub fn get_u128<S: ToString>(&self, key: &S) -> Option<u128> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<u128>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an i128
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the i128 value if the property can be represented as an
    /// i128 and None otherwise
    pub fn get_i128<S: ToString>(&self, key: &S) -> Option<i128> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<i128>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a u64
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the u64 value if the property can be represented as a u64
    /// and None otherwise
    pub fn get_u64<S: ToString>(&self, key: &S) -> Option<u64> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<u64>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an i64
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the i64 value if the property can be represented as an
    /// i64 and None otherwise
    pub fn get_i64<S: ToString>(&self, key: &S) -> Option<i64> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<i64>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a u32
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the u32 value if the property can be represented as a u32
    /// and None otherwise
    pub fn get_u32<S: ToString>(&self, key: &S) -> Option<u32> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<u32>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an i32
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the i32 value if the property can be represented as an
    /// i32 and None otherwise
    pub fn get_i32<S: ToString>(&self, key: &S) -> Option<i32> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<i32>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a u16
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the u16 value if the property can be represented as an
    /// u16 and None otherwise
    pub fn get_u16<S: ToString>(&self, key: &S) -> Option<u16> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<u16>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an i16
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the i16 value if the property can be represented as an
    /// i16 and None otherwise
    pub fn get_i16<S: ToString>(&self, key: &S) -> Option<i16> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<i16>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a u8
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the u8 value if the property can be represented as a u8
    /// and None otherwise
    pub fn get_u8<S: ToString>(&self, key: &S) -> Option<u8> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<u8>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an i8
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the i8 value if the property can be represented as an i8
    /// and None otherwise
    pub fn get_i8<S: ToString>(&self, key: &S) -> Option<i8> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<i8>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an f64
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the f64 value if the property can be represented as an
    /// f64 and None otherwise
    pub fn get_f64<S: ToString>(&self, key: &S) -> Option<f64> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<f64>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an f32
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the f32 value if the property can be represented as an
    /// f32 and None otherwise
    pub fn get_f32<S: ToString>(&self, key: &S) -> Option<f32> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<f32>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a usize
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the usize value if the property can be represented as a
    /// usize and None otherwise
    pub fn get_usize<S: ToString>(&self, key: &S) -> Option<usize> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<usize>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as an isize
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the isize value if the property can be represented as an
    /// isize and None otherwise
    pub fn get_isize<S: ToString>(&self, key: &S) -> Option<isize> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => val.parse::<isize>().ok(),
            None => None
        }
    }

    /// Get the value of a CAPO property as a bool
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the bool value if the property can be represented as a
    /// bool and None otherwise
    pub fn get_bool<S: ToString>(&self, key: &S) -> Option<bool> {
        match self.options.get(&key.to_string().to_uppercase()) {
            Some(val) => match val.to_lowercase().as_str() {
                "yes" | "true" => Some(true),
                "no" | "false" => Some(false),
                _ => None,
            },
            None => None
        }
    }

    /// Get all the options in the CAPO config
    ///
    /// # Return
    /// A HashMap<String, String> with all the option keys and their values
    pub fn get_options(&self) -> HashMap<String, String> {
        self.options.clone()
    }

    /// Get the location of a CAPO property as a String
    ///
    /// # Arguments
    /// * `key` - A ToString with the property key value
    ///
    /// # Return
    /// An Option with the location as a String or None if the property
    /// isn't found in any location
    pub fn get_location<S: ToString>(&self, key: &S) -> Option<String> {
        match self.locations.get(&key.to_string().to_uppercase()) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Get all the locations in the CAPO config
    ///
    /// # Return
    /// A HashMap<String, String> with all the properties as keys and locations
    /// as values
    pub fn get_locations(&self) -> HashMap<String, String> {
        self.locations.clone()
    }
}
