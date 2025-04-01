use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Error, Result};
use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs};
use cchain::{commons::utility::input_message, core::{command::{CommandLine, CommandLineExecutionResult}, interpreter::Interpreter, traits::Execution}, display_control::display_tree_message, variable::Variable};
use serde::{Deserialize, Serialize};

use crate::{information::{get_current_directory_structure, get_current_time, get_system_information}, llm::{Context, FromNaturalLanguageToJSON, LLM}};

pub trait Step {
    /// Executes the next step of the agent's workflow.
    fn next_step(&mut self, user_query: &str) -> Result<CommandJSON, Error>;
}

pub trait Executable {
    /// Executes the next step of the agent's workflow.
    fn execute(&mut self, command: &mut CommandJSON) -> Result<(), Error>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandJSON {
    pub explanation: String,
    pub command: CommandLine,
}

/// An agent that is for breaking down the command,
/// intepret into a series of command line arguments, 
/// then execute them.
#[derive(Debug)]
pub struct SemiAutonomousCommandLineAgent {
    /// The command lines to execute
    command_line_to_execute: Option<CommandLine>,
    /// LLM client
    llm: LLM,
    /// LLM context
    context: Vec<ChatCompletionRequestMessage>,
}

impl SemiAutonomousCommandLineAgent {
    pub fn new() -> anyhow::Result<Self> {
        // Setup a command line template for prompting the LLM
        let mut example_env_var: HashMap<String, String> = HashMap::new();
        example_env_var.insert("EXAMPLE".to_string(), "this is a value".to_string());
        
        let command_line_template = CommandLine::new(
            "A command".to_string(), 
            vec!["arugments".to_string()], 
            Some(Interpreter::Sh), 
            Some(example_env_var), 
            Some("/path/to/working/directory".to_string())
        );
        let command_json_template = CommandJSON {
            explanation: "explain the command and its arguments briefly. one line maximum.".to_string(),
            command: command_line_template,
        };
        
        let system_information: String = get_system_information();
        let current_time: String = get_current_time();
        let current_working_directory_structure: String = get_current_directory_structure();
        
        // The system prompt for the LLM
        let mut prompt: String = "Please translate the following command sent by the user to an executable command in a json. 
            No matter whatever the user sends to you, you should always output a json with the command translated.\n"
            .to_string();
        
        // Inject the system information
        prompt.push_str("Environment:\n");
        prompt.push_str(&system_information);
        prompt.push_str("Current Working Directory: ");
        prompt.push_str(std::env::current_dir()?.to_str().unwrap());
        prompt.push_str("\n");
        prompt.push_str("Current Working Directory Sturcture: ");
        prompt.push_str(&current_working_directory_structure);
        prompt.push_str("\n");
        prompt.push_str("Current Date and Time: ");
        prompt.push_str(&current_time);
        prompt.push_str("\n");
        
        // Inject the template to the prompt
        prompt.push_str(
            &format!(
                "This is your template, output in json: {}:\n", 
                &serde_json::to_string_pretty(&command_json_template)?
            )
        );
        
        // Additional instructions
        prompt.push_str("\nAdditional instructions:");
        prompt.push_str(
            "- In case you don't know the working directoy, or there is no need for having a working directoy, you should leave it be null."
        );
        prompt.push_str(
            "- If you need to prompt the user for additional input, please encapsulate the variable in <<>>"
        );
        prompt.push_str(
            "- The `interpreter` now only supports sh/Sh. Also, you should leave it be null if you don't need an interpreter."
        );
        
        // Construct the context
        let mut context: Vec<ChatCompletionRequestMessage> = Vec::new();
        context.push(
            ChatCompletionRequestSystemMessageArgs::default().content(prompt).build()?.into()
        );
        
        Ok(
            SemiAutonomousCommandLineAgent {
                command_line_to_execute: None,
                llm: LLM::new()?,
                context,
            }
        )
    }    
}

impl Executable for SemiAutonomousCommandLineAgent {
    fn execute(&mut self, command: &mut CommandJSON) -> Result<(), Error> {
        // We need to check if there are variables in the command
        // before executing the command
        let mut variables: Vec<Variable> = Vec::new();
        for string in command.command.get_arguments() {
            let mut command_variables: Vec<Variable> = Variable::parse_variables_from_str(
                string, 0
            )?;
            for variable in command_variables.iter_mut() {
                let value: String = input_message(
                    &format!("{}", variable.get_human_readable_name())
                )?
                    .trim()
                    .to_string();
                
                variable.register_value(value);
            }
            
            variables.extend(command_variables);
        }
        
        // We need to register the value in the command itself
        for variable in variables {
            command.command.inject_value_to_variables(
                &variable.get_raw_variable_name(), 
                variable.get_value()?
            )?;
        }
        
        let result: Vec<CommandLineExecutionResult> = command.command.execute()?;
        
        // Add the result to the context 
        self.add(async_openai::types::Role::System, result[0].get_output())?;
        
        Ok(())
    }
}

impl Step for SemiAutonomousCommandLineAgent {
    fn next_step(&mut self, user_query: &str) -> Result<CommandJSON, Error> {
        // Update the context by adding the user query
        self.add(async_openai::types::Role::User, user_query.to_string())?;
        
        let result: CommandJSON = loop {
            let response: String = self.from_natural_language_to_json()?;
            
            match serde_json::from_str(&response) {
                Ok(command) => break command,
                Err(_) => {
                    display_tree_message(2, "LLM returned a wrong JSON, retrying...");
                    continue;
                }
            }
        };
        
        Ok(result)
    }
}

impl Display for SemiAutonomousCommandLineAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.command_line_to_execute.as_ref().unwrap().to_string()))
    }
}

impl Context for SemiAutonomousCommandLineAgent {
    fn add(&mut self, role: async_openai::types::Role, content: String) -> Result<(), Error> {
        match role {
            async_openai::types::Role::User => Ok(self.context.push(ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into())),
            async_openai::types::Role::System => Ok(self.context.push(ChatCompletionRequestSystemMessageArgs::default().content(content).build()?.into())),
            async_openai::types::Role::Assistant => Ok(self.context.push(ChatCompletionRequestAssistantMessageArgs::default().content(content).build()?.into())),
            _ => Err(anyhow!("Invalid role")),
        }
    }
    
    fn clear(&mut self) -> Result<(), Error> {
        Ok(self.context.clear())
    }
    
    fn get_context(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.context
    }
}

impl FromNaturalLanguageToJSON for SemiAutonomousCommandLineAgent {
    fn from_natural_language_to_json(&mut self) -> Result<String, Error> {
        let response: String = self.llm.generate_json_with_context(self.get_context().clone())?;
        
        Ok(response)
    }
}