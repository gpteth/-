// Import required dependencies
use ink_lang as ink;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

// Define the DecentraJobs smart contract
#[ink::contract]
mod decentra_jobs {
    use ink_prelude::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    // Define the Job structure
    #[derive(scale::Encode, scale::Decode, scale::Clone)]
    pub struct Job {
        title: String,
        description: String,
        salary: u64,
        owner: AccountId,
        applicants: Vec<AccountId>,
        accepted_applicant: Option<AccountId>,
        is_completed: bool,
    }

    // Define the DecentraJobs contract
    #[ink(storage)]
    pub struct DecentraJobs {
        jobs: StorageHashMap<u64, Job>,
        job_id_counter: Lazy<u64>,
    }

    impl DecentraJobs {
        // Initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                jobs: StorageHashMap::new(),
                job_id_counter: Lazy::new(0),
            }
        }

        // Create a new job listing
        #[ink(message)]
        pub fn post_job(&mut self, title: String, description: String, salary: u64) {
            let job_id = self.next_job_id();
            let job = Job {
                title,
                description,
                salary,
                owner: self.env().caller(),
                applicants: Vec::new(),
                accepted_applicant: None,
                is_completed: false,
            };
            self.jobs.insert(job_id, job);
        }

        // Get a job listing by ID
        #[ink(message)]
        pub fn get_job(&self, job_id: u64) -> Option<Job> {
            self.jobs.get(&job_id).cloned()
        }

        // Apply for a job listing
        #[ink(message)]
        pub fn apply_for_job(&mut self, job_id: u64) {
            let job_opt = self.jobs.get_mut(&job_id);
            if let Some(job) = job_opt {
                let applicant = self.env().caller();
                if !job.applicants.contains(&applicant) {
                    job.applicants.push(applicant);
                }
            }
        }

        // Accept an applicant for a job listing
        #[ink(message)]
        pub fn accept_applicant(&mut self, job_id: u64, applicant: AccountId) -> Result<(), &'static str> {
            let job_opt = self.jobs.get_mut(&job_id);
            if let Some(job) = job_opt {
                if job.owner != self.env().caller() {
                    return Err("Only the job owner can accept applicants");
                }
                if !job.applicants.contains(&applicant) {
                    return Err("Applicant not found");
                }
                job.accepted_applicant = Some(applicant);
                Ok(())
            } else {
                Err("Job not found")
            }
        }

        // Complete a job and make a payment
        #[ink(message)]
        pub fn complete_job(&mut self, job_id: u64) -> Result<(), &'static str> {
            let job_opt = self.jobs.get_mut(&job_id);
            if let Some(job) = job_opt {
                if job.owner != self.env().caller() {
                    return Err("Only the job owner can complete the job");
                }
                if job.accepted_applicant.is_none() {
                    return Err("No applicant accepted for the job");
                }
                job.is_completed = true;
                // Implement payment logic here
                // You may transfer funds to the accepted applicant
                Ok(())
            } else {
                Err("Job not found")
            }
        }

        // Internal function to generate the next job ID
        fn next_job_id(&mut self) -> u64 {
            let job_id = *self.job_id_counter;
            *self.job_id_counter += 1;
            job_id
        }
    }
}
