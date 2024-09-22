use teloxide::{
    dptree::case,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{BotCommand, Me, Message, Recipient, ReplyParameters},
    utils::command::BotCommands,
};

use crate::types::{BotType, HandlerResult, HandlerType};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Show help message")]
    Help,
}

pub async fn set_commands(bot: BotType) -> anyhow::Result<()> {
    let help_command = BotCommand::new("help", "Show help message");

    let command_list = vec![help_command];

    bot.set_my_commands(command_list).await?;
    Ok(())
}

pub fn command_handler() -> HandlerType {
    teloxide::filter_command::<Command, _>().branch(case![Command::Help].endpoint(help))
}

async fn help(bot: BotType, me: Me, msg: Message) -> HandlerResult {
    let desc = Command::descriptions();
    bot.send_message(msg.chat.id, desc.username_from_me(&me).to_string())
        .reply_parameters(ReplyParameters::new(msg.id).chat_id(Recipient::Id(msg.chat.id)))
        .await?;
    Ok(())
}
