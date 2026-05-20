pub fn get_message() -> &'static str {
    "titi"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(), "titi");
    }
}