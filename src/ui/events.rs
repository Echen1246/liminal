use crate::errors::*;
use winit::event::{WindowEvent, KeyboardInput, MouseButton, ElementState};
use winit::dpi::PhysicalPosition;

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
            
            WindowEvent::KeyboardInput { input, .. } => {
                self.handle_keyboard_input(input)
            }
            
            WindowEvent::ReceivedCharacter(char) => {
                // Filter out control characters
                if !char.is_control() {
                    Some(UiEvent::TextInput {
                        text: char.to_string(),
                    })
                } else {
                    None
                }
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
    
    fn handle_keyboard_input(&mut self, input: &KeyboardInput) -> Option<UiEvent> {
        if input.state != ElementState::Pressed {
            return None;
        }
        
        // Update modifiers
        self.update_modifiers(input);
        
        // Convert virtual key code to string
        if let Some(key_string) = self.virtual_key_to_string(input.virtual_keycode) {
            Some(UiEvent::KeyPress {
                key: key_string,
                modifiers: self.modifiers.clone(),
            })
        } else {
            None
        }
    }
    
    fn update_modifiers(&mut self, input: &KeyboardInput) {
        // Note: This is a simplified approach
        // In a real implementation, you'd track modifier state more carefully
        use winit::event::VirtualKeyCode;
        
        match input.virtual_keycode {
            Some(VirtualKeyCode::LControl) | Some(VirtualKeyCode::RControl) => {
                self.modifiers.ctrl = input.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::LAlt) | Some(VirtualKeyCode::RAlt) => {
                self.modifiers.alt = input.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) => {
                self.modifiers.shift = input.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::LWin) | Some(VirtualKeyCode::RWin) => {
                self.modifiers.cmd = input.state == ElementState::Pressed;
            }
            _ => {}
        }
    }
    
    fn virtual_key_to_string(&self, key_code: Option<winit::event::VirtualKeyCode>) -> Option<String> {
        use winit::event::VirtualKeyCode;
        
        match key_code? {
            VirtualKeyCode::Return => Some("Enter".to_string()),
            VirtualKeyCode::Space => Some("Space".to_string()),
            VirtualKeyCode::Back => Some("Backspace".to_string()),
            VirtualKeyCode::Delete => Some("Delete".to_string()),
            VirtualKeyCode::Tab => Some("Tab".to_string()),
            VirtualKeyCode::Escape => Some("Escape".to_string()),
            VirtualKeyCode::Up => Some("ArrowUp".to_string()),
            VirtualKeyCode::Down => Some("ArrowDown".to_string()),
            VirtualKeyCode::Left => Some("ArrowLeft".to_string()),
            VirtualKeyCode::Right => Some("ArrowRight".to_string()),
            VirtualKeyCode::Home => Some("Home".to_string()),
            VirtualKeyCode::End => Some("End".to_string()),
            VirtualKeyCode::PageUp => Some("PageUp".to_string()),
            VirtualKeyCode::PageDown => Some("PageDown".to_string()),
            
            // Function keys
            VirtualKeyCode::F1 => Some("F1".to_string()),
            VirtualKeyCode::F2 => Some("F2".to_string()),
            VirtualKeyCode::F3 => Some("F3".to_string()),
            VirtualKeyCode::F4 => Some("F4".to_string()),
            VirtualKeyCode::F5 => Some("F5".to_string()),
            VirtualKeyCode::F6 => Some("F6".to_string()),
            VirtualKeyCode::F7 => Some("F7".to_string()),
            VirtualKeyCode::F8 => Some("F8".to_string()),
            VirtualKeyCode::F9 => Some("F9".to_string()),
            VirtualKeyCode::F10 => Some("F10".to_string()),
            VirtualKeyCode::F11 => Some("F11".to_string()),
            VirtualKeyCode::F12 => Some("F12".to_string()),
            
            // Letters
            VirtualKeyCode::A => Some("a".to_string()),
            VirtualKeyCode::B => Some("b".to_string()),
            VirtualKeyCode::C => Some("c".to_string()),
            VirtualKeyCode::D => Some("d".to_string()),
            VirtualKeyCode::E => Some("e".to_string()),
            VirtualKeyCode::F => Some("f".to_string()),
            VirtualKeyCode::G => Some("g".to_string()),
            VirtualKeyCode::H => Some("h".to_string()),
            VirtualKeyCode::I => Some("i".to_string()),
            VirtualKeyCode::J => Some("j".to_string()),
            VirtualKeyCode::K => Some("k".to_string()),
            VirtualKeyCode::L => Some("l".to_string()),
            VirtualKeyCode::M => Some("m".to_string()),
            VirtualKeyCode::N => Some("n".to_string()),
            VirtualKeyCode::O => Some("o".to_string()),
            VirtualKeyCode::P => Some("p".to_string()),
            VirtualKeyCode::Q => Some("q".to_string()),
            VirtualKeyCode::R => Some("r".to_string()),
            VirtualKeyCode::S => Some("s".to_string()),
            VirtualKeyCode::T => Some("t".to_string()),
            VirtualKeyCode::U => Some("u".to_string()),
            VirtualKeyCode::V => Some("v".to_string()),
            VirtualKeyCode::W => Some("w".to_string()),
            VirtualKeyCode::X => Some("x".to_string()),
            VirtualKeyCode::Y => Some("y".to_string()),
            VirtualKeyCode::Z => Some("z".to_string()),
            
            // Numbers
            VirtualKeyCode::Key1 => Some("1".to_string()),
            VirtualKeyCode::Key2 => Some("2".to_string()),
            VirtualKeyCode::Key3 => Some("3".to_string()),
            VirtualKeyCode::Key4 => Some("4".to_string()),
            VirtualKeyCode::Key5 => Some("5".to_string()),
            VirtualKeyCode::Key6 => Some("6".to_string()),
            VirtualKeyCode::Key7 => Some("7".to_string()),
            VirtualKeyCode::Key8 => Some("8".to_string()),
            VirtualKeyCode::Key9 => Some("9".to_string()),
            VirtualKeyCode::Key0 => Some("0".to_string()),
            
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