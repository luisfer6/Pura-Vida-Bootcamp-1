#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    AlreadyVoted = 3,
    InvalidOption = 4,
    VotingClosed = 5,
    TooFewOptions = 6,
}

#[contracttype]
pub enum DataKey {
    Question,
    Options,
    Deadline,
    Tally(u32),
    Voted(Address),
}

#[contract]
pub struct Voting;

#[contractimpl]
impl Voting {
    /// Crea la votacion. `deadline` es un timestamp (segundos) del ledger.
    pub fn init(env: Env, question: String, options: Vec<String>, deadline: u64) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Question) {
            return Err(Error::AlreadyInitialized);
        }
        if options.len() < 2 {
            return Err(Error::TooFewOptions);
        }
        env.storage().instance().set(&DataKey::Question, &question);
        env.storage().instance().set(&DataKey::Options, &options);
        env.storage().instance().set(&DataKey::Deadline, &deadline);
        Ok(())
    }

    /// Un voto por direccion. El votante debe autorizar la llamada.
    pub fn vote(env: Env, voter: Address, option: u32) -> Result<(), Error> {
        voter.require_auth();
        let options: Vec<String> = env.storage().instance().get(&DataKey::Options).ok_or(Error::NotInitialized)?;
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).ok_or(Error::NotInitialized)?;
        if env.ledger().timestamp() > deadline {
            return Err(Error::VotingClosed);
        }
        if option >= options.len() {
            return Err(Error::InvalidOption);
        }
        let voted_key = DataKey::Voted(voter);
        if env.storage().persistent().has(&voted_key) {
            return Err(Error::AlreadyVoted);
        }
        env.storage().persistent().set(&voted_key, &true);
        let tally_key = DataKey::Tally(option);
        let current: u32 = env.storage().persistent().get(&tally_key).unwrap_or(0);
        env.storage().persistent().set(&tally_key, &(current + 1));
        Ok(())
    }

    /// Votos obtenidos por una opcion.
    pub fn votes(env: Env, option: u32) -> u32 {
        env.storage().persistent().get(&DataKey::Tally(option)).unwrap_or(0)
    }

    pub fn has_voted(env: Env, voter: Address) -> bool {
        env.storage().persistent().has(&DataKey::Voted(voter))
    }

    pub fn question(env: Env) -> Result<String, Error> {
        env.storage().instance().get(&DataKey::Question).ok_or(Error::NotInitialized)
    }

    pub fn options(env: Env) -> Result<Vec<String>, Error> {
        env.storage().instance().get(&DataKey::Options).ok_or(Error::NotInitialized)
    }
}

#[cfg(test)]
mod test;
