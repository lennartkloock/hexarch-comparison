//! Domain models

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: UserName,
    pub email_address: EmailAddress,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn parse(raw: &str) -> anyhow::Result<Self> {
        let trimmed = raw.trim();
        anyhow::ensure!(!trimmed.is_empty(), "username cannot be empty");

        Ok(Self(trimmed.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
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
