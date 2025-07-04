use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub terminal: TerminalConfig,
    pub renderer: RendererConfig,
    pub ai: AiConfig,
    pub shell: ShellConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub rows: u32,
    pub cols: u32,
    pub scrollback_limit: usize,
    pub font_family: String,
    pub font_size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererConfig {
    pub vsync: bool,
    pub gpu_acceleration: bool,
    pub background_color: [f32; 4],
    pub text_color: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub ollama_base_url: String,
    pub model_name: String,
    pub context_length: usize,
    pub temperature: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub shell_command: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub environment_variables: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            terminal: TerminalConfig {
                rows: 24,
                cols: 80,
                scrollback_limit: 10000,
                font_family: "JetBrains Mono".to_string(),
                font_size: 14.0,
            },
            renderer: RendererConfig {
                vsync: true,
                gpu_acceleration: true,
                background_color: [0.1, 0.1, 0.1, 1.0],
                text_color: [0.9, 0.9, 0.9, 1.0],
            },
            ai: AiConfig {
                ollama_base_url: "http://localhost:11434".to_string(),
                model_name: "deepseek-r1:1.5b".to_string(),
                context_length: 4096,
                temperature: 0.7,
                enabled: true,
            },
            shell: ShellConfig {
                shell_command: None, // Auto-detect
                working_directory: None, // Use current directory
                environment_variables: std::collections::HashMap::new(),
            },
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| LiminalError::Config("Cannot find config directory".to_string()))?
            .join("liminal");
        
        let config_path = config_dir.join("config.toml");
        
        if config_path.exists() {
            let content = tokio::fs::read_to_string(&config_path).await
                .map_err(|e| LiminalError::Config(format!("Failed to read config file: {}", e)))?;
            
            let config: Config = toml::from_str(&content)
                .map_err(|e| LiminalError::Config(format!("Failed to parse config: {}", e)))?;
            
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save().await?;
            Ok(config)
        }
    }
    
    pub async fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| LiminalError::Config("Cannot find config directory".to_string()))?
            .join("liminal");
        
        tokio::fs::create_dir_all(&config_dir).await
            .map_err(|e| LiminalError::Config(format!("Failed to create config directory: {}", e)))?;
        
        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| LiminalError::Config(format!("Failed to serialize config: {}", e)))?;
        
        tokio::fs::write(&config_path, content).await
            .map_err(|e| LiminalError::Config(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }
} 