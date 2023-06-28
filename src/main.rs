use crate::project_tokens::ProjectToken;
use poise::{self, serenity_prelude};
use token_loader::TokenLoader;

struct Data {} //User data

mod project_tokens;
mod token_loader;

mod tests;
//Stupid error types copied from https://github.com/serenity-rs/poise/blob/current/examples/quickstart/main.rs
type AsyncError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, AsyncError>;

struct ProgramState {
    token_loader: TokenLoader<ProjectToken>,
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            token_loader: TokenLoader::new(),
        }
    }
}

///Bot pre-setup. May panic
fn main() {
    let program_state = ProgramState::default();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(main_async(program_state))
}

//#[tokio::main]
async fn main_async(program_state: ProgramState) {
    let commands = vec![age()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .token(
            program_state
                .token_loader
                .get_unwrap(&ProjectToken::DiscordToken),
        )
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .setup(move |context, ready, framework| {
            println!("Connected as '{}'", ready.user.name);

            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    match framework.run().await {
        Ok(_) => {
            println!("Framework exited with success");
        }
        Err(e) => {
            panic!("Fatal error occurred with framework: {:#?}", e)
        }
    }
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity_prelude::User>,
) -> Result<(), AsyncError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    //panic!("hi");
    Ok(())
}

// trait IntoResult<T,E>{
//     fn into_result(self, e: E) -> Result<T, E>;
// }

// impl<T> IntoResult<T,String> for Option<T> {
//     fn into_result(self, e: String) -> Result<T, String> {
//         match self {
//             None => {
//                 Err(e)
//             }
//             Some(v) => {
//                 Ok(v)
//             }
//         }
//     }
// }

// trait AsResult<T,E>{
//     fn as_result (&self, e: E) -> Result<&T, E>;
// }

// impl<T> AsResult<T,String> for Option<T> {
//     fn as_result(&self, e: String) -> Result<&T, String> {
//         match self {
//             None => {
//                 Err(e)
//             }
//             Some(ref v) => {
//                 Ok(v)
//             }
//         }
//     }
// }
// trait VariantOrError<TEnum, TError, TErrorFunc>{
//     /// Turn an enum into a result. It will be Err(E) if the enum has the same base variation (data inside the enum is not compared). error_func is lazily evaluated
//     fn variant_or_error(self,  expected_variant: TEnum, error_func: TErrorFunc) -> Result<TEnum, TError>
//         where TErrorFunc: FnOnce()-> TError;
// }

// impl<TErrorFunc, TError, TEnum> VariantOrError<TEnum,TError, TErrorFunc> for TEnum {
//     fn variant_or_error(self, expected_variant: TEnum, error_func: TErrorFunc) -> Result<TEnum,TError>
//         where TErrorFunc: FnOnce()-> TError,
//     {
//         let match_result = std::mem::discriminant(&expected_variant) == std::mem::discriminant(&self);

//         return if match_result {
//             Ok(self)
//         } else {
//             Err(error_func())
//         }

//     }
// }

// trait TrueIsError<E, TErrorFunc> {
//     fn true_is_error(self, error_func: TErrorFunc) -> Result<(),E>
//         where TErrorFunc: FnOnce()-> E
//     ;
// }

// impl<F> TrueIsError<String,F> for bool{
//     fn true_is_error(self, error : F) -> Result<(), String>
//         where F: FnOnce()-> String
//     {
//         match self {
//             true=> Err(error()),
//             false=> Ok(())
//         }
//     }
// }

// static CATS_TO_VOTE_FOR: once_cell::sync::Lazy<HashSet<String>> = once_cell::sync::Lazy::new(||{
//     let file_content = include_str!("cats.json");
//     let cats:HashSet<String> = serde_json::de::from_str(file_content).expect("failed to parse file");
//     cats
// });

// async fn cat_bot(){

//     let bot_commands = vec![
//         BotCommand{
//             description: "does something!".to_string(),
//             name: "vote".to_string(),
//             options: vec![
//                 CatCommandOption {
//                     name: "catname".to_string(),
//                     kind: CommandOptionType::String,
//                     required: true,
//                     description: "The name of the cat you wish to vote for!".to_string()
//                 }
//             ],
//             reply_fn : |interaction|{
//                 let result:Result<String, String> = {
//                     let option1 = interaction.data.options.first()
//                         .as_result("Failed to fetch first option from message".to_string())?
//                         .resolved.as_result("Failed to resolve option from message".to_string())?
//                         .to_owned();

//                     let option1 = match option1 {
//                         CommandDataOptionValue::String(s) => Ok(s),
//                         _=> Err("Command received should have string type!".to_string())
//                     }?;

//                     if !CATS_TO_VOTE_FOR.contains(option1.to_lowercase().as_str()){
//                         Ok(format!("{option1} is not a valid cat!"))
//                     }
//                     else {
//                         Ok(format!("You voted for {}!",option1))
//                     }

//                 };

//                 result
//             },
//         },

//         BotCommand{
//             name: "vote-cat".to_string(),
//             description: "pick a cat to vote for!".to_string(),
//             options:
//             {
//                   let mut options = Vec::new();
//                   for cat_name in CATS_TO_VOTE_FOR.iter() {
//                        options.push(CatCommandOption {
//                            name: cat_name.clone(),
//                            description: format!("Vote for {} the cat!",cat_name),
//                            required: false,
//                            kind: CommandOptionType::SubCommand
//                        });
//                   }

//                 options
//             },
//             reply_fn: |interaction|{
//                 //println!("interaction: {interaction:?}");
//                 let cat_option = interaction.data.options.first()
//                     .as_result("Failed to fetch first option from message".to_string())?
//                     .to_owned();

//                 let _kind = cat_option.kind.variant_or_error(CommandOptionType::SubCommand,||"Response type should have only been a subcommand!".to_string())?;
//                 let cat_name = cat_option.name.as_str();

//                 CATS_TO_VOTE_FOR.contains(cat_name).not().true_is_error(||"Invalid cat name given".to_string())?;

//                 Ok(format!("Voted for {}!",cat_name))
//             },

//         }
//     ];

//     let token = load_discord_token().unwrap();

//     let mut client = Client::builder(token,GatewayIntents::empty())
//         .event_handler(CatBotCommandHandler { commands: bot_commands })
//         .await.unwrap();

//     if let Err(e) = client.start().await{
//         eprintln!("Client failed to start! {:?}",e);
//     }
// }

// async fn schedule_next_cat_deadline(){
//     let mut scheduler = tokio_cron_scheduler::JobScheduler::new().await.unwrap();
//     scheduler.add(
//         tokio_cron_scheduler::Job::new("0 0 6 * * Sun",|uuid,lock|{
//             println!("Cat vote results time!");

//         }).unwrap()
//     ).await.unwrap();
// }

// fn load_discord_token() -> Result<String,std::io::Error>{
//     let contents = std::fs::read_to_string("./discord.file")?;
//     Ok(contents)
// }

// #[derive(Clone)]
// struct BotCommand{
//     name: String,
//     description: String,
//     options: Vec<CatCommandOption>,
//     reply_fn: fn(&ApplicationCommandInteraction)-> Result<String,String>
// }

// #[derive(Clone)]
// struct CatCommandOption {
//     name: String, //NO SPACES
//     description: String,
//     kind: CommandOptionType,
//     required: bool
// }

// struct CatBotCommandHandler{
//     commands: Vec<BotCommand>
// }

// #[async_trait]
// impl EventHandler for CatBotCommandHandler {
//     async fn ready(&self, context: Context, ready: Ready){
//         println!("Connected as '{}'",ready.user.name);

//         let mut failed_commands = 0;

//         let commands = Command::get_global_application_commands(&context.http).await;
//         let old_commands = match commands {
//             Ok(v) => {
//                Some(v)
//             }
//             Err(e) => {
//                 eprintln!("Failed to fetch commands! Assuming none are registered {:?}",e);
//                 None
//             }
//         };

//         for new_command in self.commands.clone().into_iter(){
//             // match &old_commands {
//             //     Some(v) =>{
//             //         let similar_commands: Vec<&Command> = v.iter().filter(|c|{
//             //             new_command.name == c.name&&
//             //                 new_command.description == c.description &&
//             //                 c.options.iter().zip(new_command.options.iter()).all(|pair|{
//             //                     pair.0.required == pair.1.required &&
//             //                         pair.0.name == pair.1.name &&
//             //                         pair.0.description == pair.1.description &&
//             //                         pair.0.kind == pair.1.kind
//             //                 })
//             //         }).collect();
//             //
//             //         // let similar_commands:Vec<_> = v.iter().filter(|c| c.name == new_command.name).collect();
//             //         //
//             //         if similar_commands.len()>1{
//             //             eprintln!("more than one matching command!{:?}",similar_commands);
//             //         }
//             //
//             //         if similar_commands.len()==1{
//             //             let similar_command = similar_commands.get(0).unwrap();
//             //
//             //             println!("Similar commands already found! name: {}\n Overwrite it (or ignore) y/n, m for more info?" , similar_command.name);
//             //             let mut string = String::new();
//             //             let overreide = loop {
//             //                 string.clear();
//             //                 std::io::stdin().read_line(&mut string).unwrap();
//             //                 match string.get(0..1).unwrap_or_default() {
//             //                     "y" =>{break true},
//             //                     "n" =>{break false; },
//             //                     "m" =>{
//             //                         println!("{:?}",similar_command);
//             //                         continue;
//             //                     }
//             //                     &_ => {continue;}
//             //                 }
//             //             };
//             //
//             //             if !overreide{
//             //                 continue;
//             //             }
//             //
//             //
//             //
//             //
//             //         }
//             //
//             //
//             //
//             //     },
//             //     None=>{}
//             // }

//             let command_result;
//             command_result = Command::create_global_application_command(&context.http, |command_builder|{
//                 command_builder
//                     .name(new_command.name)
//                     .description(new_command.description);

//                     for option in new_command.options {
//                         command_builder.create_option(|option_builder|{
//                             option_builder.kind(option.kind)
//                                 .name(option.name)
//                                 .description(option.description)
//                                 .required(option.required);
//                             option_builder
//                         });
//                     }

//                 command_builder
//             }).await;

//             match command_result {
//                 Ok(c) =>{
//                     println!("command created! '{}'",c.name);
//                 },
//                 Err(ref e)=> {
//                     failed_commands+=1;
//                     eprintln!("failed to create command!\ncommand_result:{:#?}\nerror:{}. {} command(s) failed to register", command_result, e,failed_commands);
//                 }
//             }
//         }

//     }

//     async fn interaction_create(&self, context: Context, interaction: Interaction){
//         if let Interaction::ApplicationCommand(command) = interaction{
//             let name = command.user.name.clone();
//             let descriminator = command.user.discriminator;

//             println!("interaction recevied from {name}:{descriminator}");

//             let command_name_requested = command.data.name.as_str();
//             let bot_command = self.commands.iter().find(|c| c.name.as_str() == command_name_requested);
//             let bot_command = match bot_command {
//                 Some(v) => v,
//                 None => {
//                     let reply = command.create_interaction_response(&context.http, |response| {
//                         response.kind(ChannelMessageWithSource)
//                             .interaction_response_data(|r| r.content("not a valid command!"))
//                     }).await;
//                     if let Err(e) = reply{
//                         eprintln!("Failed to reply to user! {:?}",e);
//                     }

//                     return;
//                 }

//             };

//             let response = (bot_command.reply_fn)(&command);

//             match response {
//                 Ok(v)=>{
//                     let reply_result = command.create_interaction_response(&context.http, |response| {
//                         response.kind(InteractionResponseType::ChannelMessageWithSource)
//                             .interaction_response_data(|message|message.content(v))
//                     }).await;
//                     match reply_result {
//                         Err(e)=>{
//                             eprintln!("Response failed! {:?}",e)
//                         }
//                         _ => {
//                             println!("replied to {}:{}",command.user.name, command.user.discriminator)
//                         }
//                     }
//                 },
//                 Err(e)=>{
//                     eprintln!("Failed to handle command! {}.",e);

//                     let reply_result = command.create_interaction_response(&context.http, |response| {
//                         response.kind(InteractionResponseType::ChannelMessageWithSource)
//                             .interaction_response_data(|message|
//                                 message.content(
//                                     format!("Oh no an error happened :( Couldn't process command because: {}.",e)
//                                 )
//                             )
//                     }).await;
//                     match reply_result {
//                         Err(e)=>{
//                             eprintln!("Response failed! {:?}",e)
//                         }
//                         _ => {
//                             println!("replied to {}:{}",command.user.name, command.user.discriminator)
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
