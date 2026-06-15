#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
};

#[test]
fn test_campus_pay() {
    let env = Env::default();

    let contract_id = env.register(CampusPayContract, ());
    let client = CampusPayContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let merchant = Address::generate(&env);

    env.mock_all_auths();

    client.register_merchant(&merchant);

    client.mint(&student, &1000);

    assert_eq!(client.balance(&student), 1000);

    client.pay(
        &student,
        &merchant,
        &300,
    );

    assert_eq!(client.balance(&student), 700);
    assert_eq!(client.balance(&merchant), 300);
}