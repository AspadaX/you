use std::collections::HashMap;

use anyhow::{Error, Result};
use cchain::core::{command::CommandLine, interpreter::Interpreter, traits::{Execution, ExecutionType}};

use crate::llm::LLM;

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
        let mut example_env_var = HashMap::new();
        example_env_var.insert("EXAMPLE".to_string(), "this is a value".to_string());
        
        let command_line_template = CommandLine::new(
            "A command".to_string(), 
            vec!["arugments".to_string()], 
            Some(Interpreter::Sh), 
            Some(example_env_var), 
            Some("/path/to/working/directory".to_string())
        );
        
        let prompt: String = "Please break down the following command sent by the user: "
            .to_string() + &self.user_query + "\n This is your template, output in json:" + &command_line_template.to_string();
        
        let result: Vec<CommandLine> = serde_json::from_str(
            &self.llm.generate_json(prompt)?
        )?;
        
        self.command_lines = result;
        
        Ok(())
    }
}

impl<T> Execution<T> for Agent 
where 
    T: Clone + Send + Sync + 'static,
{
    fn get_execution_type(&self) -> &cchain::core::traits::ExecutionType {
        &ExecutionType::Chain
    }
    
    fn execute(&mut self) -> anyhow::Result<Vec<T>, anyhow::Error> {
        for command in &mut self.command_lines {
            command.execute()?;
        }
        
        Ok(vec![])
    }
}