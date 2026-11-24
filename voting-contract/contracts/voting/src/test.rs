#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, vec, Env};

fn setup() -> (Env, VotingClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(100);
    let id = env.register(Voting, ());
    let client = VotingClient::new(&env, &id);
    client.init(
        &String::from_str(&env, "Que evento sigue?"),
        &vec![&env, String::from_str(&env, "Rock"), String::from_str(&env, "Jazz")],
        &1000,
    );
    (env, client)
}

#[test]
fn counts_votes() {
    let (env, c) = setup();
    c.vote(&Address::generate(&env), &0);
    c.vote(&Address::generate(&env), &0);
    c.vote(&Address::generate(&env), &1);
    assert_eq!(c.votes(&0), 2);
    assert_eq!(c.votes(&1), 1);
}

#[test]
fn marks_voter() {
    let (env, c) = setup();
    let v = Address::generate(&env);
    assert!(!c.has_voted(&v));
    c.vote(&v, &1);
    assert!(c.has_voted(&v));
}

#[test]
fn rejects_double_vote() {
    let (env, c) = setup();
    let v = Address::generate(&env);
    c.vote(&v, &0);
    assert_eq!(c.try_vote(&v, &1), Err(Ok(Error::AlreadyVoted)));
    assert_eq!(c.votes(&0), 1);
}

#[test]
fn rejects_invalid_option() {
    let (env, c) = setup();
    assert_eq!(c.try_vote(&Address::generate(&env), &5), Err(Ok(Error::InvalidOption)));
}

#[test]
fn rejects_after_deadline() {
    let (env, c) = setup();
    env.ledger().set_timestamp(2000);
    assert_eq!(c.try_vote(&Address::generate(&env), &0), Err(Ok(Error::VotingClosed)));
}

#[test]
fn rejects_double_init_and_few_options() {
    let (env, c) = setup();
    let q = String::from_str(&env, "x");
    assert_eq!(c.try_init(&q, &vec![&env, q.clone(), q.clone()], &5), Err(Ok(Error::AlreadyInitialized)));
    let id2 = env.register(Voting, ());
    let c2 = VotingClient::new(&env, &id2);
    assert_eq!(c2.try_init(&q, &vec![&env, q.clone()], &5), Err(Ok(Error::TooFewOptions)));
}
