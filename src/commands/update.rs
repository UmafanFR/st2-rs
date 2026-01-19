use crate::commands::{Context, Error};
use crate::umasheet;

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    ephemeral,
    track_edits,
    required_permissions = "MANAGE_CHANNELS",
    category = "Configuration"
)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    umasheet::init_uma_list().await;
    ctx.reply("Uma list updated!").await?;
    Ok(())
}
