use grammers_client::{Client, Config};
use grammers_session::Session;
// use grammers_tl_types::enums::InputPeer;
use std::env;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub struct TelegramAPI {
    client: Client,
}

impl TelegramAPI {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_id = env::var("TELEGRAM_API_ID")?.parse()?;
        let api_hash = env::var("TELEGRAM_API_HASH")?;

        let client = Client::connect(Config {
            session: Session::load_file_or_create("session.session")?,
            api_id,
            api_hash,
            params: Default::default(),
        })
        .await?;

        if !client.is_authorized().await? {
            let phone_number = env::var("TELEGRAM_PHONE")?;
            let token = client.request_login_code(&phone_number).await?;
            println!("Enter the code you received via Telegram:");
            let mut code = String::new();
            std::io::stdin().read_line(&mut code)?;
            client.sign_in(&token, code.trim()).await?;
        }

        Ok(TelegramAPI { client })
    }

    pub async fn send_message(&self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self
            .client
            .resolve_username(env::var("TELEGRAM_RECEIVER")?.as_str())
            .await?
        {
            self.client.send_message(user, message.as_str()).await?;
        }
        Ok(())
    }
}

static TELEGRAM_CLIENT_INSTANCE: OnceCell<Arc<RwLock<TelegramAPI>>> = OnceCell::const_new();
pub async fn get_telegram_client() -> Arc<RwLock<TelegramAPI>> {
    TELEGRAM_CLIENT_INSTANCE
        .get_or_init(|| async { Arc::new(RwLock::new(TelegramAPI::new().await.unwrap())) })
        .await
        .clone()
}
