use teloxide::{prelude::Requester, types::Message};

use crate::types::{BotType, HandlerResult};

pub async fn message_handler(bot: BotType, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Pong").await?;
    Ok(())
}
