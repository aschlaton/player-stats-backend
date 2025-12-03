use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::{gemini, openai};
use serde_json::Value;

use super::config::{GEMINI_MODEL, OPENAI_MODEL};

#[allow(dead_code)]
pub enum LLMProvider {
    Gemini(gemini::Client),
    OpenAI(openai::Client),
}

#[allow(dead_code)]
impl LLMProvider {
    pub fn gemini() -> Self {
        let api_key = std::env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY must be set");
        LLMProvider::Gemini(gemini::Client::new(&api_key))
    }

    pub fn openai() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set");
        LLMProvider::OpenAI(openai::Client::new(&api_key))
    }

    pub async fn prompt_with_schema(
        &self,
        system_prompt: &str,
        user_query: &str,
        schema: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            LLMProvider::Gemini(client) => {
                use rig::providers::gemini::completion::gemini_api_types::{
                    AdditionalParameters, GenerationConfig, Schema,
                };

                let schema: Schema = serde_json::from_value(schema)?;

                let generation_config = GenerationConfig {
                    response_mime_type: Some("application/json".to_string()),
                    response_schema: Some(schema),
                    temperature: Some(0.0),
                    ..Default::default()
                };

                let additional_params = AdditionalParameters::default()
                    .with_config(generation_config);

                let agent = client
                    .agent(GEMINI_MODEL)
                    .preamble(system_prompt)
                    .additional_params(serde_json::to_value(additional_params)?)
                    .build();

                let response = agent.prompt(user_query).await?;
                Ok(response)
            }
            LLMProvider::OpenAI(client) => {
                let agent = client
                    .agent(OPENAI_MODEL)
                    .preamble(system_prompt)
                    .build();

                // OpenAI: use JSON mode or function calling for structured output
                let prompt = format!(
                    "{}\n\nRespond with valid JSON matching this schema:\n{}",
                    user_query,
                    serde_json::to_string_pretty(&schema)?
                );

                let response = agent.prompt(&prompt).await?;
                Ok(response)
            }
        }
    }
}
