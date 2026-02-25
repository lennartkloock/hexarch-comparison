use std::sync::Arc;

use crate::{
    domain::{Account, EmailAddress, User, UserName},
    ports::{AccountAddressFactory, AccountRepository, UserNotifier, UserRepository},
};

/// Service
#[async_trait::async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user(
        &self,
        name: UserName,
        email_address: EmailAddress,
    ) -> anyhow::Result<User>;
}

/// Service impl
pub struct DefaultUserService {
    user_repo: Arc<dyn UserRepository>,
    account_repo: Arc<dyn AccountRepository>,
    account_address_factory: Arc<dyn AccountAddressFactory>,
    notifier: Arc<dyn UserNotifier>,
}

impl DefaultUserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        account_repo: Arc<dyn AccountRepository>,
        account_address_factory: Arc<dyn AccountAddressFactory>,
        notifier: Arc<dyn UserNotifier>,
    ) -> Self {
        Self {
            user_repo,
            account_repo,
            account_address_factory,
            notifier,
        }
    }
}

#[async_trait::async_trait]
impl UserService for DefaultUserService {
    async fn create_user(
        &self,
        username: UserName,
        email_address: EmailAddress,
    ) -> anyhow::Result<User> {
        // Check if the email address is already registered
        if self
            .user_repo
            .get_user_by_email(&email_address)
            .await?
            .is_some()
        {
            anyhow::bail!("this email address is already registered");
        }

        // Check if the username is already registered
        if self
            .user_repo
            .get_user_by_username(&username)
            .await?
            .is_some()
        {
            anyhow::bail!("this username is already registered");
        }

        // Problem: We can't use a transcation here
        // because Hexarch doesn't allow us to know that transactions exist at this point

        // Add the user to the repo
        let user = self.user_repo.create_user(username, email_address).await?;

        // Generate a new account address
        let address = self.account_address_factory.init().await;

        // Add the account to the repo
        let account = Account::new(address, user.id, 10);
        self.account_repo.create_account(account).await?;

        // Notify that the user was created
        self.notifier.user_created(&user).await;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    // ...
}
