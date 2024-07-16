use crate::domain::SubscriberName;

/// 새로운 구독자의 정보
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
