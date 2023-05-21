use std::{path::PathBuf};
use std::time::Duration;
use blake2::digest::typenum::op;
use serenity::{async_trait, Client};
use serenity::client::{Context, EventHandler};
use serenity::futures::future::err;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::GatewayIntents;

mod file_hasher;

#[tokio::main]
async fn main() {

    //compare_hash(PathBuf::from("watchable.json")).await;
    // let mut comparer = file_hasher::HashCompare::new(PathBuf::from("watchable.json")).await;
    // tokio::time::sleep(Duration::from_millis(5000)).await;
    // let result = comparer.compare().await;
    //
    //
    //
    // println!("{result:?}");
    cat_bot().await;

}

async fn cat_bot(){
    let token = load_discord_token().unwrap();

    let mut client = Client::builder(token,GatewayIntents::empty())
        .event_handler(CatBotCommandHandler)
        .await.unwrap();

    if let Err(e) = client.start().await{
        eprintln!("Client failed to start! {:?}",e);
    }
}

fn load_discord_token() -> Result<String,std::io::Error>{
    let contents = std::fs::read_to_string("discord.txt")?;
    Ok(contents)
}

struct CatBotCommandHandler;

#[async_trait]
impl EventHandler for CatBotCommandHandler {
    async fn ready(&self, context: Context, ready: Ready){
        println!("Connected as '{}'",ready.user.name);

        let command_result = Command::create_global_application_command(&context.http, |command|{
            command.name("hello")
                .description("says hi back!");

            command.create_option(|option|{
                option.name("greeting message")
                    .description("say something nice")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
        }).await;

        match command_result {
            Ok(c) =>{
                println!("command created! '{}'",c.name);
            },
            Err(ref e)=> {
                eprintln!("failed to create command!\ncommand_result:{:#?}\nerror:{}", command_result, e);
            }
        }
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction){
        if let Interaction::ApplicationCommand(command) = interaction{
            let name = command.user.name.clone();
            let descriminator = command.user.discriminator;

            println!("interaction recevied from {name}:{descriminator}");

            let content = match command.data.name.as_str() {
                "hello" => "hi there".to_string(),
                _ => "not implemented :(".to_string(),
            };

            if let Err(error) = command
                .create_interaction_response(&context.http, |response|{
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                }).await
            {
                println!("reply failed!{:?}",error);
            }
        }
    }
}



