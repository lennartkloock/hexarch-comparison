//! Contains business logic that can be tested idependently

use crate::model::{Account, AccountAddress, User};

/// Idependently testable
pub fn check_user(user_by_email: Option<User>, user_by_name: Option<User>) -> anyhow::Result<()> {
    // Check if the email address is already registered
    if user_by_email.is_some() {
        anyhow::bail!("this email address is already registered");
    }

    // Check if the username is already registered
    if user_by_name.is_some() {
        anyhow::bail!("this username is already registered");
    }

    Ok(())
}

/// Idependently testable
pub fn new_account(user_id: uuid::Uuid) -> Account {
    Account::new(AccountAddress::generate_new(rand::rng()), user_id, 10)
}

#[cfg(test)]
mod tests {
    // ...
}
