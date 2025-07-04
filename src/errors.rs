use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiminalError {
    #[error("Terminal error: {0}")]
    Terminal(String),
    
    #[error("Renderer error: {0}")]
    Renderer(String),
    
    #[error("Shell process error: {0}")]
    Shell(String),
    
    #[error("AI/Ollama error: {0}")]
    Ai(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Window creation error: {0}")]
    Window(String),
    
    #[error("WGPU error: {0}")]
    Wgpu(String),
}

pub type Result<T> = std::result::Result<T, LiminalError>; 