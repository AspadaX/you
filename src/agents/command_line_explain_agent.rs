use std::fmt::Display;

use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};

use crate::llm::{Context, FromNaturalLanguageToJSON, LLM};

use super::traits::Step;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandExplained {
    explanation: String,
}

impl Display for CommandExplained {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.explanation))
    }
}

pub struct CommandLineExplainAgent {
    /// LLM client
    llm: LLM,
    /// LLM context
    context: Vec<ChatCompletionRequestMessage>,
}

impl CommandLineExplainAgent {
    pub fn new() -> anyhow::Result<Self> {
        let mut context: Vec<ChatCompletionRequestMessage> = Vec::new();
        let example_data_structure = CommandExplained {
            explanation: "explain the command and its arguments briefly. one line maximum."
                .to_string(),
        };

        let mut system_prompt: String = format!(
            "You are an assistant that explains shell commands in simple terms. Please provide a brief explanation for any command given to you.\n\n"
        );
        system_prompt.push_str(&format!(
            "You need to respond json format like this: {}",
            &serde_json::to_string_pretty(&example_data_structure)?
        ));

        context.push(
            async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()?
                .into(),
        );

        Ok(CommandLineExplainAgent {
            llm: LLM::new()?,
            context,
        })
    }
}

impl Context for CommandLineExplainAgent {
    fn get_context(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.context
    }

    fn access_context(&mut self) -> &mut Vec<ChatCompletionRequestMessage> {
        &mut self.context
    }
}

impl FromNaturalLanguageToJSON for CommandLineExplainAgent {
    fn get_llm(&self) -> &LLM {
        &self.llm
    }
}

impl Step<CommandExplained> for CommandLineExplainAgent {}
