use anyhow::Result;
use env_logger::Env;
use celery::beat::{CronSchedule};
use love_sender::tasks;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Initialize Celery Beat
    let mut beat = celery::beat!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap_or(String::from("redis://192.168.0.100:6379/0")) },
        tasks = [
            "send_greeting" => {
                tasks::send_greeting,
                schedule = CronSchedule::from_string("0 8 * * *")?,
                args = (,),
            }
        ],
        task_routes = [
            "*" => tasks::QUEUE_NAME
        ],
    ).await?;

    beat.start().await?;

    Ok(())
}
