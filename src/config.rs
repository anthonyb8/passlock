// use anyhow::anyhow;
use arboard::Clipboard;
use std::{
    collections::HashMap,
    env::home_dir,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    thread,
    time::Duration,
};

use crate::{
    entry::{hash_sha256, Credential, CredentialStore},
    Error, Result,
};

pub struct Config {
    pub file: PathBuf,
    pub clipboard: Clipboard,
}

impl Config {
    pub fn new() -> Result<Config> {
        let clipboard = Clipboard::new().expect("Clipboard not available");
        let file = home_dir()
            .ok_or(Error::HomeNotFoundError)?
            .join(".passlock.json");

        Config::check_file(&file)?;

        Ok(Config { file, clipboard })
    }

    fn check_file(path: &PathBuf) -> std::io::Result<()> {
        if !path.exists() {
            let mut file = OpenOptions::new().write(true).create(true).open(path)?;
            file.write_all(b"{}")?; // initialize with empty JSON object
        }
        Ok(())
    }

    fn copy_to_clipboard(&mut self, value: String) -> Result<()> {
        self.clipboard.set_text(value)?;
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            let mut clipboard = Clipboard::new().expect("Failed to access clipboard to clear");
            clipboard
                .set_text("".to_string())
                .expect("Error clearing clipboard");
        });
        Ok(())
    }

    fn read_file(&mut self) -> Result<CredentialStore> {
        let contents = fs::read_to_string(&self.file)?;
        let store: CredentialStore = match serde_json::from_str(&contents) {
            Ok(c) => c,
            Err(_) => HashMap::new(),
        };

        println!("Read file: {:?}", store);

        Ok(store)
    }

    fn write_file(&mut self, credentials: &CredentialStore) -> Result<()> {
        // Serialize your data to a JSON string (or any format you use)
        let serialized = serde_json::to_string(credentials).expect("Failed to serialize");

        println!("Serialized : {:?}", serialized);

        // Create (or overwrite) the file
        let mut file = fs::File::create(&self.file)?;

        // Write the serialized string as bytes
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    // fn check_contents(
    //     &self,
    //     contents: &CredentialStore,
    //     entry: &str,
    //     username: &str,
    // ) -> Result<Option<Credential>> {
    //     if let Some(cred_vec) = contents.get(entry) {
    //         if let Some(credential) = cred_vec.iter().find(|c| c.username == username) {
    //             return Ok(Some(credential.clone()));
    //         }
    //     }
    //     return Ok(None);
    // }

    pub fn get(&mut self, entry: &str, username: &str, master: &str) -> Result<()> {
        let entry_hash = hash_sha256(entry);
        let username_hash = hash_sha256(username);
        let contents = self.read_file()?;

        if let Some(cred_vec) = contents.get(&entry_hash) {
            if let Some(credential) = cred_vec.iter().find(|cred| cred.username == username_hash) {
                self.copy_to_clipboard(credential.password_string(master)?)?;
            }
        }

        Ok(())
    }

    pub fn upsert(&mut self, entry: &str, new_cred: Credential) -> Result<()> {
        let entry_hash = hash_sha256(&entry);
        let mut contents = self.read_file()?; // mutable contents

        // Get or insert empty vector for this entry
        let cred_vec = contents.entry(entry_hash).or_insert_with(Vec::new);

        // Try find existing credential by username
        if let Some(existing) = cred_vec
            .iter_mut()
            .find(|c| c.username == new_cred.username)
        {
            // Update existing credential in place
            *existing = new_cred;
        } else {
            // Insert new credential
            cred_vec.push(new_cred);
        }
        println!("Creds: {:?}", &contents);

        // Write updated contents back to file
        self.write_file(&contents)?;

        Ok(())
    }

    pub fn delete(&mut self, entry: &str, username: &str) -> Result<()> {
        let entry_hash = hash_sha256(entry);
        let username_hash = hash_sha256(username);
        let mut contents = self.read_file()?;

        if let Some(cred_vec) = contents.get_mut(&entry_hash) {
            // Remove credentials matching username
            cred_vec.retain(|c| c.username != username_hash);

            // Remove entry key if vector empty
            if cred_vec.is_empty() {
                contents.remove(&entry_hash);
            }
        }

        // Write updated contents back to file
        self.write_file(&contents)?;

        Ok(())
    }
}
