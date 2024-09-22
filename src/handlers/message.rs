use teloxide::{dptree, prelude::Requester, types::Message};

use crate::types::{BotType, HandlerResult, HandlerType};

pub fn message_handler() -> HandlerType {
    dptree::endpoint(handler)
}
async fn handler(bot: BotType, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Pong").await?;
    Ok(())
}
