use crate::errors::*;
use std::collections::HashMap;

/// Modern theme system (not terminal colors)
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub animations: AnimationConfig,
    pub effects: EffectConfig,
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    // Background colors
    pub primary_bg: [f32; 4],
    pub secondary_bg: [f32; 4],
    pub tertiary_bg: [f32; 4],
    
    // Text colors
    pub primary_text: [f32; 4],
    pub secondary_text: [f32; 4],
    pub muted_text: [f32; 4],
    
    // Accent colors
    pub accent_primary: [f32; 4],
    pub accent_secondary: [f32; 4],
    pub accent_success: [f32; 4],
    pub accent_warning: [f32; 4],
    pub accent_error: [f32; 4],
    
    // Terminal-specific colors
    pub terminal_bg: [f32; 4],
    pub terminal_text: [f32; 4],
    pub terminal_cursor: [f32; 4],
    pub terminal_selection: [f32; 4],
    
    // UI element colors
    pub border_primary: [f32; 4],
    pub border_secondary: [f32; 4],
    pub shadow: [f32; 4],
    pub overlay: [f32; 4],
    
    // AI-specific colors
    pub ai_user_bubble: [f32; 4],
    pub ai_assistant_bubble: [f32; 4],
    pub ai_thinking: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family_primary: String,
    pub font_family_mono: String,
    pub font_family_ui: String,
    
    // Font sizes
    pub text_xs: f32,    // 12px
    pub text_sm: f32,    // 14px
    pub text_base: f32,  // 16px
    pub text_lg: f32,    // 18px
    pub text_xl: f32,    // 20px
    pub text_2xl: f32,   // 24px
    pub text_3xl: f32,   // 30px
    
    // Line heights
    pub leading_tight: f32,   // 1.25
    pub leading_normal: f32,  // 1.5
    pub leading_relaxed: f32, // 1.75
}

#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub xs: f32,    // 4px
    pub sm: f32,    // 8px
    pub base: f32,  // 16px
    pub lg: f32,    // 24px
    pub xl: f32,    // 32px
    pub xxl: f32,   // 48px
}

#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub duration_fast: f32,     // 150ms
    pub duration_normal: f32,   // 300ms
    pub duration_slow: f32,     // 500ms
    
    pub easing_ease_in: EasingFunction,
    pub easing_ease_out: EasingFunction,
    pub easing_ease_in_out: EasingFunction,
}

#[derive(Debug, Clone)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic(f32, f32, f32, f32), // Custom cubic bezier
}

#[derive(Debug, Clone)]
pub struct EffectConfig {
    pub shadow_sm: ShadowStyle,
    pub shadow_md: ShadowStyle,
    pub shadow_lg: ShadowStyle,
    pub border_radius_sm: f32,
    pub border_radius_md: f32,
    pub border_radius_lg: f32,
    pub blur_sm: f32,
    pub blur_md: f32,
    pub blur_lg: f32,
}

#[derive(Debug, Clone)]
pub struct ShadowStyle {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub spread_radius: f32,
    pub color: [f32; 4],
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl Theme {
    /// Modern dark theme (like Warp)
    pub fn dark_theme() -> Self {
        Self {
            name: "Dark".to_string(),
            colors: ColorScheme {
                // Background colors
                primary_bg: [0.1, 0.1, 0.1, 1.0],       // #1a1a1a
                secondary_bg: [0.15, 0.15, 0.15, 1.0],  // #262626
                tertiary_bg: [0.2, 0.2, 0.2, 1.0],      // #333333
                
                // Text colors
                primary_text: [1.0, 1.0, 1.0, 1.0],     // #ffffff
                secondary_text: [0.8, 0.8, 0.8, 1.0],   // #cccccc
                muted_text: [0.6, 0.6, 0.6, 1.0],       // #999999
                
                // Accent colors
                accent_primary: [0.3, 0.6, 1.0, 1.0],   // #4d9aff (blue)
                accent_secondary: [0.6, 0.4, 1.0, 1.0], // #9966ff (purple)
                accent_success: [0.2, 0.8, 0.4, 1.0],   // #33cc66 (green)
                accent_warning: [1.0, 0.7, 0.2, 1.0],   // #ffb333 (orange)
                accent_error: [1.0, 0.3, 0.3, 1.0],     // #ff4d4d (red)
                
                // Terminal-specific
                terminal_bg: [0.05, 0.05, 0.05, 1.0],   // #0d0d0d
                terminal_text: [0.9, 0.9, 0.9, 1.0],    // #e6e6e6
                terminal_cursor: [0.3, 0.6, 1.0, 1.0],  // #4d9aff
                terminal_selection: [0.3, 0.6, 1.0, 0.3], // #4d9aff with alpha
                
                // UI elements
                border_primary: [0.25, 0.25, 0.25, 1.0], // #404040
                border_secondary: [0.35, 0.35, 0.35, 1.0], // #595959
                shadow: [0.0, 0.0, 0.0, 0.5],            // Black with alpha
                overlay: [0.0, 0.0, 0.0, 0.7],           // Dark overlay
                
                // AI colors
                ai_user_bubble: [0.3, 0.6, 1.0, 0.1],   // Light blue
                ai_assistant_bubble: [0.25, 0.25, 0.25, 1.0], // Gray
                ai_thinking: [0.6, 0.4, 1.0, 0.8],      // Purple
            },
            typography: Typography::default(),
            spacing: SpacingScale::default(),
            animations: AnimationConfig::default(),
            effects: EffectConfig::default(),
        }
    }
    
    /// Light theme option
    pub fn light_theme() -> Self {
        let mut theme = Self::dark_theme();
        theme.name = "Light".to_string();
        
        // Invert background and text colors
        theme.colors.primary_bg = [1.0, 1.0, 1.0, 1.0];      // White
        theme.colors.secondary_bg = [0.95, 0.95, 0.95, 1.0]; // Light gray
        theme.colors.tertiary_bg = [0.9, 0.9, 0.9, 1.0];     // Medium gray
        
        theme.colors.primary_text = [0.1, 0.1, 0.1, 1.0];    // Dark text
        theme.colors.secondary_text = [0.3, 0.3, 0.3, 1.0];  // Medium text
        theme.colors.muted_text = [0.5, 0.5, 0.5, 1.0];      // Light text
        
        theme.colors.terminal_bg = [0.98, 0.98, 0.98, 1.0];  // Almost white
        theme.colors.terminal_text = [0.1, 0.1, 0.1, 1.0];   // Dark text
        
        theme
    }
    
    /// Get a color by name for easy access
    pub fn color(&self, name: &str) -> [f32; 4] {
        match name {
            "primary_bg" => self.colors.primary_bg,
            "secondary_bg" => self.colors.secondary_bg,
            "tertiary_bg" => self.colors.tertiary_bg,
            "primary_text" => self.colors.primary_text,
            "secondary_text" => self.colors.secondary_text,
            "muted_text" => self.colors.muted_text,
            "accent_primary" => self.colors.accent_primary,
            "accent_secondary" => self.colors.accent_secondary,
            "accent_success" => self.colors.accent_success,
            "accent_warning" => self.colors.accent_warning,
            "accent_error" => self.colors.accent_error,
            "terminal_bg" => self.colors.terminal_bg,
            "terminal_text" => self.colors.terminal_text,
            "terminal_cursor" => self.colors.terminal_cursor,
            "ai_user_bubble" => self.colors.ai_user_bubble,
            "ai_assistant_bubble" => self.colors.ai_assistant_bubble,
            _ => [1.0, 0.0, 1.0, 1.0], // Magenta for missing colors
        }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family_primary: "Inter".to_string(),
            font_family_mono: "JetBrains Mono".to_string(),
            font_family_ui: "SF Pro Display".to_string(),
            
            text_xs: 12.0,
            text_sm: 14.0,
            text_base: 16.0,
            text_lg: 18.0,
            text_xl: 20.0,
            text_2xl: 24.0,
            text_3xl: 30.0,
            
            leading_tight: 1.25,
            leading_normal: 1.5,
            leading_relaxed: 1.75,
        }
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            base: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration_fast: 150.0,
            duration_normal: 300.0,
            duration_slow: 500.0,
            
            easing_ease_in: EasingFunction::EaseIn,
            easing_ease_out: EasingFunction::EaseOut,
            easing_ease_in_out: EasingFunction::EaseInOut,
        }
    }
}

