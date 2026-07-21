const SERVICE_NAME: &str = "com.ku-terminal.ssh";

pub fn save_ssh_password(credential_id: &str, password: &str) -> anyhow::Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, credential_id)?;
    entry.set_password(password)?;
    Ok(())
}

pub fn load_ssh_password(credential_id: &str) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE_NAME, credential_id).ok()?;
    entry.get_password().ok()
}

pub fn delete_ssh_password(credential_id: &str) -> anyhow::Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, credential_id)?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(_) => Ok(()),
    }
}
