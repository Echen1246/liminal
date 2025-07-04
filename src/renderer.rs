use crate::{
    config::RendererConfig,
    errors::*,
    terminal::Terminal,
    ui::UiManager,
};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    _window: Arc<Window>,
    clear_color: [f32; 4],
}

impl Renderer {
    pub async fn new(window: Arc<Window>, renderer_config: &RendererConfig) -> Result<Self> {
        let clear_color = renderer_config.background_color;
        
        Ok(Self {
            _window: window,
            clear_color,
        })
    }
    
    pub fn resize(&mut self, _new_size: PhysicalSize<u32>) {
        // Window resizing will be handled by the window system for now
    }
    
    pub async fn render(&mut self, _terminal: &Terminal, _ui_manager: &UiManager) -> Result<()> {
        // For now, just log that we're rendering
        // In a full implementation, this would render terminal content and UI
        log::debug!("Rendering frame with background color: {:?}", self.clear_color);
        Ok(())
    }
} 