//! Models

use rand::RngExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: UserName,
    pub email_address: EmailAddress,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    /// Idependently testable
    pub fn parse(raw: &str) -> anyhow::Result<Self> {
        let trimmed = raw.trim();
        anyhow::ensure!(!trimmed.is_empty(), "username cannot be empty");

        Ok(Self(trimmed.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Idependently testable
    pub fn parse(raw: &str) -> anyhow::Result<Self> {
        // Email address check logic here
        Ok(Self(raw.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub address: AccountAddress,
    pub user_id: uuid::Uuid,
    pub balance: u64,
}

impl Account {
    pub fn new(address: AccountAddress, user_id: uuid::Uuid, balance: u64) -> Self {
        Self {
            address,
            user_id,
            balance,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountAddress(pub u128);

impl AccountAddress {
    /// Idependently testable
    pub fn generate_new(mut rng: impl rand::Rng) -> Self {
        println!("generating new account address");
        Self(rng.random())
    }
}

#[cfg(test)]
mod tests {
    // ...
}
