use crate::project_tokens;
use crate::token_loader::TokenLoader;
use std::error::Error;
use std::fmt::format;

#[test]
fn test_token() -> Result<(), String> {
    let token_loader = TokenLoader::new();
    let discord_token = token_loader.get_unwrap(&project_tokens::ProjectToken::TestToken);

    let sample = "I am a discord token with a trailing space and newline";
    if discord_token != sample {
        return Err(format!(
            "test token is not equal to sample text '{}' and was instead '{}' ",
            sample, discord_token
        ));
    }

    Ok(())
}
