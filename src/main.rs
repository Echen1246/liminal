use anyhow::Result;
use env_logger::Env;
use liminal::app::App;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    info!("Starting Liminal Terminal Emulator");
    
    // Create and run the application
    let mut app = App::new().await?;
    app.run().await?;
    
    Ok(())
} 