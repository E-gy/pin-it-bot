use serenity::{
	async_trait,
	model::{channel::Reaction, gateway::Ready, guild::Guild},
	prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		log::info!("@{}#{} connected successfully!", ready.user.name, ready.user.discriminator);
	}

	async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
		if is_new {
			if let Some(chan) = guild.system_channel_id {
				if let Err(e) = chan.send_message(ctx.http, |m| {
					m.add_embed(|e| e
						.color(0xf72f2f)
						.title("Hi, I'm a pin bot 📌")
						.description("Just add a 📌 emoji to any message to pin it.
						Likewise removing it (all of them) will unpin the message.")
					)
				}).await {
					log::error!("Failed to send the welcome message: {:?}", e);
				}
			}
		}
	}

	async fn reaction_add(&self, ctx: Context, react: Reaction) {
		if react.emoji.unicode_eq("📌") {
			if let Err(e) = react.channel_id.pin(&ctx.http, react.message_id).await {
				log::error!("Failed to pin message {}: {:?}", react.message_id, e);
			}
		}
	}

	async fn reaction_remove(&self, ctx: Context, react: Reaction) {
		if react.emoji.unicode_eq("📌") {
			match react.channel_id.reaction_users(&ctx.http, react.message_id, react.emoji.clone(), Some(1), None).await {
				Ok(u) => if u.is_empty() {
					if let Err(e) = react.channel_id.unpin(&ctx.http, react.message_id).await {
						log::error!("Failed to unpin message {}: {:?}", react.message_id, e);
					}
				}
				Err(e) => log::error!("Failed to query message emojis {}: {:?}", react.message_id, e),
			}
		}
	}
}

#[tokio::main]
async fn main() {
	env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "pin_it_bot=info"));
	let token = std::env::var("DISCORD_BOT_TOKEN").expect("Can't find bot token");
	let mut client = Client::builder(&token).event_handler(Handler).await.expect("Failed to start");
	client.start().await.expect("Failed to connect");
}
