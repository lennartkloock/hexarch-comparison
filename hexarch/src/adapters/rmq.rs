use crate::domain::User;
use crate::ports::UserNotifier;

/// Rabbit MQ adapter
pub struct RabbitMq;

impl RabbitMq {
    pub fn new() -> Self {
        println!("rmq connection logic");
        Self
    }
}

#[async_trait::async_trait]
impl UserNotifier for RabbitMq {
    async fn user_created(&self, user: &User) {
        println!("notifying mq with {user:?}");
        // ...
    }
}
