use crate::errors::*;
use winit::dpi::PhysicalSize;
use std::collections::HashMap;

/// Modern UI widget trait - not terminal cell based
pub trait Widget {
    fn render(&self, renderer: &mut dyn WidgetRenderer) -> Result<()>;
    fn bounds(&self) -> Rect;
    fn handle_event(&mut self, event: WidgetEvent) -> Result<()>;
    fn set_bounds(&mut self, bounds: Rect);
    fn preferred_size(&self) -> PhysicalSize<u32>;
    fn widget_type(&self) -> WidgetType;
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && 
        y >= self.y && y <= self.y + self.height
    }
}

#[derive(Debug, Clone)]
pub enum WidgetType {
    TerminalBlock,
    Button,
    TextInput,
    AIPanel,
    StatusBar,
    ScrollView,
}

#[derive(Debug, Clone)]
pub enum WidgetEvent {
    Click { x: f32, y: f32 },
    Hover { hovering: bool },
    KeyPress { key: String },
    TextInput { text: String },
    Focus,
    Blur,
}

/// The star of the show - Warp-style terminal blocks
pub struct TerminalBlock {
    pub command: String,
    pub output: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: CommandStatus,
    pub bounds: Rect,
    pub is_expanded: bool,
    pub is_hovered: bool,
    pub ai_suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum CommandStatus {
    Running,
    Success,
    Error,
    Cancelled,
}

impl TerminalBlock {
    pub fn new(command: String, output: String) -> Self {
        Self {
            command,
            output,
            timestamp: chrono::Utc::now(),
            status: CommandStatus::Success,
            bounds: Rect::new(0.0, 0.0, 800.0, 200.0),
            is_expanded: true,
            is_hovered: false,
            ai_suggestions: Vec::new(),
        }
    }
    
    pub fn add_ai_suggestion(&mut self, suggestion: String) {
        self.ai_suggestions.push(suggestion);
    }
    
    pub fn toggle_expansion(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

impl Widget for TerminalBlock {
    fn render(&self, renderer: &mut dyn WidgetRenderer) -> Result<()> {
        renderer.render_terminal_block(self)
    }
    
    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }
    
    fn handle_event(&mut self, event: WidgetEvent) -> Result<()> {
        match event {
            WidgetEvent::Click { x: _, y: _ } => {
                self.toggle_expansion();
            }
            WidgetEvent::Hover { hovering } => {
                self.is_hovered = hovering;
            }
            _ => {}
        }
        Ok(())
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn preferred_size(&self) -> PhysicalSize<u32> {
        let height = if self.is_expanded { 200 } else { 50 };
        PhysicalSize::new(800, height)
    }
    
    fn widget_type(&self) -> WidgetType {
        WidgetType::TerminalBlock
    }
}

/// Modern button widget
pub struct Button {
    pub text: String,
    pub bounds: Rect,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub style: ButtonStyle,
    pub on_click: Option<Box<dyn Fn() -> Result<()>>>,
}

#[derive(Debug, Clone)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    AI,
}

impl Button {
    pub fn new(text: String, style: ButtonStyle) -> Self {
        Self {
            text,
            bounds: Rect::new(0.0, 0.0, 100.0, 32.0),
            is_hovered: false,
            is_pressed: false,
            style,
            on_click: None,
        }
    }
}

impl Widget for Button {
    fn render(&self, renderer: &mut dyn WidgetRenderer) -> Result<()> {
        renderer.render_button(self)
    }
    
    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }
    
    fn handle_event(&mut self, event: WidgetEvent) -> Result<()> {
        match event {
            WidgetEvent::Click { x: _, y: _ } => {
                if let Some(callback) = &self.on_click {
                    callback()?;
                }
            }
            WidgetEvent::Hover { hovering } => {
                self.is_hovered = hovering;
            }
            _ => {}
        }
        Ok(())
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn preferred_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(100, 32)
    }
    
    fn widget_type(&self) -> WidgetType {
        WidgetType::Button
    }
}

/// AI Panel widget (like Warp's AI assistant)
pub struct AIPanel {
    pub bounds: Rect,
    pub is_visible: bool,
    pub conversation: Vec<AIMessage>,
    pub input_text: String,
    pub is_thinking: bool,
}

#[derive(Debug, Clone)]
pub struct AIMessage {
    pub role: AIRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum AIRole {
    User,
    Assistant,
    System,
}

impl AIPanel {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0.0, 0.0, 400.0, 600.0),
            is_visible: false,
            conversation: Vec::new(),
            input_text: String::new(),
            is_thinking: false,
        }
    }
    
    pub fn add_message(&mut self, role: AIRole, content: String) {
        self.conversation.push(AIMessage {
            role,
            content,
            timestamp: chrono::Utc::now(),
        });
    }
    
    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Widget for AIPanel {
    fn render(&self, renderer: &mut dyn WidgetRenderer) -> Result<()> {
        if self.is_visible {
            renderer.render_ai_panel(self)
        } else {
            Ok(())
        }
    }
    
    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }
    
    fn handle_event(&mut self, event: WidgetEvent) -> Result<()> {
        match event {
            WidgetEvent::TextInput { text } => {
                self.input_text = text;
            }
            WidgetEvent::KeyPress { key } => {
                if key == "Enter" && !self.input_text.is_empty() {
                    let user_message = self.input_text.clone();
                    self.add_message(AIRole::User, user_message);
                    self.input_text.clear();
                    self.is_thinking = true;
                    // AI response would be handled elsewhere
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn preferred_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(400, 600)
    }
    
    fn widget_type(&self) -> WidgetType {
        WidgetType::AIPanel
    }
}

/// Widget renderer trait for GPU rendering
pub trait WidgetRenderer {
    fn render_terminal_block(&mut self, block: &TerminalBlock) -> Result<()>;
    fn render_button(&mut self, button: &Button) -> Result<()>;
    fn render_ai_panel(&mut self, panel: &AIPanel) -> Result<()>;
    fn render_text(&mut self, text: &str, bounds: Rect, style: TextStyle) -> Result<()>;
    fn render_rect(&mut self, bounds: Rect, style: RectStyle) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: [f32; 4],
    pub bold: bool,
    pub italic: bool,
}

#[derive(Debug, Clone)]
pub struct RectStyle {
    pub fill_color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_width: f32,
    pub border_radius: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0], // White
            bold: false,
            italic: false,
        }
    }
}

impl Default for RectStyle {
    fn default() -> Self {
        Self {
            fill_color: [0.2, 0.2, 0.2, 1.0], // Dark gray
            border_color: [0.4, 0.4, 0.4, 1.0], // Light gray
            border_width: 1.0,
            border_radius: 4.0,
        }
    }
} 