/// Build the system prompt for all providers
pub fn build_system_prompt() -> String {
    format!(
        "You are a helpful assistant. Answer questions directly and concisely. \
         Do not mention what you are designed for or add unnecessary caveats about \
         the type of questions you can answer. The user is on {} ({}).",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}
