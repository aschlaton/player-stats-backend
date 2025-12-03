use super::provider::LLMProvider;

/// gemini
pub const GEMINI_MODEL: &str = "gemini-2.5-pro";

/// openai
pub const OPENAI_MODEL: &str = "gpt-4o";

/// gemini() for Google Gemini
/// openai() for OpenAI GPT
pub fn get_provider() -> LLMProvider {
    LLMProvider::gemini()
}
