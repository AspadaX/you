use anyhow::anyhow;
use anyhow::{Error, Result};
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessageContentPartTextArgs, ResponseFormat};
use async_openai::types::ChatCompletionRequestUserMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateChatCompletionResponse;
use async_openai::Client;

#[derive(Debug)]
pub struct LLM {
    model: String,
    client: Client<OpenAIConfig>
}

impl LLM {
    pub fn new() -> Result<Self, Error> {
        let api_base: String = std::env::var("DONE_OPENAI_API_BASE")?;
        let api_key: String = std::env::var("DONE_OPENAI_API_KEY")?;
        let model: String = std::env::var("DONE_OPENAI_MODEL")?;

        let llm_configuration: OpenAIConfig = OpenAIConfig::default()
            .with_api_key(api_key)
            .with_api_base(api_base);
        let client: Client<OpenAIConfig> = async_openai::Client::with_config(
            llm_configuration
        );

        Ok(Self { model, client})
    }

    pub fn generate(&self, prompt: String) -> Result<String, Error> {
        let runtime = tokio::runtime::Runtime::new()?;
        let result = runtime.block_on(
            async {
                let request = CreateChatCompletionRequestArgs::default()
                    .model(&self.model)
                    .messages(vec![ChatCompletionRequestUserMessageArgs::default()
                        .content(vec![
                            ChatCompletionRequestMessageContentPartTextArgs::default()
                                .text(prompt)
                                .build()?
                                .into(),
                        ])
                        .build()?
                        .into()])
                    .build()?;

                let response: CreateChatCompletionResponse =
                    match self.client.chat().create(request.clone()).await {
                        std::result::Result::Ok(response) => response,
                        Err(e) => {
                            anyhow::bail!("Failed to execute function: {}", e);
                        }
                    };
                
                if let Some(content) = response.choices[0].clone().message.content {
                    return Ok(content);
                }

                return Err(anyhow!("No response is retrieved from the LLM"));
            }
        )?;

        Ok(result)
    }

    pub fn generate_json(&self, prompt: String) -> Result<String, Error> {
        let runtime = tokio::runtime::Runtime::new()?;
        let result = runtime.block_on(
            async {
                let request = CreateChatCompletionRequestArgs::default()
                    .model(&self.model)
                    .response_format(ResponseFormat::JsonObject)
                    .messages(vec![ChatCompletionRequestUserMessageArgs::default()
                        .content(vec![
                            ChatCompletionRequestMessageContentPartTextArgs::default()
                                .text(prompt)
                                .build()?
                                .into(),
                        ])
                        .build()?
                        .into()])
                    .build()?;

                let response: CreateChatCompletionResponse =
                    match self.client.chat().create(request.clone()).await {
                        std::result::Result::Ok(response) => response,
                        Err(e) => {
                            anyhow::bail!("Failed to execute function: {}", e);
                        }
                    };
                
                if let Some(content) = response.choices[0].clone().message.content {
                    return Ok(content);
                }

                return Err(anyhow!("No response is retrieved from the LLM"));
            }
        )?;

        Ok(result)
    }
}