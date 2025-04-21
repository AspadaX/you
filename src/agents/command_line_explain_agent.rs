use anyhow::{Error, Result};
use async_openai::types::ChatCompletionRequestMessage;

use crate::llm::LLM;

use super::{command_json::CommandJSON, traits::Step};

pub struct CommandLineExplainAgent {
    /// LLM client
    llm: LLM,
    /// LLM context
    context: Vec<ChatCompletionRequestMessage>,
}

impl Step for CommandLineExplainAgent {
    fn next_step(&mut self, user_query: &str) -> Result<CommandJSON, Error> {
        
    }
}