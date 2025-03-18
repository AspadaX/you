use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Error, Result};
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs};
use cchain::{commons::utility::input_message, core::{command::CommandLine, interpreter::Interpreter, traits::Execution}, variable::Variable};
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::llm::{Context, LLM};

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
    /// LLM context
    context: Vec<ChatCompletionRequestMessage>
}

impl Agent {
    pub fn new(user_query: String) -> anyhow::Result<Self> {
        Ok(
            Agent {
                user_query,
                command_lines: vec![],
                llm: LLM::new()?,
                context: vec![],
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
        
        let required_information: Vec<&str> = vec![
            "system", "kernel_version", "os_version", "host_name"
        ];
        // Feed the system information as a background knowledge
        let mut system_information = String::new();
        for information in required_information {
            match information {
                "system" => system_information.push_str(&format!("System: {}\n", System::name().unwrap())),
                "kernel_version" => system_information.push_str(&format!("Kernel Version: {}\n", System::kernel_version().unwrap())),
                "os_version" => system_information.push_str(&format!("OS Version: {}\n", System::os_version().unwrap())),
                "host_name" => system_information.push_str(&format!("Host Name: {}\n", System::host_name().unwrap())),
                _ => {}
            }
        }
        
        let prompt: String = "Please break down the following command sent by the user in a json array: "
            .to_string() + &self.user_query + &format!(
                "\n {}This is your template, output in json array, which means that you should put your broken down commands in {{'commands': [{}]}}:\nAdditional notes: - You don't need to specifiy a working directory if you don't have a clue. In that case, just leave the working directory be null. - if you need to prompt the user for additional input, please encapsulate the variable in <<>>", 
                system_information,
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
            )?.trim().to_string();
            
            if &user_input == "y" {
                // First, we need to check if there are variables in the command
                let mut variables: Vec<Variable> = Vec::new();
                for string in command.get_arguments() {
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

impl Context for Agent {
    fn add(&mut self, role: async_openai::types::Role, content: String) -> Result<(), Error> {
        match role {
            async_openai::types::Role::User => Ok(self.context.push(ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into())),
            async_openai::types::Role::System => Ok(self.context.push(ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into())),
            async_openai::types::Role::Assistant => Ok(self.context.push(ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into())),
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