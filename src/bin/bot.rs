use racer::telegram::command::{answer, RacerCommand};
use teloxide::{repls::CommandReplExt, Bot};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("fail to load env");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let bot = Bot::from_env();

    RacerCommand::repl(bot, answer).await;
}
