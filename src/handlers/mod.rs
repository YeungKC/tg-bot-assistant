mod command;
mod message;

use command::command_handler;
use message::message_handler;
use teloxide::{
    dispatching::{DefaultKey, UpdateFilterExt},
    dptree,
    prelude::Dispatcher,
    types::Update,
};

use crate::types::{BotType, BoxedError};

pub fn create_dispatcher(bot: BotType) -> Dispatcher<BotType, BoxedError, DefaultKey> {
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(command_handler())
            .branch(dptree::endpoint(message_handler)),
    )
    .enable_ctrlc_handler()
    .build()
}

pub use command::set_commands;
