#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Resume {
    pub owner: Address,
    pub name: String,
    pub headline: String,
    pub skills: Vec<String>,
    pub experience: Vec<String>,
    pub education: Vec<String>,
    pub timestamp: u64,
}

#[contracttype]
pub enum ResumeKey {
    ResumeByOwner(Address),
}

#[contract]
pub struct BlockchainResume;

#[contractimpl]
impl BlockchainResume {
    pub fn create_resume(
        env: Env,
        owner: Address,
        name: String,
        headline: String,
        skills: Vec<String>,
        experience: Vec<String>,
        education: Vec<String>,
    ) {
        let resume = Resume {
            owner: owner.clone(),
            name,
            headline,
            skills,
            experience,
            education,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&ResumeKey::ResumeByOwner(owner.clone()), &resume);
        log!(&env, "Resume created for: {}", owner);
    }

    pub fn get_resume(env: Env, owner: Address) -> Resume {
        env.storage().instance().get(&ResumeKey::ResumeByOwner(owner)).unwrap()
    }
}
