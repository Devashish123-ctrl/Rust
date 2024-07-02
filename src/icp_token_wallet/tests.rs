#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let token = Token::new("IRCRC2".to_string(), 100);
        assert_eq!(token.balance_of("creator".to_string()), 100);
        assert_eq!(token.transfer("creator".to_string(), "user1".to_string(), 50), Ok(()));
        assert_eq!(token.balance_of("creator".to_string()), 50);
        assert_eq!(token.balance_of("user1".to_string()), 50);
    }

    #[test]
    fn test_insufficient_balance() {
        let token = Token::new("IRCRC2".to_string(), 100);
        assert_eq!(token.transfer("creator".to_string(), "user1".to_string(), 150), Err("Insufficient balance".to_string()));
    }
}
