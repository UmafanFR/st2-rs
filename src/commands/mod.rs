mod follow;
mod help;
mod myfollow;
mod post;
mod unfollow;
mod update;

use crate::umasheet;
use futures::{Stream, StreamExt};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn autocomplete_uma_name<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(umasheet::get_uma_list_cached())
        .filter(move |name| {
            futures::future::ready(
                name.to_lowercase()
                    .starts_with(partial.to_lowercase().as_str()),
            )
        })
        .take(15)
}

pub use follow::follow;
pub use help::help;
pub use myfollow::myfollow;
pub use post::post;
pub use unfollow::unfollow;
pub use update::update;
