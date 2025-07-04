use liminal::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    println!("Default config created successfully");
    
    // Test serialization
    let toml_str = toml::to_string_pretty(&config)?;
    println!("\nSerialized config:");
    println!("{}", toml_str);
    
    // Test deserialization
    let parsed_config: Config = toml::from_str(&toml_str)?;
    println!("\nConfig parsed successfully!");
    
    Ok(())
} 