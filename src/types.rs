use std::error::Error;

use teloxide::{
    adaptors::{CacheMe, Throttle},
    dispatching::UpdateHandler,
    Bot,
};

pub type BoxedError = Box<dyn Error + Send + Sync>;
pub type HandlerType = UpdateHandler<BoxedError>;
pub type HandlerResult = Result<(), BoxedError>;
pub type BotType = CacheMe<Throttle<Bot>>;