impl Default for EffectConfig {
    fn default() -> Self {
        Self {
            shadow_sm: ShadowStyle {
                offset_x: 0.0,
                offset_y: 1.0,
                blur_radius: 2.0,
                spread_radius: 0.0,
                color: [0.0, 0.0, 0.0, 0.1],
            },
            shadow_md: ShadowStyle {
                offset_x: 0.0,
                offset_y: 4.0,
                blur_radius: 8.0,
                spread_radius: 0.0,
                color: [0.0, 0.0, 0.0, 0.15],
            },
            shadow_lg: ShadowStyle {
                offset_x: 0.0,
                offset_y: 10.0,
                blur_radius: 20.0,
                spread_radius: 0.0,
                color: [0.0, 0.0, 0.0, 0.2],
            },
            border_radius_sm: 4.0,
            border_radius_md: 8.0,
            border_radius_lg: 12.0,
            blur_sm: 4.0,
            blur_md: 8.0,
            blur_lg: 16.0,
        }
    }
}

/// Theme manager for switching between themes
pub struct ThemeManager {
    pub current_theme: Theme,
    pub available_themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("Dark".to_string(), Theme::dark_theme());
        themes.insert("Light".to_string(), Theme::light_theme());
        
        Self {
            current_theme: Theme::dark_theme(),
            available_themes: themes,
        }
    }
    
    pub fn switch_theme(&mut self, theme_name: &str) -> Result<()> {
        if let Some(theme) = self.available_themes.get(theme_name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(crate::errors::LiminalError::Config(
                format!("Theme '{}' not found", theme_name)
            ))
        }
    }
    
    pub fn add_theme(&mut self, theme: Theme) {
        self.available_themes.insert(theme.name.clone(), theme);
    }
    
    pub fn list_themes(&self) -> Vec<String> {
        self.available_themes.keys().cloned().collect()
    }
}

/// Animation state for widgets
#[derive(Debug, Clone)]
pub struct AnimationState {
    pub start_time: std::time::Instant,
    pub duration: f32,
    pub easing: EasingFunction,
    pub from_value: f32,
    pub to_value: f32,
    pub current_value: f32,
    pub is_complete: bool,
}

impl AnimationState {
    pub fn new(from: f32, to: f32, duration: f32, easing: EasingFunction) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            duration,
            easing,
            from_value: from,
            to_value: to,
            current_value: from,
            is_complete: false,
        }
    }
    
    pub fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let progress = (elapsed / self.duration).clamp(0.0, 1.0);
        
        if progress >= 1.0 {
            self.current_value = self.to_value;
            self.is_complete = true;
        } else {
            let eased_progress = self.apply_easing(progress);
            self.current_value = self.from_value + (self.to_value - self.from_value) * eased_progress;
        }
    }
    
    fn apply_easing(&self, t: f32) -> f32 {
        match self.easing {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            EasingFunction::Cubic(_, _, _, _) => {
                // Simplified cubic bezier approximation
                t * t * (3.0 - 2.0 * t)
            }
        }
    }
} 