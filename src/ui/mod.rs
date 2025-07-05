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
            UiEvent::DoubleClick { x, y } => {
                // Handle double click events
                for widget in &mut self.widgets {
                    if widget.bounds().contains_point(x, y) {
                        widget.handle_event(WidgetEvent::DoubleClick { x, y })?;
                        break;
                    }
                }
            }
            UiEvent::RightClick { x, y } => {
                // Handle right click events (context menu)
                for widget in &mut self.widgets {
                    if widget.bounds().contains_point(x, y) {
                        widget.handle_event(WidgetEvent::RightClick { x, y })?;
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
            UiEvent::Drag { start_x, start_y, x, y } => {
                // Handle drag events
                for widget in &mut self.widgets {
                    if widget.bounds().contains_point(start_x, start_y) {
                        widget.handle_event(WidgetEvent::Drag { start_x, start_y, x, y })?;
                        break;
                    }
                }
            }
            UiEvent::Scroll { x, y, delta_x, delta_y } => {
                // Handle scroll events
                for widget in &mut self.widgets {
                    if widget.bounds().contains_point(x, y) {
                        widget.handle_event(WidgetEvent::Scroll { x, y, delta_x, delta_y })?;
                        break;
                    }
                }
            }
            UiEvent::KeyPress { key, modifiers } => {
                // Send key press to focused widget
                if let Some(focused_idx) = self.focused_widget {
                    if let Some(widget) = self.widgets.get_mut(focused_idx) {
                        widget.handle_event(WidgetEvent::KeyPress { key, modifiers })?;
                    }
                }
            }
            UiEvent::TextInput { text } => {
                // Send text input to focused widget
                if let Some(focused_idx) = self.focused_widget {
                    if let Some(widget) = self.widgets.get_mut(focused_idx) {
                        widget.handle_event(WidgetEvent::TextInput { text })?;
                    }
                }
            }
            UiEvent::Resize { width, height } => {
                self.size = PhysicalSize::new(width, height);
                self.layout()?;
            }
            UiEvent::Focus => {
                // Handle focus events
                // For now, just update internal state
            }
            UiEvent::Blur => {
                // Handle blur events
                self.focused_widget = None;
            }
        }
        Ok(())
    }
    
    /// Get widgets for GPU rendering
    pub fn get_render_data(&self) -> Vec<&dyn Widget> {
        self.widgets.iter().map(|w| w.as_ref()).collect()
    }
} 