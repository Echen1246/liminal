pub mod app;
pub mod ai;
pub mod config;
pub mod errors;
pub mod renderer;
pub mod shell;
pub mod terminal;
pub mod ui;

// Re-export commonly used types
pub use errors::*;
pub use config::Config; 