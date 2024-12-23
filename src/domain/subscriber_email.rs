use validator::ValidateEmail;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if !s.validate_email() {
            return Err(format!("{} is not a valid subscriber email.", s));
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);
    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // We get `quickcheck` and `fake` to interoperate via the `rand` crate.
            // We create a seed from a random `u64` generated by `quickcheck`'s `Gen`,
            // which is in turn determined by `quickcheck`'s own seed.
            // We then use this seed to create a new random number generator, which
            // we pass to `fake`'s `SafeEmail`.
            // This way we can generate a random valid email every time `quickcheck`
            // calls `arbitrary`, and the generation process is deterministic for
            // a given seed.
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    // #[test]
    // fn valid_emails_are_parsed_successfully() {
    //     let email = SafeEmail().fake();
    //     let email = SubscriberEmail::parse(email).unwrap();
    //     assert_eq!(email.as_ref(), email.as_ref());
    // }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
