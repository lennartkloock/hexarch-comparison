use crate::{
    model::{EmailAddress, UserName},
    postgres::Postgres,
    rmq::RabbitMq,
};

mod model;
mod postgres;
mod rmq;
mod signup_logic;

#[tokio::main]
async fn main() {
    let db = Postgres::new();
    let rmq = RabbitMq::new();

    // POST /user
    handler(&db, &rmq, "UserName", "user@example.com").await;
}

/// This handler doesn't contain any business logic.
/// All of the business logic was extracted into simple functions that can be tested idependently just like in Hexarch.
async fn handler(db: &Postgres, rmq: &RabbitMq, username: &str, email_address: &str) {
    println!("got http call to POST /user");
    let username = UserName::parse(username).expect("error handling");
    let email_address = EmailAddress::parse(email_address).expect("error handling");

    let tx = db.start_tx();

    // Check if the email address is already registered
    let user_by_email = postgres::get_user_by_email(tx, &email_address)
        .await
        .expect("error handling");
    let user_by_name = postgres::get_user_by_username(tx, &username)
        .await
        .expect("error handling");

    signup_logic::check_user(user_by_email, user_by_name).expect("error handling");

    // Insert the user
    let user = postgres::insert_user(tx, username, email_address)
        .await
        .expect("error handling");

    // Create a new account
    let account = signup_logic::new_account(user.id);
    // Insert the account
    postgres::insert_account(tx, account)
        .await
        .expect("error handling");

    tx.commit().expect("error handling");

    // Notify that the user was created
    rmq::notify_user_created(rmq, &user).await;
}
