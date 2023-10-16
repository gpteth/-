// Import necessary libraries
use ink_lang as ink;
use ink_prelude::vec::Vec;

// Define the SkillDAO smart contract
#[ink::contract]
mod skill_dao {
    use ink_prelude::string::String;

    // Define the project structure
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Project {
        title: String,
        description: String,
        owner: AccountId,
    }

    // Define the SkillDAO contract
    #[ink(storage)]
    pub struct SkillDAO {
        owner: AccountId,
        projects: Vec<Project>,
    }

    impl SkillDAO {
        // Constructor to initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                projects: Vec::new(),
            }
        }

        // Function to create a new project
        #[ink(message)]
        pub fn create_project(&mut self, title: String, description: String) {
            let caller = Self::env().caller();
            let project = Project {
                title,
                description,
                owner: caller,
            };
            self.projects.push(project);
        }

        // Function to get the list of projects
        #[ink(message)]
        pub fn get_projects(&self) -> Vec<Project> {
            self.projects.clone()
        }

        // Function to get the owner of the contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    // Event to log project creation
    #[ink(event)]
    pub struct ProjectCreated {
        #[ink(topic)]
        title: String,
        #[ink(topic)]
        owner: AccountId,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl SkillDAO {
        // Event emitter for project creation
        fn emit_project_created_event(&self, title: String, owner: AccountId) {
            self.env().emit_event(ProjectCreated { title, owner });
        }
    }

    // Tests for the SkillDAO contract
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_project_works() {
            let mut skill_dao = SkillDAO::new();
            let title = String::from("Project 1");
            let description = String::from("Description of Project 1");

            ink_env::test::execute_contract_call(|_| {
                skill_dao.create_project(title.clone(), description.clone());
            });

            let projects = skill_dao.get_projects();
            let project = &projects[0];
            assert_eq!(project.title, title);
            assert_eq!(project.description, description);
        }
    }
}
