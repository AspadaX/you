use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Error, Result};
use cchain::{commons::utility::input_message, core::{command::CommandLine, interpreter::Interpreter, traits::Execution}};
use serde::{Deserialize, Serialize};

use crate::llm::LLM;

#[derive(Debug, Deserialize, Serialize)]
pub struct BreakdownCommands {
    commands: Vec<CommandLine>,
}

/// An agent that is for breaking down the command,
/// intepret into a series of command line arguments, 
/// then execute them.
#[derive(Debug)]
pub struct Agent {
    /// The user's command in natural language.
    user_query: String,
    /// The command lines to execute
    command_lines: Vec<CommandLine>,
    /// LLM client
    llm: LLM,
}

impl Agent {
    pub fn new(user_query: String) -> anyhow::Result<Self> {
        Ok(
            Agent {
                user_query,
                command_lines: vec![],
                llm: LLM::new()?,
            }
        )
    }
    
    /// Breakdown the user's command into a series of command line arguments.
    pub fn breakdown(&mut self) -> Result<(), Error> {
        let mut example_env_var: HashMap<String, String> = HashMap::new();
        example_env_var.insert("EXAMPLE".to_string(), "this is a value".to_string());
        
        let command_line_template = CommandLine::new(
            "A command".to_string(), 
            vec!["arugments".to_string()], 
            Some(Interpreter::Sh), 
            Some(example_env_var), 
            Some("/path/to/working/directory".to_string())
        );
        
        let prompt: String = "Please break down the following command sent by the user in a json array: "
            .to_string() + &self.user_query + &format!(
                "\n This is your template, output in json array, which means that you should put your broken down commands in {{'commands': [{}]}}:", 
                &serde_json::to_string_pretty(&command_line_template)?
            );
        
        let response: String = self.llm.generate_json(prompt)?;
        
        let result: BreakdownCommands = serde_json::from_str(&response).unwrap();
        
        self.command_lines = result.commands;
        
        Ok(())
    }
    
    pub fn execute(&mut self) -> Result<(), anyhow::Error> {
        for command in &mut self.command_lines {
            let user_input: String = input_message(
                &format!(
                    "Are you okay with executing the following command: {} (y/n)", 
                    command
                )
            )?;
            
            if &user_input == "y" {
                command.execute()?;
                continue;
            }
            
            if &user_input == "n" {
                return Err(anyhow!("Execution rejected"));
            }
        }
        
        Ok(())
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();
        for command_line in self.command_lines.iter() {
            output_string.push_str(&command_line.to_string());
        }
        
        f.write_str(&format!("{}", output_string))
    }
}