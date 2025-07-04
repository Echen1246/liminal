use crate::errors::*;
use crate::ui::widgets::*;
use winit::dpi::PhysicalSize;

/// Modern layout engine (not terminal row/col based)
pub struct LayoutEngine {
    pub layout_type: LayoutType,
    pub padding: Padding,
    pub spacing: f32,
}

#[derive(Debug, Clone)]
pub enum LayoutType {
    Vertical,
    Horizontal,
    Grid { columns: u32, rows: u32 },
    Absolute,
    Flex,
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
    
    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
    
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::all(8.0)
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            layout_type: LayoutType::Vertical,
            padding: Padding::default(),
            spacing: 8.0,
        }
    }
    
    pub fn with_layout_type(mut self, layout_type: LayoutType) -> Self {
        self.layout_type = layout_type;
        self
    }
    
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
    
    /// Calculate layout for all widgets (modern UI positioning)
    pub fn calculate_layout(&self, widgets: &mut [Box<dyn Widget>], container_size: PhysicalSize<u32>) -> Result<()> {
        match self.layout_type {
            LayoutType::Vertical => self.layout_vertical(widgets, container_size),
            LayoutType::Horizontal => self.layout_horizontal(widgets, container_size),
            LayoutType::Grid { columns, rows } => self.layout_grid(widgets, container_size, columns, rows),
            LayoutType::Absolute => self.layout_absolute(widgets, container_size),
            LayoutType::Flex => self.layout_flex(widgets, container_size),
        }
    }
    
    fn layout_vertical(&self, widgets: &mut [Box<dyn Widget>], container_size: PhysicalSize<u32>) -> Result<()> {
        let mut y = self.padding.top;
        let available_width = container_size.width as f32 - self.padding.left - self.padding.right;
        
        for widget in widgets {
            let preferred_size = widget.preferred_size();
            let height = preferred_size.height as f32;
            
            widget.set_bounds(Rect::new(
                self.padding.left,
                y,
                available_width,
                height,
            ));
            
            y += height + self.spacing;
        }
        
        Ok(())
    }
    
    fn layout_horizontal(&self, widgets: &mut [Box<dyn Widget>], container_size: PhysicalSize<u32>) -> Result<()> {
        let mut x = self.padding.left;
        let available_height = container_size.height as f32 - self.padding.top - self.padding.bottom;
        
        for widget in widgets {
            let preferred_size = widget.preferred_size();
            let width = preferred_size.width as f32;
            
            widget.set_bounds(Rect::new(
                x,
                self.padding.top,
                width,
                available_height,
            ));
            
            x += width + self.spacing;
        }
        
        Ok(())
    }
    
    fn layout_grid(&self, widgets: &mut [Box<dyn Widget>], container_size: PhysicalSize<u32>, columns: u32, rows: u32) -> Result<()> {
        let available_width = container_size.width as f32 - self.padding.left - self.padding.right;
        let available_height = container_size.height as f32 - self.padding.top - self.padding.bottom;
        
        let cell_width = (available_width - (columns - 1) as f32 * self.spacing) / columns as f32;
        let cell_height = (available_height - (rows - 1) as f32 * self.spacing) / rows as f32;
        
        for (i, widget) in widgets.iter_mut().enumerate() {
            let col = i as u32 % columns;
            let row = i as u32 / columns;
            
            if row >= rows {
                break; // Don't layout widgets beyond the grid
            }
            
            let x = self.padding.left + col as f32 * (cell_width + self.spacing);
            let y = self.padding.top + row as f32 * (cell_height + self.spacing);
            
            widget.set_bounds(Rect::new(x, y, cell_width, cell_height));
        }
        
        Ok(())
    }
    
    fn layout_absolute(&self, widgets: &mut [Box<dyn Widget>], _container_size: PhysicalSize<u32>) -> Result<()> {
        // Absolute layout doesn't change widget positions
        // Widgets maintain their manually set bounds
        Ok(())
    }
    
    fn layout_flex(&self, widgets: &mut [Box<dyn Widget>], container_size: PhysicalSize<u32>) -> Result<()> {
        // Simplified flexbox-like layout
        let available_width = container_size.width as f32 - self.padding.left - self.padding.right;
        let available_height = container_size.height as f32 - self.padding.top - self.padding.bottom;
        
        // Calculate total preferred sizes
        let total_preferred_width: f32 = widgets.iter()
            .map(|w| w.preferred_size().width as f32)
            .sum();
        
        let total_spacing = (widgets.len() as f32 - 1.0) * self.spacing;
        let scale_factor = (available_width - total_spacing) / total_preferred_width;
        
        let mut x = self.padding.left;
        
        for widget in widgets {
            let preferred_size = widget.preferred_size();
            let width = preferred_size.width as f32 * scale_factor;
            
            widget.set_bounds(Rect::new(
                x,
                self.padding.top,
                width,
                available_height,
            ));
            
            x += width + self.spacing;
        }
        
        Ok(())
    }
}

/// Layout constraints for widgets
#[derive(Debug, Clone)]
pub struct LayoutConstraints {
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_self: Alignment,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            align_self: Alignment::Stretch,
        }
    }
}

/// Helper functions for common layout scenarios
impl LayoutEngine {
    /// Create a layout for Warp-style terminal blocks
    pub fn terminal_layout() -> Self {
        Self::new()
            .with_layout_type(LayoutType::Vertical)
            .with_padding(Padding::symmetric(16.0, 20.0))
            .with_spacing(12.0)
    }
    
    /// Create a layout for AI panel sidebar
    pub fn ai_panel_layout() -> Self {
        Self::new()
            .with_layout_type(LayoutType::Vertical)
            .with_padding(Padding::all(16.0))
            .with_spacing(8.0)
    }
    
    /// Create a layout for toolbar/button groups
    pub fn toolbar_layout() -> Self {
        Self::new()
            .with_layout_type(LayoutType::Horizontal)
            .with_padding(Padding::symmetric(8.0, 12.0))
            .with_spacing(6.0)
    }
} 