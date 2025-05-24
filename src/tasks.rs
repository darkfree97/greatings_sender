use celery::prelude::*;
use chrono::Utc;
use chrono::Datelike;
use crate::telegram::get_telegram_client;

pub const QUEUE_NAME: &str = "celery";
const GREETINGS: [&'static str; 10] = [
    "Я тебе кохаю!",
    "Ти моє сонечко!",
    "Думаю про тебе весь час!",
    "Мрію поскоріше зустрітися з тобою!",
    "Цілую 999999 разів!",
    "Хочу тебе обійняти поскоріше!",
    "Сумую за тобою!",
    "Ти для мене найдорожча!",
    "Без тебе світ втрачає барви!",
    "Ти наймиліша!"
];

#[celery::task]
pub async fn send_greeting() -> TaskResult<()> {
    let today = Utc::now().date_naive();
    let day_of_year = today.ordinal();
    let index = (day_of_year % 10) as usize;
    let telegram_client = get_telegram_client().await;
    let message = GREETINGS.get(index).unwrap_or(&"Привіт Світ!").to_string();
    let result = telegram_client.read().await.send_message(message).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(TaskError::UnexpectedError(e.to_string()))
        }
    }
}