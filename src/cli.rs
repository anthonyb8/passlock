use crate::{entry::Credential, Config, Result};
use clap::Args;
use inquire::Text;

fn prompt(prompt_str: &str) -> Result<String> {
    let response = Text::new(prompt_str).prompt()?;
    Ok(response)
}

fn password_prompt() -> Result<String> {
    let password = rpassword::prompt_password("Master Password: ")?;
    Ok(password)
}

#[derive(Debug, Args)]
pub struct CreateArgs {}

impl CreateArgs {
    pub fn process_command(&self, config: &mut Config) -> Result<()> {
        let entry = prompt("Entry: ")?.to_lowercase();
        let username = prompt("Username: ")?;
        let password = prompt("Password: ")?;
        let master = password_prompt()?;

        config.upsert(
            entry.trim(),
            Credential::new(username.trim(), &password, &master)?,
        )?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct ReadArgs {}

impl ReadArgs {
    pub fn process_command(&self, config: &mut Config) -> Result<()> {
        let entry = prompt("Entry: ")?.to_lowercase();
        let username = prompt("Username: ")?;
        let master = password_prompt()?;

        config.get(entry.trim(), username.trim(), &master)?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct UpdateArgs {}

impl UpdateArgs {
    pub fn process_command(&self, config: &mut Config) -> Result<()> {
        let entry = prompt("Entry: ")?.to_lowercase();
        let username = prompt("Username: ")?;
        let password = prompt("Password: ")?;
        let master = password_prompt()?;

        config.upsert(
            entry.trim(),
            Credential::new(username.trim(), &password, &master)?,
        )?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct DeleteArgs {}

impl DeleteArgs {
    pub fn process_command(&self, config: &mut Config) -> Result<()> {
        let entry = prompt("Entry: ")?.to_lowercase();
        let username = prompt("Username: ")?;

        config.delete(entry.trim(), username.trim())?;

        Ok(())
    }
}
