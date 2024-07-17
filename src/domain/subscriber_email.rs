//! 구독자의 이메일 주소를 처리한다.

use validator::ValidateEmail;
#[derive(Debug)]
pub struct SubscriberEmail(String);

pub type SubscriberParseError = String;
pub type SubscriberParseResult = Result<SubscriberEmail, SubscriberParseError>;

impl SubscriberEmail {
    fn parse(s: String) -> SubscriberParseResult {
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = SubscriberParseError;
    fn try_from(s: String) -> SubscriberParseResult {
        SubscriberEmail::parse(s)
    }
}

pub trait SubscribeEmailExt {
    fn try_into_subscriber_email(self) -> SubscriberParseResult;
}

impl SubscribeEmailExt for String {
    fn try_into_subscriber_email(self) -> SubscriberParseResult {
        SubscriberEmail::parse(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(email.try_into_subscriber_email());
    }

    #[test]
    fn email_missing_at_symbol_is_rejectd() {
        let email = "ursuladomain.com".to_string();
        assert_err!(email.try_into_subscriber_email());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(email.try_into_subscriber_email());
    }

    // `quickcheck`에는 `Clone`과 `Debug`가 필요하다.
    #[derive(Clone, Debug)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        valid_email.0.try_into_subscriber_email().is_ok()
    }
}
