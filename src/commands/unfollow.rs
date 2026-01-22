use crate::commands::{Context, Error, autocomplete_uma_name};
use crate::{sheet, umasheet};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    ephemeral,
    category = "Umastagram"
)]
pub async fn unfollow(
    ctx: Context<'_>,
    #[description = "Umamusume to unfollow"]
    #[autocomplete = "autocomplete_uma_name"]
    umamusume: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let username = ctx.author().name.clone();

    let range = umasheet::get_follow_range(&username, &umamusume)
        .await?
        .ok_or_else(|| format!("You are not following **{}**!", umamusume))?;

    sheet::sheet()
        .clear(&range)
        .await
        .map_err(|e| format!("Failed to unfollow: {}", e))?;

    ctx.reply(format!("You are no longer following **{}**!", umamusume))
        .await?;
    Ok(())
}
