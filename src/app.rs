use nostr_sdk::{Client, EventBuilder, Keys, SecretKey, ToBech32};

use crate::config::Config;

pub struct App {
    pub config: Config,
    pub keys: Keys,
    pub client: Client,
}

impl App {
    pub fn new(config: Config) -> Self {
        let sk = SecretKey::parse(&config.nsec).unwrap();
        let keys = Keys::new(sk);
        Self {
            config,
            keys: keys.clone(),
            client: Client::new(keys.clone()),
        }
    }

    pub async fn connect(&self) -> anyhow::Result<()> {
        for relay in self.config.relays.iter() {
            self.client.add_relay(relay).await?;
        }
        self.client.connect().await;
        Ok(())
    }

    pub async fn publish_text_note(&self, message: &str) -> anyhow::Result<String> {
        let builder = EventBuilder::text_note(message);
        let output = self.client.send_event_builder(builder).await?;
        let event_id = output.id().to_bech32()?;
        println!("Success: {:?}", output.success);
        println!("Note sent to: {:?}", output.failed);
        Ok(event_id)
    }
}
