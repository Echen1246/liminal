use liminal::ai::{ChatMessage, OllamaClient};
use liminal::config::{AiConfig, Config};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("Liminal Terminal - Ollama AI Integration Example");
    println!("================================================\n");
    
    // Load configuration
    let config = Config::load().await?;
    
    // Create Ollama client
    let ollama_client = match OllamaClient::new(&config.ai).await {
        Ok(client) => {
            println!("✓ Successfully connected to Ollama at {}", config.ai.ollama_base_url);
            client
        }
        Err(e) => {
            eprintln!("✗ Failed to connect to Ollama: {}", e);
            eprintln!("Make sure Ollama is installed and running:");
            eprintln!("  1. Install Ollama from https://ollama.ai");
            eprintln!("  2. Run: ollama serve");
            eprintln!("  3. Pull a model: ollama pull {}", config.ai.model_name);
            return Err(e.into());
        }
    };
    
    // List available models
    match ollama_client.get_available_models().await {
        Ok(models) => {
            println!("Available models:");
            for model in models {
                println!("  - {}", model);
            }
            println!();
        }
        Err(e) => {
            eprintln!("Warning: Could not list models: {}", e);
        }
    }
    
    println!("Type your questions or commands (type 'quit' to exit):");
    println!("Examples:");
    println!("  - 'generate: list all files in current directory'");
    println!("  - 'explain: ls -la output'");
    println!("  - 'question: what does the grep command do?'");
    println!();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }
        
        // Parse input to determine the type of request
        let response = if input.starts_with("generate:") {
            let description = input.strip_prefix("generate:").unwrap().trim();
            println!("🤖 Generating command for: {}", description);
            ollama_client.generate_command(description).await
        } else if input.starts_with("explain:") {
            let output = input.strip_prefix("explain:").unwrap().trim();
            println!("🤖 Explaining output: {}", output);
            ollama_client.explain_output("command", output).await
        } else if input.starts_with("question:") {
            let question = input.strip_prefix("question:").unwrap().trim();
            println!("🤖 Answering question: {}", question);
            ollama_client.answer_question(question, None).await
        } else {
            // Default to answering as a question
            println!("🤖 Processing: {}", input);
            ollama_client.answer_question(input, None).await
        };
        
        match response {
            Ok(answer) => {
                println!("📝 Response:");
                println!("{}", answer);
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
        
        println!(); // Add spacing
    }
    
    Ok(())
} 