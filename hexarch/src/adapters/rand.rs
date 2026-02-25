use rand::RngExt;

use crate::{domain::AccountAddress, ports::AccountAddressFactory};

pub struct RandAccountAddressFactory;

impl RandAccountAddressFactory {
    pub fn new() -> Self {
        println!("init rand account address factory");
        Self
    }
}

#[async_trait::async_trait]
impl AccountAddressFactory for RandAccountAddressFactory {
    async fn init(&self) -> AccountAddress {
        println!("generating new account address");
        AccountAddress(rand::rng().random())
    }
}
