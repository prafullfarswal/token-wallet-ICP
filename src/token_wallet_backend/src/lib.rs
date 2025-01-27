use candid::{CandidType, Principal};
use ic_cdk_macros::{init, query, update};
use std::sync::Mutex;

#[derive(Default, CandidType)]
pub struct Wallet {
    balance: u64,
}

static WALLET: Mutex<Wallet> = Mutex::new(Wallet { balance: 0 });

#[init]
fn init() {
    ic_cdk::print("Token Wallet Initialized!");
}

#[update]
fn send_tokens(receiver: Principal, amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Amount should be greater than 0.".to_string());
    }
    let mut wallet = WALLET.lock().expect("Failed to lock wallet");
    if wallet.balance < amount {
        return Err("Insufficient funds.".to_string());
    }
    wallet.balance -= amount;
    ic_cdk::print(&format!("Sent {} tokens to {}", amount, receiver));
    Ok(())
}

#[update]
fn receive_tokens(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Amount should be greater than 0.".to_string());
    }
    let mut wallet = WALLET.lock().expect("Failed to lock wallet");
    wallet.balance += amount;
    ic_cdk::print(&format!("Received {} tokens", amount));
    Ok(())
}

#[query]
fn get_balance() -> u64 {
    let wallet = WALLET.lock().expect("Failed to lock wallet");
    wallet.balance
}

// Adding test module
#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;


    #[test]
    fn test_send_tokens_success() {
        let receiver = Principal::from_text("aaaaa-aa").unwrap();
        let initial_balance = get_balance();
        
        // Adding some tokens to the wallet
        receive_tokens(100).unwrap();

        // Now sending tokens
        assert_eq!(send_tokens(receiver, 50), Ok(())); // send 50 tokens
        
        // After sending, the balance should reduce
        assert_eq!(get_balance(), initial_balance + 50); // balance should be 50
    }

    #[test]
    fn test_send_tokens_insufficient_funds() {
        let receiver = Principal::from_text("aaaaa-aa").unwrap();
        
        // Try sending more tokens than the balance
        assert_eq!(send_tokens(receiver, 100), Err("Insufficient funds.".to_string()));
    }

    #[test]
    fn test_send_tokens_invalid_amount() {
        let receiver = Principal::from_text("aaaaa-aa").unwrap();
        
        // Try sending 0 tokens (should fail)
        assert_eq!(send_tokens(receiver, 0), Err("Amount should be greater than 0.".to_string()));
    }

    #[test]
    fn test_receive_tokens() {
        let initial_balance = get_balance();
        
        // Receiving tokens
        assert_eq!(receive_tokens(50), Ok(())); // add 50 tokens
        
        // After receiving, the balance should increase
        assert_eq!(get_balance(), initial_balance + 50);
    }

    #[test]
    fn test_get_balance() {
        // Check initial balance (should be 0 if no tokens have been added)
        assert_eq!(get_balance(), 0);
    }
}
