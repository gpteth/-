use ink_lang as ink;

#[ink::contract]
mod legal_link {
    #[ink(storage)]
    pub struct LegalLink {
        owner: AccountId,
        contracts: ink_storage::collections::HashMap<AccountId, String>,
    }

    impl LegalLink {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                contracts: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_contract(&mut self, title: String, content: String) {
            let caller = self.env().caller();
            self.contracts.insert(caller, content.clone());
            self.env()
                .emit_event(ContractCreated {
                    owner: caller,
                    title,
                    content,
                });
        }

        #[ink(message)]
        pub fn get_contract(&self, account_id: AccountId) -> Option<&String> {
            self.contracts.get(&account_id)
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            let caller = self.env().caller();
            // Ensure only the current owner can transfer ownership
            assert_eq!(self.owner, caller, "Only the owner can transfer ownership");
            self.owner = new_owner;
            self.env()
                .emit_event(OwnershipTransferred {
                    previous_owner: caller,
                    new_owner,
                });
        }

        #[ink(message)]
        pub fn delete_contract(&mut self) {
            let caller = self.env().caller();
            // Ensure only the contract owner can delete it
            assert_eq!(self.owner, caller, "Only the owner can delete the contract");
            self.contracts.take(&caller);
            self.env().emit_event(ContractDeleted { owner: caller });
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    #[ink(event)]
    pub struct ContractCreated {
        owner: AccountId,
        title: String,
        content: String,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        previous_owner: AccountId,
        new_owner: AccountId,
    }

    #[ink(event)]
    pub struct ContractDeleted {
        owner: AccountId,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_contract_works() {
            let mut legal_link = LegalLink::new();
            let caller = AccountId::from([0x1; 32]);
            ink_env::test::set_caller(caller);
            legal_link.create_contract(String::from("Sample Contract"), String::from("Contract Content"));
            let contract = legal_link.get_contract(caller);
            assert_eq!(contract, Some(&String::from("Contract Content")));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut legal_link = LegalLink::new();
            let caller = AccountId::from([0x1; 32]);
            ink_env::test::set_caller(caller);
            legal_link.transfer_ownership(AccountId::from([0x2; 32]));
            let new_owner = legal_link.get_owner();
            assert_eq!(new_owner, AccountId::from([0x2; 32]));
        }

        #[ink::test]
        fn delete_contract_works() {
            let mut legal_link = LegalLink::new();
            let caller = AccountId::from([0x1; 32]);
            ink_env::test::set_caller(caller);
            legal_link.create_contract(String::from("Sample Contract"), String::from("Contract Content"));
            legal_link.delete_contract();
            let contract = legal_link.get_contract(caller);
            assert_eq!(contract, None);
        }
    }
}
