use crate::model::User;

/// Rabbit MQ client
pub struct RabbitMq;

impl RabbitMq {
    pub fn new() -> Self {
        println!("rmq connection logic");
        Self
    }
}

pub async fn notify_user_created(_conn: &RabbitMq, user: &User) {
    println!("notifying mq with {user:?}");
    // ...
}
