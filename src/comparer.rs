pub fn are_same(old_block: &str, new_block: &str) -> bool {
    old_block == new_block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different_strings() {
        let s1 = "Hello";
        let s2 = "World";
        assert!(!are_same(s1, s2));
    }

    #[test]
    fn test_same_strings() {
        let s1 = "version 123";
        let s2 = "version 123";
        assert!(are_same(s1, s2));
    }
}
