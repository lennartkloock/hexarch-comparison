use crate::{
    domain::{Account, EmailAddress, User, UserName},
    ports::{AccountRepository, UserRepository},
};

/// Postgres adapter
pub struct Postgres;

impl Postgres {
    pub fn new() -> Self {
        println!("postgres connection logic");
        Self
    }
}

#[async_trait::async_trait]
impl UserRepository for Postgres {
    async fn create_user(&self, username: UserName, email_address: EmailAddress) -> anyhow::Result<User> {
        println!("inserting user with name: {username:?} and email: {email_address:?}");
        // ...
        Ok(User {
            id: uuid::Uuid::new_v4(),
            username,
            email_address,
        })
    }

    async fn get_user_by_email(&self, _email_address: &EmailAddress) -> anyhow::Result<Option<User>> {
        // ...
        Ok(None)
    }

    async fn get_user_by_username(&self, _username: &UserName) -> anyhow::Result<Option<User>> {
        // ...
        Ok(None)
    }
}

#[async_trait::async_trait]
impl AccountRepository for Postgres {
    async fn create_account(&self, account: Account) -> anyhow::Result<()> {
        println!("inserting account {account:?}");
        // ...
        Ok(())
    }
}
