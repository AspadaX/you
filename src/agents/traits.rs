use anyhow::Error;

use super::command_json::CommandJSON;

pub trait Step {
    /// Executes the next step of the agent's workflow.
    fn next_step(&mut self, user_query: &str) -> Result<CommandJSON, Error>;
}
