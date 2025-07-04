pub mod widgets;
pub mod layout;
pub mod renderer;
pub mod events;
pub mod styling;

use crate::errors::*;
use winit::dpi::PhysicalSize;

pub use widgets::*;
pub use layout::*;
pub use renderer::*;
pub use events::*;
pub use styling::*;

/// Custom UI Manager - Not text grid based like traditional terminals
/// Inspired by Warp's approach: GPU-rendered widgets with modern UX
pub struct UiManager {
    pub widgets: Vec<Box<dyn Widget>>,
    pub layout_engine: LayoutEngine,
    pub theme: Theme,
    pub size: PhysicalSize<u32>,
    pub focused_widget: Option<usize>,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            layout_engine: LayoutEngine::new(),
            theme: Theme::default(),
            size: PhysicalSize::new(1024, 768),
            focused_widget: None,
        }
    }
    
    /// Add modern UI widgets (not terminal cells)
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
    
    /// Create Warp-style terminal blocks
    pub fn create_terminal_block(&mut self, output: &str, command: &str) -> Result<()> {
        let block = TerminalBlock::new(command.to_string(), output.to_string());
        self.add_widget(Box::new(block));
        Ok(())
    }
    
    /// Modern UI layout (not text positioning)
    pub fn layout(&mut self) -> Result<()> {
        self.layout_engine.calculate_layout(&mut self.widgets, self.size)?;
        Ok(())
    }
    
    /// Handle modern UI events (not just keyboard)
    pub fn handle_event(&mut self, event: UiEvent) -> Result<()> {
        match event {
            UiEvent::Click { x, y } => {
                // Find widget under cursor
                for (i, widget) in self.widgets.iter_mut().enumerate() {
                    if widget.bounds().contains_point(x, y) {
                        self.focused_widget = Some(i);
                        widget.handle_event(WidgetEvent::Click { x, y })?;
                        break;
                    }
                }
            }
            UiEvent::Hover { x, y } => {
                for widget in &mut self.widgets {
                    widget.handle_event(WidgetEvent::Hover { 
                        hovering: widget.bounds().contains_point(x, y) 
                    })?;
                }
            }
            UiEvent::Resize { width, height } => {
                self.size = PhysicalSize::new(width, height);
                self.layout()?;
            }
        }
        Ok(())
    }
    
    /// Get widgets for GPU rendering
    pub fn get_render_data(&self) -> Vec<&dyn Widget> {
        self.widgets.iter().map(|w| w.as_ref()).collect()
    }
} 