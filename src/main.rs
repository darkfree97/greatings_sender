use anyhow::Result;
use love_sender::tasks;
use love_sender::telegram::get_telegram_client;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Telegram API before using in Celery
    get_telegram_client().await;
    
    // Initialize Celery
    let app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap_or(String::from("redis://192.168.0.100:6379/0")) },
        tasks = [tasks::send_greeting],
        task_routes = [
            "*" => tasks::QUEUE_NAME
        ],
    ).await?;
    app.display_pretty().await;
    // app.send_task(tasks::send_greeting::new()).await?;
    app.consume_from(&[tasks::QUEUE_NAME]).await?;
    app.close().await?;
    Ok(())
}
