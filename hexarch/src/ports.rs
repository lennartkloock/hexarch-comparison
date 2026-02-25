//! Ports

use crate::domain::{Account, AccountAddress, EmailAddress, User, UserName};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, username: UserName, email_address: EmailAddress) -> anyhow::Result<User>;
    async fn get_user_by_email(&self, email_address: &EmailAddress) -> anyhow::Result<Option<User>>;
    async fn get_user_by_username(&self, username: &UserName) -> anyhow::Result<Option<User>>;
}

#[async_trait::async_trait]
pub trait AccountRepository: Send + Sync + 'static {
    async fn create_account(&self, account: Account) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait AccountAddressFactory: Send + Sync + 'static {
    async fn init(&self) -> AccountAddress;
}

#[async_trait::async_trait]
pub trait UserNotifier: Send + Sync + 'static {
    async fn user_created(&self, user: &User);
}

#[cfg(test)]
mod tests {
    // ...
}
