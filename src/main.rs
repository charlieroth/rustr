use clap::Parser;
mod app;
mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load("config.yaml")?;
    let app = app::App::new(config);
    app.connect().await?;

    let args = cli::CliArgs::parse();
    match args.cmd {
        cli::CliCommand::Post { message } => match app.publish_text_note(&message).await {
            Ok(event_id) => {
                println!("Posted message with event id: {}", event_id);
                Ok(())
            }
            Err(e) => anyhow::bail!(e),
        },
        cli::CliCommand::Profile { npub } => match app.profile(npub).await {
            Ok(profile) => {
                println!("{:?}", profile);
                Ok(())
            }
            Err(e) => anyhow::bail!(e),
        },
    }
}
