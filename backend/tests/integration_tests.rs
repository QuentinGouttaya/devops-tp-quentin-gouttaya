#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_task_creation() {
        let title = "Test task".to_string();
        assert!(!title.is_empty());
    }
}
