use anyhow::anyhow;
use anyhow::{Error, Result};
use async_openai::Client;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequest,
};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateChatCompletionResponse,
    },
};
use surfing::extract_json_to_string;
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct LLM {
    model: String,
    client: Client<OpenAIConfig>,
}

impl LLM {
    pub fn new() -> Result<Self, Error> {
        let api_base: String = std::env::var("DONE_OPENAI_API_BASE")
            .or_else(|_| std::env::var("YOU_OPENAI_API_BASE"))?;
        let api_key: String = std::env::var("DONE_OPENAI_API_KEY")
            .or_else(|_| std::env::var("YOU_OPENAI_API_KEY"))?;
        let model: String =
            std::env::var("DONE_OPENAI_MODEL").or_else(|_| std::env::var("YOU_OPENAI_MODEL"))?;

        let llm_configuration: OpenAIConfig = OpenAIConfig::default()
            .with_api_key(api_key)
            .with_api_base(api_base);
        let client: Client<OpenAIConfig> = async_openai::Client::with_config(llm_configuration);

        Ok(Self { model, client })
    }

    pub fn generate_json_with_context(
        &self,
        context: Vec<ChatCompletionRequestMessage>,
    ) -> Result<String, Error> {
        let runtime: Runtime = tokio::runtime::Runtime::new()?;
        let result: String = runtime.block_on(async {
            let request: CreateChatCompletionRequest = CreateChatCompletionRequestArgs::default()
                .model(&self.model)
                .messages(context)
                .build()?;

            let response: CreateChatCompletionResponse =
                match self.client.chat().create(request.clone()).await {
                    std::result::Result::Ok(response) => response,
                    Err(e) => {
                        anyhow::bail!("Failed to execute function: {}", e);
                    }
                };

            if let Some(content) = response.choices[0].clone().message.content {
                return Ok(extract_json_to_string(&content).unwrap());
            }

            return Err(anyhow!("No response is retrieved from the LLM"));
        })?;

        Ok(result)
    }
}

/// A context for storing messages.
pub trait Context {
    fn add(&mut self, role: async_openai::types::Role, content: String) -> Result<(), Error> {
        let context: &mut Vec<ChatCompletionRequestMessage> = self.access_context();

        match role {
            async_openai::types::Role::User => Ok(context.push(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            )),
            async_openai::types::Role::System => Ok(context.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            )),
            async_openai::types::Role::Assistant => Ok(context.push(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            )),
            _ => Err(anyhow!("Invalid role")),
        }
    }

    #[allow(dead_code)]
    fn clear(&mut self) -> Result<(), Error> {
        let context: &mut Vec<ChatCompletionRequestMessage> = self.access_context();
        Ok(context.clear())
    }

    /// Acquire a mutable reference to the context
    fn get_context(&self) -> &Vec<ChatCompletionRequestMessage>;

    /// Acquire an immutable reference to the context
    fn access_context(&mut self) -> &mut Vec<ChatCompletionRequestMessage>;
}

/// A trait for converting natural language to JSON.
///
/// This trait provides a method to convert natural language input into a JSON representation
/// using a large language model (LLM). It requires the implementor to also implement the `Context`
/// trait, which manages the context of messages exchanged with the LLM.
///
/// # Example
///
/// ```rust
/// use anyhow::Result;
/// use async_openai::types::{ChatCompletionRequestMessage, Role};
/// use crate::llm::{LLM, Context, FromNaturalLanguageToJSON};
///
/// struct MyAgent {
///     llm: LLM,
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
///     fn get_llm(&self) -> &LLM {
///         &self.llm
///     }
/// }
///
/// fn main() -> Result<()> {
///     let llm = LLM::new()?;
///     let mut agent = MyAgent {
///         llm,
///         context: Vec::new(),
///     };
///
///     agent.add(Role::User, "Translate this command to JSON.".to_string())?;
///     let json_response = agent.from_natural_language_to_json()?;
///     println!("Generated JSON: {}", json_response);
///
///     Ok(())
/// }
/// ```
pub trait FromNaturalLanguageToJSON: Context {
    /// Returns a reference to the LLM instance used for generating JSON.
    fn get_llm(&self) -> &LLM;

    /// Converts the current context of natural language messages into a JSON representation.
    ///
    /// This method sends the current context to the LLM and retrieves a JSON-formatted response.
    ///
    /// # Errors
    ///
    /// Returns an error if the LLM fails to generate a response or if the response is invalid.
    fn from_natural_language_to_json(&mut self) -> Result<String, Error> {
        self.get_llm()
            .generate_json_with_context(self.get_context().clone())
    }
}
