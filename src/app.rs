use crate::{
    ai::OllamaClient,
    config::Config,
    errors::*,
    renderer::Renderer,
    shell::ShellManager,
    terminal::Terminal,
    ui::UiManager,
};
use anyhow::Result;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;
use winit::{
    application::ApplicationHandler,
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct App {
    config: Config,
    ollama_client: Arc<RwLock<OllamaClient>>,
    terminal: Arc<RwLock<Terminal>>,
    shell_manager: Arc<RwLock<ShellManager>>,
    renderer: Option<Renderer>,
    ui_manager: Arc<RwLock<UiManager>>,
    window: Option<Arc<Window>>,
    event_loop: Option<EventLoop<()>>,
}

impl App {
    pub async fn new() -> Result<Self> {
        info!("Initializing application components");
        
        let config = Config::load().await?;
        let ollama_client = Arc::new(RwLock::new(OllamaClient::new(&config.ai).await?));
        let terminal = Arc::new(RwLock::new(Terminal::new(&config.terminal)?));
        let shell_manager = Arc::new(RwLock::new(ShellManager::new()?));
        let ui_manager = Arc::new(RwLock::new(UiManager::new()));
        
        let event_loop = EventLoop::new()?;
        
        Ok(Self {
            config,
            ollama_client,
            terminal,
            shell_manager,
            renderer: None,
            ui_manager,
            window: None,
            event_loop: Some(event_loop),
        })
    }
    
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting main application loop");
        
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run_app(self)?;
        
        Ok(())
    }
    
    async fn setup_window_and_renderer(&mut self, window: Arc<Window>) -> Result<()> {
        self.window = Some(window.clone());
        
        // Initialize WGPU renderer
        let renderer = Renderer::new(window.clone(), &self.config.renderer).await?;
        self.renderer = Some(renderer);
        
        // Start shell process
        let mut shell_manager = self.shell_manager.write().await;
        shell_manager.start_shell().await?;
        
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Liminal Terminal")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
            
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );
        
        pollster::block_on(async {
            if let Err(e) = self.setup_window_and_renderer(window).await {
                warn!("Failed to setup window and renderer: {}", e);
            }
        });
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    pollster::block_on(async {
                        let terminal = self.terminal.read().await;
                        let ui_manager = self.ui_manager.read().await;
                        
                        if let Err(e) = renderer.render(&*terminal, &*ui_manager).await {
                            warn!("Render error: {}", e);
                        }
                    });
                }
            }
            _ => {}
        }
    }
} 