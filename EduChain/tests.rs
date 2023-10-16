#[cfg(test)]
mod tests {
    use super::*;
    use ink_env::test::ink_env;
    use ink_lang as ink;

    #[ink::test]
    fn test_add_certification() {
        // Initialize a test context
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut contract = EduChain::new();

        // Add a certification
        contract.add_certification(accounts.alice, "Cert 1".to_string(), 12345, accounts.bob);

        // Retrieve the student's certifications and assert
        let certifications = contract.get_student_certifications(accounts.alice);
        assert_eq!(certifications.len(), 1);
    }

    // Add more test functions for other contract methods
}
