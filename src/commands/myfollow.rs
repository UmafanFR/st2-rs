use serenity::all::MessageBuilder;

use crate::commands::{Context, Error};
use crate::umasheet;

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    ephemeral,
    category = "Umastagram"
)]
pub async fn myfollow(ctx: Context<'_>) -> Result<(), Error> {
    let username = ctx.author().name.clone();
    let follows = umasheet::get_user_follow(&username).await?;

    let mut message = MessageBuilder::new();
    if follows.is_empty() {
        message.push("You are not following anyone.");
    } else {
        message.push("You are following: ");
        message.push(follows.join(", "));
    }
    ctx.say(message.build()).await?;

    Ok(())
}
