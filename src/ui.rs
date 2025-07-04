use crate::errors::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UiElement {
    AiResponse {
        content: String,
        timestamp: std::time::Instant,
    },
    StatusBar {
        content: String,
    },
    Popup {
        title: String,
        content: String,
        visible: bool,
    },
    Button {
        id: String,
        text: String,
        position: (f32, f32),
        size: (f32, f32),
        enabled: bool,
    },
}

#[derive(Debug, Clone)]
pub struct UiLayout {
    pub terminal_area: Rectangle,
    pub ai_panel_area: Option<Rectangle>,
    pub status_bar_area: Rectangle,
    pub popup_area: Option<Rectangle>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && 
        y >= self.y && y <= self.y + self.height
    }
}

pub struct UiManager {
    elements: HashMap<String, UiElement>,
    layout: UiLayout,
    window_size: (f32, f32),
    ai_panel_visible: bool,
    popup_stack: Vec<String>,
}

impl UiManager {
    pub fn new() -> Self {
        let window_size = (1024.0, 768.0);
        let layout = Self::calculate_layout(window_size, false);
        
        Self {
            elements: HashMap::new(),
            layout,
            window_size,
            ai_panel_visible: false,
            popup_stack: Vec::new(),
        }
    }
    
    fn calculate_layout(window_size: (f32, f32), ai_panel_visible: bool) -> UiLayout {
        let (width, height) = window_size;
        let status_bar_height = 30.0;
        let ai_panel_width = if ai_panel_visible { 300.0 } else { 0.0 };
        
        let terminal_width = width - ai_panel_width;
        let terminal_height = height - status_bar_height;
        
        UiLayout {
            terminal_area: Rectangle::new(0.0, 0.0, terminal_width, terminal_height),
            ai_panel_area: if ai_panel_visible {
                Some(Rectangle::new(terminal_width, 0.0, ai_panel_width, terminal_height))
            } else {
                None
            },
            status_bar_area: Rectangle::new(0.0, terminal_height, width, status_bar_height),
            popup_area: None,
        }
    }
    
    pub fn resize(&mut self, new_size: (f32, f32)) {
        self.window_size = new_size;
        self.layout = Self::calculate_layout(new_size, self.ai_panel_visible);
    }
    
    pub fn toggle_ai_panel(&mut self) {
        self.ai_panel_visible = !self.ai_panel_visible;
        self.layout = Self::calculate_layout(self.window_size, self.ai_panel_visible);
    }
    
    pub fn show_ai_response(&mut self, content: String) {
        if !self.ai_panel_visible {
            self.toggle_ai_panel();
        }
        
        self.elements.insert(
            "ai_response".to_string(),
            UiElement::AiResponse {
                content,
                timestamp: std::time::Instant::now(),
            },
        );
    }
    
    pub fn update_status_bar(&mut self, content: String) {
        self.elements.insert(
            "status_bar".to_string(),
            UiElement::StatusBar { content },
        );
    }
    
    pub fn show_popup(&mut self, id: String, title: String, content: String) {
        let popup_width = 400.0;
        let popup_height = 300.0;
        let popup_x = (self.window_size.0 - popup_width) / 2.0;
        let popup_y = (self.window_size.1 - popup_height) / 2.0;
        
        self.layout.popup_area = Some(Rectangle::new(popup_x, popup_y, popup_width, popup_height));
        
        self.elements.insert(
            id.clone(),
            UiElement::Popup {
                title,
                content,
                visible: true,
            },
        );
        
        self.popup_stack.push(id);
    }
    
    pub fn hide_popup(&mut self, id: &str) {
        if let Some(element) = self.elements.get_mut(id) {
            if let UiElement::Popup { visible, .. } = element {
                *visible = false;
            }
        }
        
        self.popup_stack.retain(|popup_id| popup_id != id);
        
        if self.popup_stack.is_empty() {
            self.layout.popup_area = None;
        }
    }
    
    pub fn add_button(&mut self, id: String, text: String, position: (f32, f32), size: (f32, f32)) {
        self.elements.insert(
            id.clone(),
            UiElement::Button {
                id,
                text,
                position,
                size,
                enabled: true,
            },
        );
    }
    
    pub fn handle_click(&mut self, x: f32, y: f32) -> Option<String> {
        // Check if click is on a button
        for (id, element) in &self.elements {
            if let UiElement::Button { position, size, enabled, .. } = element {
                if *enabled {
                    let button_rect = Rectangle::new(position.0, position.1, size.0, size.1);
                    if button_rect.contains_point(x, y) {
                        return Some(id.clone());
                    }
                }
            }
        }
        
        // Check if click is outside popup (to close it)
        if let Some(popup_area) = &self.layout.popup_area {
            if !popup_area.contains_point(x, y) && !self.popup_stack.is_empty() {
                if let Some(top_popup) = self.popup_stack.last().cloned() {
                    self.hide_popup(&top_popup);
                }
            }
        }
        
        None
    }
    
    pub fn get_layout(&self) -> &UiLayout {
        &self.layout
    }
    
    pub fn get_elements(&self) -> &HashMap<String, UiElement> {
        &self.elements
    }
    
    pub fn is_ai_panel_visible(&self) -> bool {
        self.ai_panel_visible
    }
    
    pub fn get_terminal_area(&self) -> Rectangle {
        self.layout.terminal_area
    }
    
    pub fn get_ai_panel_area(&self) -> Option<Rectangle> {
        self.layout.ai_panel_area
    }
    
    pub fn get_status_bar_area(&self) -> Rectangle {
        self.layout.status_bar_area
    }
    
    pub fn clear_ai_responses(&mut self) {
        self.elements.retain(|_, element| {
            !matches!(element, UiElement::AiResponse { .. })
        });
    }
    
    pub fn set_button_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(UiElement::Button { enabled: button_enabled, .. }) = self.elements.get_mut(id) {
            *button_enabled = enabled;
        }
    }
} 