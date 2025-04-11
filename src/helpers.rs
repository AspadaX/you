use anyhow::{Result, Error};
use cchain::{commons::utility::input_message, display_control::{display_message, Level}};
use indicatif::ProgressBar;

use crate::{agent::{CommandJSON, Executable, SemiAutonomousCommandLineAgent, Step}, llm::Context, styles::start_spinner};

pub fn process_run_with_one_single_instruction(command_in_natural_language: &str) -> Result<(), Error> {
    let mut agent = SemiAutonomousCommandLineAgent::new()?;
    let mut is_first_round: bool = true;
    
    let mut user_prompt = String::new();
    loop {
        // Initialize an empty vector to store command lines
        let mut command_json: CommandJSON;
        
        // Use the user query provided in the `run` argument for the first round
        let spinner: ProgressBar = start_spinner("LLM is thinking...".to_string());
        if is_first_round {
            is_first_round = false;
            command_json = agent.next_step(
                &command_in_natural_language
            )?;
        } else {
            command_json = agent.next_step(&user_prompt)?;
        }
        
        // Clear the spinner
        spinner.finish_and_clear();
        
        // For prompting the LLM and the user
        let command_lines_text: String = "    > ".to_string() + command_json.command.to_string().as_str() + "\n";
        let command_lines_explanation: String = "        * ".to_string() + command_json.explanation.to_string().as_str() + "\n";
        
        // Register the user's input
        user_prompt = input_message(
            &format!("Execute the following command? (y for yes, or type to hint LLM)\n{}{}", command_lines_text, command_lines_explanation)
        )?;
        // we add the command lines to the agent's memory
        agent.add(async_openai::types::Role::Assistant, format!("{}{}", command_lines_text, command_lines_explanation))?;
        
        if user_prompt.trim() == "y" {
            match agent.execute(&mut command_json) {
                Ok(_) => {
                    display_message(Level::Logging, "Commands had been executed successfully.");
                    break;
                },
                Err(error) => {
                    display_message(Level::Error, &error.to_string());
                }
            };
        }
    }
    
    Ok(())
}

pub fn process_interactive_mode() -> Result<(), Error> {
    let mut agent = SemiAutonomousCommandLineAgent::new()?;
    
    let mut user_query: String = input_message("Yes, boss. What can I do for you:")?;
    loop {
        // Initialize an empty vector to store command lines
        let mut command_json: CommandJSON;
        
        // Use the user query for generate a command 
        let spinner: ProgressBar = start_spinner("LLM is thinking...".to_string());
        command_json = agent.next_step(&user_query)?;
        
        // Clear the spinner
        spinner.finish_and_clear();
        
        // For prompting the LLM and the user
        let command_lines_text: String = "    > ".to_string() + command_json.command.to_string().as_str() + "\n";
        let command_lines_explanation: String = "        * ".to_string() + command_json.explanation.to_string().as_str() + "\n";
        
        // Register the user's input
        user_query = input_message(
            &format!("Execute the following command? (y for yes, e for exit, or type to hint LLM)\n{}{}", command_lines_text, command_lines_explanation)
        )?;
        // we add the command lines to the agent's memory
        agent.add(async_openai::types::Role::Assistant, format!("{}{}", command_lines_text, command_lines_explanation))?;
        
        if user_query.trim() == "y" {
            match agent.execute(&mut command_json) {
                Ok(_) => {
                    display_message(Level::Logging, "Commands had been executed successfully.");
                    user_query = input_message("Boss, what else can I do for you:")?;
                },
                Err(error) => {
                    display_message(Level::Error, &error.to_string());
                }
            };
        }
        
        if user_query.trim() == "e" {
            display_message(Level::Logging, "See you boss.");
            break;
        }
    }
    
    Ok(())
}