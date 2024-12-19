use candid::{CandidType, Deserialize};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::*;
use std::collections::HashMap;
use ic_ledger_types::{AccountIdentifier, Tokens, BlockIndex, Subaccount};

#[derive(CandidType, Deserialize, Clone)]
struct WalletState {
    balances: HashMap<AccountIdentifier, Tokens>,
    owner: AccountIdentifier,
}

impl Default for WalletState {
    fn default() -> Self {
        let default_subaccount = Subaccount([0; 32]);
        WalletState {
            balances: HashMap::new(),
            owner: AccountIdentifier::new(&ic_cdk::id(), &default_subaccount),
        }
    }
}

thread_local! {
    static STATE: std::cell::RefCell<WalletState> = std::cell::RefCell::new(WalletState::default());
}

#[derive(CandidType, Deserialize)]
struct TransferArgs {
    to: AccountIdentifier,
    amount: Tokens,
}

#[update]
async fn transfer(args: TransferArgs) -> CallResult<BlockIndex> {
    STATE.with(|s| {
        let sender = s.borrow().owner;
        let sender_balance = s.borrow().balances
            .get(&sender)
            .cloned()
            .unwrap_or_else(|| Tokens::from_e8s(0));

        if sender_balance.e8s() < args.amount.e8s() {
            ic_cdk::trap("Insufficient balance");
        }

        let new_sender_balance = Tokens::from_e8s(
            sender_balance.e8s() - args.amount.e8s()
        );

        let recipient_balance = s.borrow().balances
            .get(&args.to)
            .cloned()
            .unwrap_or_else(|| Tokens::from_e8s(0));

        let new_recipient_balance = Tokens::from_e8s(
            recipient_balance.e8s() + args.amount.e8s()
        );

        // Update balances
        s.borrow_mut().balances.insert(sender, new_sender_balance);
        s.borrow_mut().balances.insert(args.to, new_recipient_balance);
    });

    Ok(0) // Return dummy block index for demo
}

#[query]
fn get_balance(account: AccountIdentifier) -> Tokens {
    STATE.with(|s| {
        s.borrow()
            .balances
            .get(&account)
            .cloned()
            .unwrap_or_else(|| Tokens::from_e8s(0))
    })
}

#[update]
fn receive_tokens(amount: Tokens) {
    STATE.with(|s| {
        let owner = s.borrow().owner;
        let current_balance = s.borrow()
            .balances
            .get(&owner)
            .cloned()
            .unwrap_or_else(|| Tokens::from_e8s(0));

        let new_balance = Tokens::from_e8s(
            current_balance.e8s() + amount.e8s()
        );
        
        s.borrow_mut().balances.insert(owner, new_balance);
    });
}

// Helper function to create a subaccount
fn create_subaccount(index: u8) -> Subaccount {
    let mut bytes = [0; 32];
    bytes[0] = index;
    Subaccount(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        // Initialize test state
        let default_subaccount = Subaccount([0; 32]);
        let test_subaccount = create_subaccount(1);
        
        let sender = AccountIdentifier::new(&ic_cdk::id(), &default_subaccount);
        let recipient = AccountIdentifier::new(&ic_cdk::id(), &test_subaccount);
        
        // Add initial balance for testing
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.balances.insert(sender, Tokens::from_e8s(200));
        });
        
        // Test transfer
        let transfer_args = TransferArgs {
            to: recipient,
            amount: Tokens::from_e8s(100),
        };
        
        let _ = ic_cdk::block_on(transfer(transfer_args));
        
        // Verify balances
        let sender_balance = get_balance(sender);
        let recipient_balance = get_balance(recipient);
        
        assert_eq!(sender_balance.e8s(), 100);
        assert_eq!(recipient_balance.e8s(), 100);
    }
}
