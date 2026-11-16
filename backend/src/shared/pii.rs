//! Personal data kept out of logs.

/// An email address as it may appear in a log line: the first character of
/// the local part and the domain, enough to tell a provider outage apart
/// from a single bad address without writing the address itself.
pub fn masked_email(address: &str) -> String {
    match address.rsplit_once('@') {
        Some((local, domain)) => {
            let first = local.chars().next().map(String::from).unwrap_or_default();
            format!("{first}***@{domain}")
        }
        None => "***".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_first_character_and_the_domain_only() {
        assert_eq!(masked_email("ada.lovelace@example.cm"), "a***@example.cm");
        assert_eq!(masked_email("@example.cm"), "***@example.cm");
    }

    #[test]
    fn hides_a_value_that_is_not_an_address() {
        assert_eq!(masked_email("not-an-email"), "***");
    }
}
