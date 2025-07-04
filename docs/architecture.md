# Liminal Terminal Emulator - Architecture & Data Flow

## Overview

Liminal is a modern, GPU-accelerated terminal emulator built in Rust with integrated AI assistance. It provides a high-performance terminal experience with advanced features like local AI integration, modern UI components, and GPU-accelerated rendering.

## Core Architecture

### Component Overview

```
┌─────────────────┐
│      main.rs    │  Entry point
└─────────┬───────┘
          │
┌─────────▼───────┐
│     App.rs      │  Main application coordinator
└─────────┬───────┘
          │
    ┌─────┴─────┐
    │           │
┌───▼───┐   ┌───▼───┐   ┌─────────┐   ┌─────────┐   ┌─────────┐
│Terminal│   │Renderer│  │ Shell   │   │   AI    │   │   UI    │
│        │   │        │  │ Manager │   │ Client  │   │ Manager │
└───┬───┘   └───┬───┘   └────┬────┘   └────┬────┘   └────┬────┘
    │           │              │             │             │
┌───▼───┐   ┌───▼───┐     ┌───▼───┐     ┌───▼───┐     ┌───▼───┐
│ ANSI  │   │ WGPU  │     │Process│     │Ollama │     │Layout │
│Parser │   │Render │     │ I/O   │     │ API   │     │Engine │
└───────┘   └───────┘     └───────┘     └───────┘     └───────┘
```

### Module Responsibilities

#### 1. Application Coordinator (`app.rs`)
- **Purpose**: Central coordinator for all components
- **Responsibilities**:
  - Initialize all subsystems
  - Handle window events and user input
  - Coordinate data flow between components
  - Manage application lifecycle

#### 2. Terminal Emulator (`terminal.rs`)
- **Purpose**: Core terminal emulation logic
- **Key Components**:
  - `TerminalBuffer`: Stores terminal content in a 2D grid
  - `TerminalCell`: Individual character with styling information
  - `Parser`: VTE-based ANSI escape sequence parser
- **Responsibilities**:
  - Parse ANSI escape sequences
  - Maintain terminal state (cursor, colors, styles)
  - Handle terminal operations (scroll, clear, etc.)

#### 3. GPU Renderer (`renderer.rs`)
- **Purpose**: GPU-accelerated rendering using WGPU
- **Key Components**:
  - WGPU setup and configuration
  - Text rendering pipeline
  - UI element rendering
- **Responsibilities**:
  - Render terminal text with proper styling
  - Render UI overlays (panels, popups, buttons)
  - Handle window resizing and surface management

#### 4. Shell Process Manager (`shell.rs`)
- **Purpose**: Manage shell process I/O
- **Key Components**:
  - Process spawning and management
  - Asynchronous I/O handling
  - Environment setup
- **Responsibilities**:
  - Start and manage shell processes
  - Handle bidirectional communication
  - Process shell output and forward to terminal

#### 5. AI Integration (`ai.rs`)
- **Purpose**: Local AI assistance via Ollama
- **Key Components**:
  - Ollama API client
  - Request/response handling
  - Model management
- **Responsibilities**:
  - Manage Ollama server lifecycle
  - Process AI queries and responses
  - Handle different AI interaction modes

#### 6. UI Management (`ui.rs`)
- **Purpose**: User interface layout and interaction
- **Key Components**:
  - Layout calculation
  - UI element management
  - Event handling
- **Responsibilities**:
  - Calculate window layouts
  - Manage UI state and interactions
  - Coordinate between terminal and AI panels

## Data Flow

### 1. User Input Flow

```
User Input (Keyboard/Mouse)
          ↓
    Winit Event Loop
          ↓
    App::window_event()
          ↓
    ┌─────────────┬─────────────┐
    │             │             │
    ▼             ▼             ▼
Terminal      UI Manager   AI Client
Commands      Interactions  Queries
    │             │             │
    ▼             ▼             ▼
Shell Process   UI State    Ollama API
```

### 2. Shell Output Flow

