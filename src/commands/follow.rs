use crate::commands::{Context, Error, autocomplete_uma_name};
use crate::{sheet, umasheet};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    ephemeral,
    category = "Umastagram"
)]
pub async fn follow(
    ctx: Context<'_>,
    #[description = "Umamusume to follow"]
    #[autocomplete = "autocomplete_uma_name"]
    umamusume: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let username = ctx.author().name.clone();

    if umasheet::is_already_following(&username, &umamusume).await? {
        return Err(format!("You are already following **{}**!", umamusume).into());
    }

    let row = vec![vec![
        serde_json::json!(username),
        serde_json::json!(umamusume),
    ]];

    sheet::sheet()
        .append("Membres!A:B", row)
        .await
        .map_err(|e| format!("Failed to add to sheet: {}", e))?;

    ctx.reply(format!("You are now following **{}**!", umamusume))
        .await?;
    Ok(())
}
