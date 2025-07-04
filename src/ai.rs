use crate::{config::AiConfig, errors::*};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    pub options: ChatOptions,
}

#[derive(Debug, Serialize)]
pub struct ChatOptions {
    pub temperature: f32,
    pub num_ctx: usize,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: ChatMessage,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: Option<u64>,
    pub digest: Option<String>,
    pub modified_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    pub models: Vec<ModelInfo>,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
    model_name: String,
    config: AiConfig,
}

impl OllamaClient {
    pub async fn new(config: &AiConfig) -> Result<Self> {
        let client = Client::new();
        
        let mut ollama_client = Self {
            client,
            base_url: config.ollama_base_url.clone(),
            model_name: config.model_name.clone(),
            config: config.clone(),
        };
        
        // Check if Ollama is running and available
        if config.enabled {
            ollama_client.ensure_ollama_available().await?;
        }
        
        Ok(ollama_client)
    }
    
    async fn ensure_ollama_available(&mut self) -> Result<()> {
        // Check if Ollama is installed
        if !self.is_ollama_installed().await {
            return Err(LiminalError::Ai(
                "Ollama is not installed. Please install Ollama from https://ollama.ai".to_string()
            ));
        }
        
        // Check if Ollama server is running
        if !self.is_ollama_running().await? {
            log::info!("Starting Ollama server...");
            self.start_ollama_server().await?;
        }
        
        // Check if the model is available
        if !self.is_model_available().await? {
            log::info!("Model '{}' not found. Pulling model...", self.model_name);
            self.pull_model().await?;
        }
        
        Ok(())
    }
    
    async fn is_ollama_installed(&self) -> bool {
        AsyncCommand::new("ollama")
            .arg("--version")
            .output()
            .await
            .is_ok()
    }
    
    async fn is_ollama_running(&self) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await;
        
        Ok(response.is_ok())
    }
    
    async fn start_ollama_server(&self) -> Result<()> {
        // Start Ollama server in the background
        let _child = AsyncCommand::new("ollama")
            .arg("serve")
            .spawn()
            .map_err(|e| LiminalError::Ai(format!("Failed to start Ollama server: {}", e)))?;
        
        // Wait a moment for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // Verify it's running
        for _ in 0..10 {
            if self.is_ollama_running().await? {
                return Ok(());
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Err(LiminalError::Ai("Failed to start Ollama server".to_string()))
    }
    
    async fn is_model_available(&self) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| LiminalError::Ai(format!("Failed to check available models: {}", e)))?;
        
        let models: ModelsResponse = response.json().await
            .map_err(|e| LiminalError::Ai(format!("Failed to parse models response: {}", e)))?;
        
        Ok(models.models.iter().any(|model| model.name.contains(&self.model_name)))
    }
    
    async fn pull_model(&self) -> Result<()> {
        let mut child = AsyncCommand::new("ollama")
            .arg("pull")
            .arg(&self.model_name)
            .spawn()
            .map_err(|e| LiminalError::Ai(format!("Failed to pull model: {}", e)))?;
        
        let status = child.wait().await
            .map_err(|e| LiminalError::Ai(format!("Failed to wait for model pull: {}", e)))?;
        
        if !status.success() {
            return Err(LiminalError::Ai(format!("Failed to pull model '{}'", self.model_name)));
        }
        
        Ok(())
    }
    
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String> {
        if !self.config.enabled {
            return Err(LiminalError::Ai("AI functionality is disabled".to_string()));
        }
        
        let request = ChatRequest {
            model: self.model_name.clone(),
            messages,
            stream: false,
            options: ChatOptions {
                temperature: self.config.temperature,
                num_ctx: self.config.context_length,
            },
        };
        
        let response = self.client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| LiminalError::Ai(format!("Failed to send chat request: {}", e)))?;
        
        let chat_response: ChatResponse = response.json().await
            .map_err(|e| LiminalError::Ai(format!("Failed to parse chat response: {}", e)))?;
        
        Ok(chat_response.message.content)
    }
    
    pub async fn generate_command(&self, description: &str) -> Result<String> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant that generates shell commands based on natural language descriptions. Only respond with the command, no explanations unless asked.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Generate a shell command for: {}", description),
            },
        ];
        
        self.chat(messages).await
    }
    
    pub async fn explain_output(&self, command: &str, output: &str) -> Result<String> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant that explains shell command output. Be concise but informative.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Explain the output of the command '{}': {}", command, output),
            },
        ];
        
        self.chat(messages).await
    }
    
    pub async fn answer_question(&self, question: &str, context: Option<&str>) -> Result<String> {
        let mut content = format!("Answer this question about shell/terminal usage: {}", question);
        
        if let Some(ctx) = context {
            content.push_str(&format!("\n\nContext: {}", ctx));
        }
        
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant that answers questions about shell commands, terminal usage, and system administration. Be practical and provide examples when helpful.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content,
            },
        ];
        
        self.chat(messages).await
    }
    
    pub async fn get_available_models(&self) -> Result<Vec<String>> {
        let response = self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| LiminalError::Ai(format!("Failed to fetch available models: {}", e)))?;
        
        let models: ModelsResponse = response.json().await
            .map_err(|e| LiminalError::Ai(format!("Failed to parse models response: {}", e)))?;
        
        Ok(models.models.into_iter().map(|model| model.name).collect())
    }
} 