```
Shell Process Output
        ↓
Shell Manager (receives bytes)
        ↓
Terminal::process_data()
        ↓
VTE Parser (ANSI sequences)
        ↓
Terminal Buffer (updates)
        ↓
Renderer::render_terminal_text()
        ↓
GPU Rendering (WGPU)
```

### 3. AI Integration Flow

```
User AI Query
     ↓
UI Manager (captures input)
     ↓
AI Client::chat()
     ↓
Ollama HTTP API
     ↓
AI Response
     ↓
UI Manager (display response)
     ↓
Renderer (render AI panel)
```

### 4. Rendering Pipeline

```
Frame Start
     ↓
Renderer::render()
     ├─ Clear surface
     ├─ Render terminal background
     ├─ Render terminal text (WGPU + glyph_brush)
     ├─ Render UI elements
     └─ Present frame
```

## Key Features

### GPU-Accelerated Text Rendering

- **WGPU**: Modern graphics API for cross-platform GPU access
- **glyph_brush**: Efficient text rendering with font caching
- **Custom shaders**: WGSL shaders for terminal-specific rendering
- **Performance**: Hardware-accelerated text rendering for smooth scrolling

### ANSI Terminal Emulation

- **VTE Parser**: Industry-standard ANSI escape sequence parsing
- **Full compatibility**: Support for colors, styles, cursor movement
- **Scrollback buffer**: Configurable history retention
- **Modern terminal features**: 256-color support, Unicode handling

### Local AI Integration

- **Ollama integration**: Local LLM execution for privacy
- **Multiple AI modes**:
  - Command generation from natural language
  - Output explanation and analysis
  - General terminal/shell assistance
- **Automatic setup**: Ollama installation detection and model management

### Modern UI Architecture

- **Flexible layouts**: Resizable panels and responsive design
- **Component system**: Modular UI elements (popups, buttons, panels)
- **Event-driven**: Efficient event handling and state management

## Configuration

The application uses a TOML-based configuration system:

```toml
[terminal]
rows = 24
cols = 80
scrollback_limit = 10000
font_family = "JetBrains Mono"
font_size = 14.0

[renderer]
vsync = true
gpu_acceleration = true
background_color = [0.1, 0.1, 0.1, 1.0]
text_color = [0.9, 0.9, 0.9, 1.0]

[ai]
ollama_base_url = "http://localhost:11434"
model_name = "llama3.2"
context_length = 4096
temperature = 0.7
enabled = true

[shell]
shell_command = "/bin/zsh"  # Auto-detected if not specified
working_directory = "/Users/username"
```

## Performance Considerations

### Memory Management
- **Zero-copy where possible**: Minimize data copying between components
- **Efficient buffer management**: Circular buffers for scrollback
- **GPU memory**: Proper VRAM management for textures and buffers

### Concurrency
- **Async/await**: Non-blocking I/O operations
- **Message passing**: Channel-based communication between threads
- **Shared state**: Arc<RwLock<T>> for safe concurrent access

### Rendering Optimization
- **Batch rendering**: Group similar render operations
- **Dirty checking**: Only re-render changed content
- **Font caching**: Efficient glyph atlas management

## Security & Privacy

### Offline-First Design
- **Local AI**: No cloud dependencies for AI features
- **Airgapped operation**: Fully functional without internet
- **Data privacy**: All terminal content stays local

### Process Isolation
- **Shell sandboxing**: Proper process management
- **Resource limits**: Prevent runaway processes
- **Secure defaults**: Conservative configuration options

## Future Enhancements

### Planned Features
- **Themes and customization**: Extended visual customization
- **Plugin system**: Extensible architecture for community plugins
- **Multi-tab support**: Multiple terminal sessions
- **Split panes**: Tiled terminal layouts
- **Session management**: Save and restore terminal sessions

### Performance Improvements
- **Hardware-specific optimizations**: Platform-specific rendering paths
- **Advanced caching**: Smarter content caching strategies
- **Background rendering**: Off-screen preparation of content 