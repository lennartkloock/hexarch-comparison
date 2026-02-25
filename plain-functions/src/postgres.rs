use crate::model::{Account, EmailAddress, User, UserName};

/// Postgres client
pub struct Postgres;

impl Postgres {
    pub fn new() -> Self {
        println!("postgres connection logic");
        Self
    }

    pub fn start_tx(&self) -> &Self {
        self
    }

    pub fn commit(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub async fn insert_user(_conn: &Postgres, username: UserName, email_address: EmailAddress) -> anyhow::Result<User> {
    println!("inserting user with name: {username:?} and email: {email_address:?}");
    // ...
    Ok(User {
        id: uuid::Uuid::new_v4(),
        username,
        email_address,
    })
}

pub async fn get_user_by_email(_conn: &Postgres, _email_address: &EmailAddress) -> anyhow::Result<Option<User>> {
    // ...
    Ok(None)
}

pub async fn get_user_by_username(_conn: &Postgres, _username: &UserName) -> anyhow::Result<Option<User>> {
    // ...
    Ok(None)
}

pub async fn insert_account(_conn: &Postgres, account: Account) -> anyhow::Result<()> {
    println!("inserting account {account:?}");
    // ...
    Ok(())
}
