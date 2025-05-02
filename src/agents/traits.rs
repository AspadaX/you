use anyhow::Error;
use cchain::display_control::display_tree_message;
use serde::{Deserialize, Serialize};

use crate::llm::{Context, FromNaturalLanguageToJSON};

/// The `Step` trait defines a workflow step for an agent that processes user queries
/// and converts them into actionable commands or objects of type `T`.
///
/// This trait combines the functionalities of `Context` and `FromNaturalLanguageToJSON`
/// to manage the agent's context and handle natural language processing.
///
/// # Type Parameters
///
/// * `T` - The type of the object that the agent will produce as a result of processing
///   the user's query. This type must implement the `Explainable` trait and be deserializable
///   using Serde.
///
/// # Example
///
/// ```
/// use anyhow::Result;
/// use serde::Deserialize;
/// use async_openai::types::ChatCompletionRequestMessage;
/// use my_crate::llm::{Step, Context, FromNaturalLanguageToJSON};
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct MyCommand {
///     action: String,
///     target: String,
/// }
///
/// struct MyAgent {
///     context: Vec<ChatCompletionRequestMessage>,
/// }
///
/// impl Context for MyAgent {
///     fn get_context(&self) -> &Vec<ChatCompletionRequestMessage> {
///         &self.context
///     }
///
///     fn access_context(&mut self) -> &mut Vec<ChatCompletionRequestMessage> {
///         &mut self.context
///     }
/// }
///
/// impl FromNaturalLanguageToJSON for MyAgent {
///     fn from_natural_language_to_json(&mut self) -> Result<String, anyhow::Error> {
///         // Mock implementation for demonstration purposes
///         Ok(r#"{"action": "move", "target": "north"}"#.to_string())
///     }
/// }
///
/// impl<T> Step<T> for MyAgent where for<'de> T: Explainable + Deserialize<'de> {}
///
/// fn main() -> Result<()> {
///     let mut agent = MyAgent { context: vec![] };
///     let command: MyCommand = agent.next_step("Move north")?;
///     assert_eq!(command, MyCommand { action: "move", target: "north" });
///     Ok(())
/// }
pub trait Step<T>: Context + FromNaturalLanguageToJSON
where
    for<'de> T: Serialize + Deserialize<'de>,
{
    /// Executes the next step of the agent's workflow.
    ///
    /// # Arguments
    ///
    /// * `user_query` - A string slice that represents the user's query.
    ///
    /// # Returns
    ///
    /// This function returns a result containing the deserialized object of type `T` if successful,
    /// or an `Error` if something goes wrong during the process.
    ///
    /// # Example
    ///
    /// ```
    /// use anyhow::Result;
    /// use serde::Deserialize;
    /// use my_crate::llm::{Step, Context, FromNaturalLanguageToJSON};
    ///
    /// #[derive(Deserialize)]
    /// struct MyCommand {
    ///     action: String,
    ///     target: String,
    /// }
    ///
    /// struct MyAgent {
    ///     context: Vec<async_openai::types::ChatCompletionRequestMessage>,
    /// }
    ///
    /// impl Context for MyAgent {
    ///     fn get_context(&self) -> &Vec<async_openai::types::ChatCompletionRequestMessage> {
    ///         &self.context
    ///     }
    ///
    ///     fn access_context(&mut self) -> &mut Vec<async_openai::types::ChatCompletionRequestMessage> {
    ///         &mut self.context
    ///     }
    /// }
    ///
    /// impl FromNaturalLanguageToJSON for MyAgent {
    ///     fn from_natural_language_to_json(&mut self) -> Result<String, anyhow::Error> {
    ///         // Mock implementation for demonstration purposes
    ///         Ok(r#"{"action": "move", "target": "north"}"#.to_string())
    ///     }
    /// }
    ///
    /// impl<T> Step<T> for MyAgent where for<'de> T: Explainable + Deserialize<'de> {}
    ///
    /// fn main() -> Result<()> {
    ///     let mut agent = MyAgent { context: vec![] };
    ///     let command: MyCommand = agent.next_step("Move north")?;
    ///     assert_eq!(command.action, "move");
    ///     assert_eq!(command.target, "north");
    ///     Ok(())
    /// }
    /// ```
    fn next_step(&mut self, user_query: &str) -> Result<T, Error> {
        // Update the context by adding the user query
        self.add(async_openai::types::Role::User, user_query.to_string())?;

        let result: T = loop {
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

/// The `AgentExecution` trait defines the behavior for executing a command or task
/// represented by an object of type `T`. This trait is designed to be implemented
/// by agents that can process and execute specific tasks.
///
/// # Type Parameters
///
/// * `T` - The type of the command or task object that the agent will execute. This type
///   must implement the `Serialize` and `Deserialize` traits from Serde.
///
/// # Example
///
/// ```
/// use anyhow::Result;
/// use serde::{Serialize, Deserialize};
/// use my_crate::llm::AgentExecution;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyCommand {
///     action: String,
///     target: String,
/// }
///
/// struct MyAgent;
///
/// impl AgentExecution<MyCommand> for MyAgent {
///     fn execute(&mut self, command: &mut MyCommand) -> Result<String, anyhow::Error> {
///         // Mock implementation for demonstration purposes
///         Ok(format!("Executing action: {} on target: {}", command.action, command.target))
///     }
/// }
///
/// fn main() -> Result<()> {
///     let mut agent = MyAgent;
///     let mut command = MyCommand {
///         action: "move".to_string(),
///         target: "north".to_string(),
///     };
///     let result = agent.execute(&mut command)?;
///     assert_eq!(result, "Executing action: move on target: north");
///     Ok(())
/// }
/// ```
pub trait AgentExecution {
    /// Executes a given command or task represented by an object of type `T`.
    ///
    /// # Arguments
    ///
    /// * `command` - A mutable reference to the command or task object of type `T` that needs to be executed.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` containing a `String` if the execution is successful,
    /// or an `Error` if something goes wrong during the execution.
    ///
    /// # Example
    ///
    /// ```
    /// use anyhow::Result;
    /// use serde::{Serialize, Deserialize};
    /// use my_crate::llm::AgentExecution;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct MyCommand {
    ///     action: String,
    ///     target: String,
    /// }
    ///
    /// struct MyAgent;
    ///
    /// impl AgentExecution<MyCommand> for MyAgent {
    ///     fn execute(&mut self, data: &mut MyCommand) -> Result<String, anyhow::Error> {
    ///         // Mock implementation for demonstration purposes
    ///         Ok(format!("Executing action: {} on target: {}", command.action, command.target))
    ///     }
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let mut agent = MyAgent;
    ///     let mut command = MyCommand {
    ///         action: "move".to_string(),
    ///         target: "north".to_string(),
    ///     };
    ///     let result = agent.execute(&mut command)?;
    ///     assert_eq!(result, "Executing action: move on target: north");
    ///     Ok(())
    /// }
    /// ```
    fn execute(&mut self) -> Result<String, Error>;
}
