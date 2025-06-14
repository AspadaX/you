use std::{collections::HashMap, fmt::Display};

use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs};

use crate::{
    information::{
        ContextualInformation, get_current_directory_structure, get_current_time,
        get_system_information,
    },
    llm::{Context, FromNaturalLanguageToJSON, LLM},
};

use super::{command_json::LLMActionType, traits::Step};

/// An agent that is for breaking down the command,
/// intepret into a series of command line arguments,
/// then execute them.
#[derive(Debug, Clone)]
pub struct SemiAutonomousCommandLineAgent {
    /// The command lines to execute
    command_line_to_execute: Option<String>,
    /// LLM client
    llm: LLM,
    /// LLM context
    context: Vec<ChatCompletionRequestMessage>,
}

impl SemiAutonomousCommandLineAgent {
    pub fn new(contextual_information_object: &ContextualInformation) -> anyhow::Result<Self> {
        // Setup a command line template for prompting the LLM
        let mut example_env_var: HashMap<String, String> = HashMap::new();
        example_env_var.insert("EXAMPLE".to_string(), "this is a value".to_string());

        let command_json_template: String = LLMActionType::get_llm_action_type_prompt_template();

        // The system prompt for the LLM
        let mut prompt: String = "Please translate the following command sent by the user to an executable sh command/script in a json.
            If you would like to have additional information to send or receive from the user, or perform other actions, please refer to the templates below.\n"
            .to_string();

        // Inject the system information
        prompt.push_str(&contextual_information_object.get_contextual_information()?);

        // Inject the template to the prompt
        prompt.push_str(&format!(
            "This is your template, output in json: {}:\n",
            &serde_json::to_string_pretty(&command_json_template)?
        ));

        // Additional instructions
        prompt.push_str("\nAdditional instructions:");
        prompt.push_str("- The `interpreter` now only supports sh. ");

        // Construct the context
        let mut context: Vec<ChatCompletionRequestMessage> = Vec::new();
        context.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );

        Ok(SemiAutonomousCommandLineAgent {
            command_line_to_execute: None,
            llm: LLM::new()?,
            context,
        })
    }
}

impl Step<LLMActionType> for SemiAutonomousCommandLineAgent {}

impl Display for SemiAutonomousCommandLineAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}",
            self.command_line_to_execute.as_ref().unwrap().to_string()
        ))
    }
}

impl Context for SemiAutonomousCommandLineAgent {
    fn get_context(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.context
    }

    fn access_context(&mut self) -> &mut Vec<ChatCompletionRequestMessage> {
        &mut self.context
    }
}

impl FromNaturalLanguageToJSON for SemiAutonomousCommandLineAgent {
    fn get_llm(&self) -> &LLM {
        &self.llm
    }
}
