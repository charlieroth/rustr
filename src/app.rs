use std::time::Duration;

use nostr_relay_pool::relay::ReqExitPolicy;
use nostr_sdk::{
    serde_json, Client, EventBuilder, Filter, Keys, Kind, PublicKey, SecretKey, ToBech32,
};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: Option<String>,
    pub pubkey: Option<String>,
    pub npub: Option<String>,
}

pub struct App {
    pub config: Config,
    pub client: Client,
    pub keys: Keys,
}

impl App {
    pub fn new(config: Config) -> Self {
        let sk = SecretKey::parse(&config.nsec).unwrap();
        let keys = Keys::new(sk);
        Self {
            config,
            keys: keys.clone(),
            client: Client::new(keys),
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
        Ok(event_id)
    }

    pub async fn profile(&self, npub: Option<String>) -> anyhow::Result<Profile> {
        let public_key: PublicKey = if let Some(npub) = npub {
            PublicKey::parse(format!("nostr:{}", npub).as_str())?
        } else {
            self.keys.public_key()
        };

        let filter = Filter::new()
            .author(public_key)
            .kind(Kind::Metadata)
            .limit(1);
        let relay = self.client.relay(self.config.relays[0].clone()).await?;
        let events = relay
            .fetch_events(
                vec![filter],
                Duration::from_secs(10),
                ReqExitPolicy::ExitOnEOSE,
            )
            .await?;

        if events.is_empty() {
            return Err(anyhow::anyhow!("No profile found"));
        }

        let event = events.first().unwrap();
        let profile: Profile = serde_json::from_str(&event.content)?;
        Ok(profile)
    }
}
