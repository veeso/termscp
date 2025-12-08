use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::config::key_bindings::KeyBindings;
use crate::config::serialization::{SerializerError, SerializerErrorKind, deserialize, serialize};

#[derive(Default)]
pub struct KeyBindingsClient {
    config: KeyBindings,
    config_path: PathBuf,
}

impl KeyBindingsClient {
    pub fn new(path: PathBuf) -> Result<Self, SerializerError> {
        let default_key_bindings = KeyBindings::default();
        // Create provider
        let mut provider = Self {
            config: default_key_bindings,
            config_path: path.to_path_buf(),
        };

        // If Config file doesn't exist, create it
        if !path.exists() {
            if let Err(err) = provider.save() {
                error!("Couldn't write key_bindings file: {}", err);
                return Err(err);
            }
            debug!("Theme file didn't exist; created file");
        } else {
            // otherwise Load configuration from file
            if let Err(err) = provider.load() {
                error!("Couldn't read key_bindings file: {}", err);
                return Err(err);
            }
            debug!("Read key_bindings file");
        }
        Ok(provider)
    }

    /// Load key_bindings from file
    pub fn load(&mut self) -> Result<(), SerializerError> {
        // Open key_bindings file for read
        debug!("Loading key_bindings from file...");
        match OpenOptions::new()
            .read(true)
            .open(self.config_path.as_path())
        {
            Ok(reader) => {
                // Deserialize
                match deserialize(Box::new(reader)) {
                    Ok(config) => {
                        self.config = config;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                error!("Failed to read key_bindings: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }

    /// Save key_bindings to file
    pub fn save(&self) -> Result<(), SerializerError> {
        // Open file
        debug!("Writing key_bindings");
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.config_path.as_path())
        {
            Ok(writer) => serialize(&self.config, Box::new(writer)),
            Err(err) => {
                error!("Failed to write key_bindings: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }
}
