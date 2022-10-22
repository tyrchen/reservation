use std::time::Duration;

use sqlx::postgres::PgListener;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut listener = PgListener::connect(&url).await.unwrap();
    listener.listen("reservation_update").await.unwrap();
    println!("Listening for reservation_update events...");

    let stream = listener.into_stream();
    let stream = stream.throttle(Duration::from_secs(10));
    tokio::pin!(stream);
    while let Some(Ok(event)) = stream.next().await {
        println!("Received event: {:?}", event);
    }
}
