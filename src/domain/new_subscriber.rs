//! 새로운 구독자
use crate::domain::SubscriberName;


pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
