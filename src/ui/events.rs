use crate::errors::*;
use winit::event::{WindowEvent, MouseButton, ElementState};
use winit::dpi::PhysicalPosition;
use winit::keyboard::{KeyCode, PhysicalKey};

/// High-level UI events (not terminal-specific)
#[derive(Debug, Clone)]
pub enum UiEvent {
    Click { x: f32, y: f32 },
    DoubleClick { x: f32, y: f32 },
    RightClick { x: f32, y: f32 },
    Hover { x: f32, y: f32 },
    Drag { start_x: f32, start_y: f32, x: f32, y: f32 },
    Scroll { x: f32, y: f32, delta_x: f32, delta_y: f32 },
    KeyPress { key: String, modifiers: Modifiers },
    TextInput { text: String },
    Resize { width: u32, height: u32 },
    Focus,
    Blur,
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool, // Meta key on Mac
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: false,
            cmd: false,
        }
    }
}

/// Event handler for converting winit events to UI events
pub struct EventHandler {
    pub mouse_position: PhysicalPosition<f64>,
    pub mouse_pressed: bool,
    pub last_click_time: std::time::Instant,
    pub click_count: u32,
    pub drag_start: Option<PhysicalPosition<f64>>,
    pub modifiers: Modifiers,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            mouse_position: PhysicalPosition::new(0.0, 0.0),
            mouse_pressed: false,
            last_click_time: std::time::Instant::now(),
            click_count: 0,
            drag_start: None,
            modifiers: Modifiers::default(),
        }
    }
    
    /// Convert winit window events to high-level UI events
    pub fn handle_window_event(&mut self, event: &WindowEvent) -> Option<UiEvent> {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let old_pos = self.mouse_position;
                self.mouse_position = *position;
                
                if self.mouse_pressed {
                    if let Some(drag_start) = self.drag_start {
                        Some(UiEvent::Drag {
                            start_x: drag_start.x as f32,
                            start_y: drag_start.y as f32,
                            x: position.x as f32,
                            y: position.y as f32,
                        })
                    } else {
                        None
                    }
                } else {
                    Some(UiEvent::Hover {
                        x: position.x as f32,
                        y: position.y as f32,
                    })
                }
            }
            
            WindowEvent::MouseInput { state, button, .. } => {
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        self.mouse_pressed = true;
                        self.drag_start = Some(self.mouse_position);
                        
                        // Detect double clicks
                        let now = std::time::Instant::now();
                        if now.duration_since(self.last_click_time).as_millis() < 300 {
                            self.click_count += 1;
                        } else {
                            self.click_count = 1;
                        }
                        self.last_click_time = now;
                        
                        if self.click_count == 2 {
                            self.click_count = 0; // Reset for next sequence
                            Some(UiEvent::DoubleClick {
                                x: self.mouse_position.x as f32,
                                y: self.mouse_position.y as f32,
                            })
                        } else {
                            Some(UiEvent::Click {
                                x: self.mouse_position.x as f32,
                                y: self.mouse_position.y as f32,
                            })
                        }
                    }
                    
                    (ElementState::Released, MouseButton::Left) => {
                        self.mouse_pressed = false;
                        self.drag_start = None;
                        None // We already sent the click event on press
                    }
                    
                    (ElementState::Pressed, MouseButton::Right) => {
                        Some(UiEvent::RightClick {
                            x: self.mouse_position.x as f32,
                            y: self.mouse_position.y as f32,
                        })
                    }
                    
                    _ => None,
                }
            }
            
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        Some(UiEvent::Scroll {
                            x: self.mouse_position.x as f32,
                            y: self.mouse_position.y as f32,
                            delta_x: *x * 10.0, // Scale line delta to pixels
                            delta_y: *y * 10.0,
                        })
                    }
                    winit::event::MouseScrollDelta::PixelDelta(delta) => {
                        Some(UiEvent::Scroll {
                            x: self.mouse_position.x as f32,
                            y: self.mouse_position.y as f32,
                            delta_x: delta.x as f32,
                            delta_y: delta.y as f32,
                        })
                    }
                }
            }
            
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event)
            }
            
            WindowEvent::Resized(size) => {
                Some(UiEvent::Resize {
                    width: size.width,
                    height: size.height,
                })
            }
            
            WindowEvent::Focused(focused) => {
                if *focused {
                    Some(UiEvent::Focus)
                } else {
                    Some(UiEvent::Blur)
                }
            }
            
            _ => None,
        }
    }
    
    fn handle_keyboard_input(&mut self, event: &winit::event::KeyEvent) -> Option<UiEvent> {
        if event.state != ElementState::Pressed {
            return None;
        }
        
        // Update modifiers
        self.update_modifiers(event);
        
        // Convert key code to string
        if let Some(key_string) = self.key_code_to_string(&event.physical_key) {
            Some(UiEvent::KeyPress {
                key: key_string,
                modifiers: self.modifiers.clone(),
            })
        } else {
            None
        }
    }
    
    fn update_modifiers(&mut self, event: &winit::event::KeyEvent) {
        // Update modifiers based on the key event
        // This is a simplified approach - in practice you'd track modifier state more carefully
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ControlLeft) | PhysicalKey::Code(KeyCode::ControlRight) => {
                self.modifiers.ctrl = event.state == ElementState::Pressed;
            }
            PhysicalKey::Code(KeyCode::AltLeft) | PhysicalKey::Code(KeyCode::AltRight) => {
                self.modifiers.alt = event.state == ElementState::Pressed;
            }
            PhysicalKey::Code(KeyCode::ShiftLeft) | PhysicalKey::Code(KeyCode::ShiftRight) => {
                self.modifiers.shift = event.state == ElementState::Pressed;
            }
            PhysicalKey::Code(KeyCode::SuperLeft) | PhysicalKey::Code(KeyCode::SuperRight) => {
                self.modifiers.cmd = event.state == ElementState::Pressed;
            }
            _ => {}
        }
    }
    
    fn key_code_to_string(&self, key: &PhysicalKey) -> Option<String> {
        match key {
            PhysicalKey::Code(code) => {
                match code {
                    KeyCode::Enter => Some("Enter".to_string()),
                    KeyCode::Space => Some("Space".to_string()),
                    KeyCode::Backspace => Some("Backspace".to_string()),
                    KeyCode::Delete => Some("Delete".to_string()),
                    KeyCode::Tab => Some("Tab".to_string()),
                    KeyCode::Escape => Some("Escape".to_string()),
                    KeyCode::ArrowUp => Some("ArrowUp".to_string()),
                    KeyCode::ArrowDown => Some("ArrowDown".to_string()),
                    KeyCode::ArrowLeft => Some("ArrowLeft".to_string()),
                    KeyCode::ArrowRight => Some("ArrowRight".to_string()),
                    KeyCode::Home => Some("Home".to_string()),
                    KeyCode::End => Some("End".to_string()),
                    KeyCode::PageUp => Some("PageUp".to_string()),
                    KeyCode::PageDown => Some("PageDown".to_string()),
                    
                    // Function keys
                    KeyCode::F1 => Some("F1".to_string()),
                    KeyCode::F2 => Some("F2".to_string()),
                    KeyCode::F3 => Some("F3".to_string()),
                    KeyCode::F4 => Some("F4".to_string()),
                    KeyCode::F5 => Some("F5".to_string()),
                    KeyCode::F6 => Some("F6".to_string()),
                    KeyCode::F7 => Some("F7".to_string()),
                    KeyCode::F8 => Some("F8".to_string()),
                    KeyCode::F9 => Some("F9".to_string()),
                    KeyCode::F10 => Some("F10".to_string()),
                    KeyCode::F11 => Some("F11".to_string()),
                    KeyCode::F12 => Some("F12".to_string()),
                    
                    // Letters
                    KeyCode::KeyA => Some("a".to_string()),
                    KeyCode::KeyB => Some("b".to_string()),
                    KeyCode::KeyC => Some("c".to_string()),
                    KeyCode::KeyD => Some("d".to_string()),
                    KeyCode::KeyE => Some("e".to_string()),
                    KeyCode::KeyF => Some("f".to_string()),
                    KeyCode::KeyG => Some("g".to_string()),
                    KeyCode::KeyH => Some("h".to_string()),
                    KeyCode::KeyI => Some("i".to_string()),
                    KeyCode::KeyJ => Some("j".to_string()),
                    KeyCode::KeyK => Some("k".to_string()),
                    KeyCode::KeyL => Some("l".to_string()),
                    KeyCode::KeyM => Some("m".to_string()),
                    KeyCode::KeyN => Some("n".to_string()),
                    KeyCode::KeyO => Some("o".to_string()),
                    KeyCode::KeyP => Some("p".to_string()),
                    KeyCode::KeyQ => Some("q".to_string()),
                    KeyCode::KeyR => Some("r".to_string()),
                    KeyCode::KeyS => Some("s".to_string()),
                    KeyCode::KeyT => Some("t".to_string()),
                    KeyCode::KeyU => Some("u".to_string()),
                    KeyCode::KeyV => Some("v".to_string()),
                    KeyCode::KeyW => Some("w".to_string()),
                    KeyCode::KeyX => Some("x".to_string()),
                    KeyCode::KeyY => Some("y".to_string()),
                    KeyCode::KeyZ => Some("z".to_string()),
                    
                    // Numbers
                    KeyCode::Digit1 => Some("1".to_string()),
                    KeyCode::Digit2 => Some("2".to_string()),
                    KeyCode::Digit3 => Some("3".to_string()),
                    KeyCode::Digit4 => Some("4".to_string()),
                    KeyCode::Digit5 => Some("5".to_string()),
                    KeyCode::Digit6 => Some("6".to_string()),
                    KeyCode::Digit7 => Some("7".to_string()),
                    KeyCode::Digit8 => Some("8".to_string()),
                    KeyCode::Digit9 => Some("9".to_string()),
                    KeyCode::Digit0 => Some("0".to_string()),
                    
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

/// Event dispatching and handling
pub struct EventDispatcher {
    pub handlers: Vec<Box<dyn EventHandlerTrait>>,
}

pub trait EventHandlerTrait {
    fn handle_event(&mut self, event: &UiEvent) -> Result<bool>; // Return true if handled
    fn priority(&self) -> i32; // Higher priority handlers get events first
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
    
    pub fn add_handler(&mut self, handler: Box<dyn EventHandlerTrait>) {
        self.handlers.push(handler);
        // Sort by priority (highest first)
        self.handlers.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }
    
    pub fn dispatch_event(&mut self, event: UiEvent) -> Result<()> {
        for handler in &mut self.handlers {
            if handler.handle_event(&event)? {
                break; // Event was handled, stop propagation
            }
        }
        Ok(())
    }
} 