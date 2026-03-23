use near_sdk::{env, near, require, AccountId, PanicOnDefault};

const ERR_OWNER_ONLY: &str = "Only the owner can call this method";
const ERR_OWNER_UNCHANGED: &str = "Owner is already set to this account";

/// Minimal ownership contract used as the baseline for the vault.
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self { owner }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        self.assert_owner();
        require!(self.owner != new_owner, ERR_OWNER_UNCHANGED);
        self.owner = new_owner;
    }

    fn assert_owner(&self) {
        require!(env::predecessor_account_id() == self.owner, ERR_OWNER_ONLY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn account(id: &str) -> AccountId {
        id.parse().unwrap()
    }

    fn context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn initializes_with_the_configured_owner() {
        let owner = account("owner.testnet");

        testing_env!(context(owner.clone()).build());
        let contract = Contract::new(owner.clone());

        assert_eq!(contract.get_owner(), owner);
    }

    #[test]
    fn owner_can_transfer_ownership() {
        let owner = account("owner.testnet");
        let next_owner = account("next-owner.testnet");

        testing_env!(context(owner.clone()).build());
        let mut contract = Contract::new(owner.clone());

        testing_env!(context(owner).build());
        contract.transfer_ownership(next_owner.clone());

        assert_eq!(contract.get_owner(), next_owner);
    }

    #[test]
    #[should_panic(expected = "Only the owner can call this method")]
    fn non_owner_cannot_transfer_ownership() {
        let owner = account("owner.testnet");
        let attacker = account("attacker.testnet");
        let next_owner = account("next-owner.testnet");

        testing_env!(context(owner.clone()).build());
        let mut contract = Contract::new(owner);

        testing_env!(context(attacker).build());
        contract.transfer_ownership(next_owner);
    }

    #[test]
    #[should_panic(expected = "Owner is already set to this account")]
    fn transfer_rejects_the_same_owner() {
        let owner = account("owner.testnet");

        testing_env!(context(owner.clone()).build());
        let mut contract = Contract::new(owner.clone());

        testing_env!(context(owner.clone()).build());
        contract.transfer_ownership(owner);
    }
}
