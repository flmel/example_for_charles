use models::Event;
// Find all our documentation at https://docs.near.org
use near_sdk::{env, near, near_bindgen, AccountId};

mod models;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    owner: AccountId,
    events: Vec<Event>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::current_account_id(),
            events: Vec::new(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let events: Vec<Event> = Vec::new();

        Contract { owner, events }
    }

    pub fn add_event(&mut self, title: String, estimated_budget: u128, description: String) {
        let id = self.events.len() as i32;
        self.events
            .push(Event::new(id, title, estimated_budget, description));

        env::log_str("Added a new event!");
    }

    pub fn list_events(&self) -> Vec<Event> {
        let events = &self.events;

        return events.to_vec();
    }

    pub fn event_count(&mut self) -> usize {
        return self.events.len();
    }

    pub fn add_vote(&mut self, id: usize) {
        let event: &mut Event = self.events.get_mut(id).unwrap();
        let voter = env::predecessor_account_id();

        event.total_votes = event.total_votes + 1;
        env::log_str("Vote submitted successfully for this event!");
        event.votes.push(voter.to_string());
    }

    pub fn get_total_votes(&mut self, id: usize) -> u64 {
        let event: &mut Event = self.events.get_mut(id).unwrap();
        return event.total_votes.try_into().unwrap();
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn add_project() {
        // Use `AccountId::new` instead of `AccountId::new_unchecked`
        let alice: AccountId = "alice.testnet".parse().unwrap();
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());
        let mut contract = Contract::new(alice);

        contract.add_event(
            "New Contemporary Art Show".to_string(),
            200,
            "Amazing selection of
            international artists from all over the world"
                .to_string(),
        );

        let result = contract.event_count();

        assert_eq!(result, 1);
    }

    #[test]
    fn add_voter() {
        // Use `AccountId::new` instead of `AccountId::new_unchecked`
        let alice: AccountId = "alice.testnet".parse().unwrap();
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice);

        contract.add_event(
            "New Contemporary Art Show".to_string(),
            200,
            "Amazing selection of
        international artists from all over the world"
                .to_string(),
        );

        contract.add_vote(0);

        let result = contract.get_total_votes(0);

        assert_eq!(result, 1);
    }
}
