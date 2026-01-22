mod commands;
mod log;
mod sheet;
mod umasheet;

use dotenv::dotenv;
use serenity::all::{ClientBuilder, Colour, CreateEmbed, GatewayIntents};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, commands::Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, commands::Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            let embed = CreateEmbed::default()
                .title(format!("Error in {}", ctx.command().name))
                .description(error.to_string())
                .color(Colour::RED);

            let reply = poise::CreateReply::default().embed(embed).ephemeral(true);
            if let Err(e) = ctx.send(reply).await {
                log::error!(
                    "Failed to send error message (interaction may have timed out): {}",
                    e
                );
            }
            log::error!(
                "Error in command (by {}):  `{}` ({})",
                ctx.author().name,
                ctx.invocation_string(),
                error,
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log::error!("Error while handling error: {}", e)
            }
        }
    }
}

async fn post_command(ctx: Context<'_>) {
    log::info!(
        "Command executed (by {}):  `{}`",
        ctx.author().name,
        ctx.invocation_string(),
    );
}

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    dotenv().ok();
    log::init_logger();
    sheet::init_sheet("1UimUbrDmGzTKc1H27rfvwZvIyDMTTwiyP7gcWZNOkzA").await;
    umasheet::init_uma_list().await;

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::follow(),
                commands::unfollow(),
                commands::post(),
                commands::update(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.unwrap().start().await {
        log::error!("Client error: {:?}", why);
    }
}
