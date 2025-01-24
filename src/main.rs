mod app;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let message = args.get(1).expect("No message provided");

    let config = config::Config::load("config.yaml").unwrap();
    let app = app::App::new(config);
    println!("application config: {:?}", app.config);
    println!("application keys: {:?}", app.keys);

    app.connect().await?;
    let event_id = app.publish_text_note(message).await?;
    println!("Event ID: {}", event_id);
    Ok(())
}
