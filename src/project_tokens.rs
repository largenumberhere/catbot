use crate::token_loader::TokenDetails;

#[derive(Debug, strum::EnumIter, PartialEq, Clone, Eq, Hash)]
pub enum ProjectToken {
    TestToken,
    DiscordToken,
}

impl TokenDetails for ProjectToken {
    fn get_token_path(token_type: &ProjectToken) -> String {
        match token_type {
            ProjectToken::TestToken => String::from("./.keys/test.file"),
            ProjectToken::DiscordToken => String::from("./.keys/discord.file"),
        }
    }

    fn get_token_name(token_type: &ProjectToken) -> String {
        match token_type {
            ProjectToken::TestToken => String::from("Test token"),
            ProjectToken::DiscordToken => String::from("Discord bot token"),
        }
    }
}
