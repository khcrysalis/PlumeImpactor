use keyring::{Entry, Error};

const KEYRING_SERVICE: &str = env!("CARGO_PKG_NAME");
const KEYRING_EMAIL: &str = "Apple ID Email";
const KEYRING_PASS: &str = "Apple ID Password";

pub struct AccountCredentials;

impl AccountCredentials {
	pub fn set_credentials(&self, email: String, password: String) -> Result<(), Error> {
		let entry_email = Entry::new(KEYRING_SERVICE, KEYRING_EMAIL)?;
		let entry_pass = Entry::new(KEYRING_SERVICE, KEYRING_PASS)?;
		entry_email.set_secret(email.as_bytes())?;
		entry_pass.set_secret(password.as_bytes())?;
		Ok(())
	}

	pub fn get_email(&self) -> Result<String, Error> {
		let entry = Entry::new(KEYRING_SERVICE, KEYRING_EMAIL)?;
		entry.get_password()
	}

	pub fn get_password(&self) -> Result<String, Error> {
		let entry = Entry::new(KEYRING_SERVICE, KEYRING_PASS)?;
		entry.get_password()
	}

	pub fn delete_password(&self) -> Result<(), Error> {
		let entry_email = Entry::new(KEYRING_SERVICE, KEYRING_EMAIL)?;
		let entry_pass = Entry::new(KEYRING_SERVICE, KEYRING_PASS)?;
		entry_email.delete_credential()?;
		entry_pass.delete_credential()?;
		Ok(())
	}

	pub fn credentials_exist(&self, email: String, password: String) -> Result<bool, Error> {
		let stored_email = self.get_email().unwrap_or_default();
		let stored_password = self.get_password().unwrap_or_default();
		Ok(stored_email == email && stored_password == password)
	}
}
