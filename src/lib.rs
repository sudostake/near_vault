// Find all our documentation at https://docs.near.org
use near_sdk::{near, AccountId};

/// The vault smart contract managing owner access.
#[near(contract_state)]
pub struct Contract {
    /// Account ID of the vault owner.
    owner: AccountId,
}

// Implement the contract structure
#[near]
impl Contract {
    // Add your contract methods here
}
