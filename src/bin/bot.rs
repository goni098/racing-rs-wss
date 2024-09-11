use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("fail to load env");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
