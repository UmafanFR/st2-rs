use crate::commands::{Context, Error};
use crate::umasheet;

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    ephemeral,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    umasheet::init_uma_list().await;
    ctx.reply("Uma list updated!").await?;
    Ok(())
}
