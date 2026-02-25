use std::sync::Arc;

use crate::{
    adapters::{postgres::Postgres, rand::RandAccountAddressFactory, rmq::RabbitMq},
    domain::{EmailAddress, UserName},
    service::{DefaultUserService, UserService},
};

mod adapters;
mod domain;
mod ports;
mod service;

#[tokio::main]
async fn main() {
    // Adapter creation
    let postgres = Arc::new(Postgres::new());
    let rmq = RabbitMq::new();
    let rand = RandAccountAddressFactory::new();

    // Service creation
    let service = DefaultUserService::new(postgres.clone(), postgres, Arc::new(rand), Arc::new(rmq));

    // POST /user
    handler(Arc::new(service), "UserName", "user@example.com").await;
}

async fn handler(service: Arc<dyn UserService>, username: &str, email_address: &str) {
    println!("got http call to POST /user");
    let username = UserName::parse(username).expect("error handling");
    let email_address = EmailAddress::parse(email_address).expect("error handling");
    service
        .create_user(username, email_address)
        .await
        .expect("error handling");
}
