use std::collections::HashMap;

use candid::Principal;
use ic_cdk::{export_candid, query, update};

thread_local! {
    static BALANCES: std::cell::RefCell<HashMap<Principal, i64>> = std::cell::RefCell::new(HashMap::new());
}

#[update]
fn add_expense(payer: Principal, amount: i64, participants: Vec<Principal>) {
    let caller = ic_cdk::caller();
    ic_cdk::println!("add_expense called by: {:?}", caller);
    assert_eq!(payer, caller, "You are not the actual payer");

    let share = amount / participants.len() as i64;

    ic_cdk::println!(
        "Payer: {:?}, Amount: {}, Participants: {:?}, Each share: {}",
        payer,
        amount,
        participants,
        share
    );

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        for participant in participants.iter() {
            *balances.entry(*participant).or_insert(0) -= share;
            ic_cdk::println!("Deducted {} from {:?}", share, participant);
        }

        *balances.entry(payer).or_insert(0) += amount;
        ic_cdk::println!("Credited {} to payer {:?}", amount, payer);
    });
}

#[update]
fn split_expense(payer: Principal, amount: i64, participants: Vec<Principal>) {
    let caller = ic_cdk::caller();
    ic_cdk::println!("split_expense called by: {:?}", caller);
    assert_eq!(payer, caller, "Only payer can call this");

    let share = amount / participants.len() as i64;

    ic_cdk::println!(
        "Payer: {:?}, Amount: {}, Participants: {:?}, Each share: {}",
        payer,
        amount,
        participants,
        share
    );

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        for participant in participants.iter() {
            *balances.entry(*participant).or_insert(0) -= share;
            ic_cdk::println!("Updated {:?}'s balance by -{}", participant, share);
        }

        *balances.entry(payer).or_insert(0) += amount;
        ic_cdk::println!("Updated payer {:?}'s balance by +{}", payer, amount);
    });
}

#[query]
fn get_net_balance_with_user(user: Principal) -> i64 {
    let caller = ic_cdk::caller();
    ic_cdk::println!(
        "Caller {:?} is querying balance with user {:?}",
        caller,
        user
    );

    BALANCES.with(|balances| {
        let balances = balances.borrow();
        let balance = *balances.get(&user).unwrap_or(&0);

        ic_cdk::println!("Balance for user {:?}: {}", user, balance);
        balance
    })
}

#[query]
fn get_total_net_balance() -> i64 {
    let caller = ic_cdk::caller();
    ic_cdk::println!("Fetching total net balance for caller: {:?}", caller);

    BALANCES.with(|balances| {
        let balances = balances.borrow();
        let balance = *balances.get(&caller).unwrap_or(&0);

        ic_cdk::println!("Caller balance: {}", balance);
        balance
    })
}

#[update]
fn settle_balance_with_user(user: Principal) {
    let caller = ic_cdk::caller();
    ic_cdk::println!(
        "Caller: {:?} is attempting to settle with user: {:?}",
        caller,
        user
    );

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        let caller_balance = balances.get(&caller).unwrap_or(&0).clone();
        let user_balance = balances.get(&user).unwrap_or(&0).clone();

        ic_cdk::println!(
            "Current balances -> Caller: {}, User: {}",
            caller_balance,
            user_balance
        );

        if caller_balance < 0 && user_balance > 0 {
            let settle_amount = caller_balance.abs().min(user_balance);

            ic_cdk::println!("Settling amount: {}", settle_amount);

            *balances.entry(caller).or_insert(0) += settle_amount;
            *balances.entry(user).or_insert(0) -= settle_amount;

            let updated_caller = balances.get(&caller).unwrap_or(&0);
            let updated_user = balances.get(&user).unwrap_or(&0);
            ic_cdk::println!(
                "Updated balances -> Caller: {}, User: {}",
                updated_caller,
                updated_user
            );
        } else {
            ic_cdk::println!(
                "No settlement performed. Either caller doesn't owe or user isn't owed."
            );
        }
    });
}

#[query]
fn get_all_user_balances() -> Vec<(Principal, i64)> {
    BALANCES.with(|balances| {
        let balances = balances.borrow();

        ic_cdk::println!(
            "Fetching all user balances... Total users: {}",
            balances.len()
        );

        for (principal, amount) in balances.iter() {
            ic_cdk::println!("User: {:?}, Balance: {}", principal, amount);
        }

        balances.iter().map(|(p, a)| (*p, *a)).collect()
    })
}

export_candid!();
