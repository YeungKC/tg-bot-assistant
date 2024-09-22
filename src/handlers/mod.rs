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
use tracing::trace;

use crate::types::{BotType, BoxedError};

pub fn create_dispatcher(bot: BotType) -> Dispatcher<BotType, BoxedError, DefaultKey> {
    Dispatcher::builder(
        bot,
        dptree::entry()
            .inspect(|update: Update| {
                trace!("Update: {:?}", update);
            })
            .branch(
                Update::filter_message()
                    .branch(command_handler())
                    .branch(message_handler()),
            ),
    )
    .enable_ctrlc_handler()
    .build()
}

pub use command::set_commands;
