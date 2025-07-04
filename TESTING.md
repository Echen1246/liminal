# 🧪 Testing Guide for Liminal Terminal Emulator

Welcome to testing your Warp-style terminal emulator! This guide covers all testing scenarios.

## 🚀 Quick Start

### 1. **Build & Run**
```bash
# Clean build
cargo clean
cargo build --release

# Run the main application
cargo run

# Or run with debug logging
RUST_LOG=debug cargo run
```

### 2. **Test Individual Components**
```bash
# Test configuration system
cargo run --example test_config

# Test AI integration (requires Ollama)
cargo run --example ollama_integration

# Test WGPU text rendering concepts
cargo run --example wgpu_text_rendering
```

## 📋 Testing Checklist

### ✅ **Core Compilation Tests**
- [ ] `cargo check` - No compilation errors
- [ ] `cargo build` - Successful build
- [ ] `cargo test` - All unit tests pass
- [ ] `cargo clippy` - No linting warnings

### ✅ **Configuration Tests**
- [ ] Config file creation at `~/.config/liminal/config.toml`
- [ ] Default config values are correct
- [ ] Config loading/saving works
- [ ] TOML parsing handles all fields

### ✅ **Terminal Emulation Tests**
- [ ] VTE parser handles ANSI escape sequences
- [ ] Terminal buffer stores text correctly
- [ ] Cursor movement commands work
- [ ] Color escape sequences are parsed
- [ ] Text wrapping functions properly

### ✅ **Shell Integration Tests**
- [ ] Shell process spawning (bash, zsh, cmd)
- [ ] Command execution and output capture
- [ ] Environment variable handling
- [ ] Cross-platform shell detection

### ✅ **AI Integration Tests** (Requires Ollama)
- [ ] Ollama server detection
- [ ] Model availability checking
- [ ] HTTP request/response handling
- [ ] Chat conversation flow
- [ ] Error handling for offline scenarios

### ✅ **Custom UI System Tests**
- [ ] Widget creation (TerminalBlock, Button, AIPanel)
- [ ] Layout engine positioning
- [ ] Event handling (click, hover, keyboard)
- [ ] Theme switching (dark/light)
- [ ] Animation system functionality

### ✅ **Renderer Tests**
- [ ] WGPU initialization
- [ ] Surface creation
- [ ] Font loading and text rendering
- [ ] Window resizing
- [ ] Frame rate performance

## 🛠 Development Testing

### **Run Tests with Coverage**
```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Run tests with coverage report
cargo tarpaulin --out Html
open tarpaulin-report.html
```

### **Performance Testing**
```bash
# Run benchmarks
cargo bench

# Profile with perf (Linux) or Instruments (macOS)
cargo build --release
perf record ./target/release/liminal
```

### **Memory Testing**
```bash
# Run with Valgrind (Linux)
cargo build
valgrind --tool=memcheck ./target/debug/liminal

# Run with sanitizers
RUSTFLAGS="-Z sanitizer=address" cargo run
```

## 🔧 Manual Testing Scenarios

### **Terminal Functionality**
1. **Basic Commands**
   ```bash
   ls -la
   echo "Hello World"
   cat /etc/passwd
   ```

2. **ANSI Colors**
   ```bash
   echo -e "\033[31mRed Text\033[0m"
   echo -e "\033[1;32mBold Green\033[0m"
   ```

3. **Interactive Programs**
   ```bash
   vim
   htop
   nano
   ```

4. **Long Output**
   ```bash
   find / -name "*.txt" 2>/dev/null
   cat /var/log/system.log
   ```

### **AI Integration** (Requires Ollama)
1. **Install DeepSeek R1:1.5B**
   ```bash
   ollama pull deepseek-r1:1.5b
   ```

2. **Test AI Commands**
   - Ask: "How do I list files in Linux?"
   - Ask: "Explain this error message"
   - Ask: "Generate a command to find large files"

3. **Context Testing**
   - Run a command that fails
   - Ask AI to explain the error
   - Ask for suggestions to fix it

### **UI Interaction Testing**
1. **Terminal Blocks** (Warp-style)
   - Click to expand/collapse command output
   - Hover effects on terminal blocks
   - Copy command or output text

2. **AI Panel**
   - Toggle AI panel visibility
   - Type messages and get responses
   - Scroll through conversation history

3. **Theme Switching**
   - Switch between dark and light themes
   - Verify all UI elements update colors
   - Test custom color schemes

## 🚨 Common Issues & Solutions

### **Issue: Config parsing error**
```
Error: Failed to parse config: missing field `shell`
```
**Solution:**
```bash
rm ~/.config/liminal/config.toml
cargo run  # Regenerates default config
```

### **Issue: Ollama connection failed**
```
Error: Failed to connect to Ollama server
```
**Solution:**
```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama
ollama serve

# Pull a small model
ollama pull deepseek-r1:1.5b
```

### **Issue: WGPU initialization failed**
```
Error: Failed to create WGPU adapter
```
**Solution:**
- Update graphics drivers
- Try different backends: `WGPU_BACKEND=vulkan cargo run`
- Check GPU compatibility

### **Issue: Font not found**
```
Error: Failed to load font: JetBrains Mono
```
**Solution:**
```bash
# Install JetBrains Mono font
# macOS: brew install font-jetbrains-mono
# Linux: sudo apt install fonts-jetbrains-mono
# Windows: Download from https://www.jetbrains.com/mono/
```

## 📊 Performance Benchmarks

### **Expected Performance**
- **Startup time**: < 500ms
- **Frame rate**: 60 FPS stable
- **Memory usage**: < 100MB idle
- **Terminal latency**: < 16ms
- **AI response time**: 1-5 seconds (depends on model)

### **Benchmark Commands**
```bash
# Measure startup time
time cargo run --release -- --version

# Test large output performance
seq 1 10000 | cargo run --release

# Memory usage
ps aux | grep liminal
```

## 🔄 Continuous Integration

### **GitHub Actions Workflow**
```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
```

## 📝 Test Reports

### **Generate Test Report**
```bash
# Run all tests and generate report
cargo test -- --nocapture > test_results.txt
cargo clippy > clippy_results.txt
cargo audit > security_audit.txt
```

### **Example Test Output**
```
✅ Configuration system: PASS
✅ Terminal parsing: PASS
✅ Shell integration: PASS
✅ AI integration: PASS (with Ollama)
✅ UI widgets: PASS
✅ Theme system: PASS
✅ Performance: 60 FPS, 45MB memory
```

## 🎯 Next Steps

After testing successfully:

1. **Enhance GPU Renderer**: Implement full WGPU rendering
2. **Add More Widgets**: Tabs, split panes, search
3. **Improve AI**: Add more AI modes and better context
4. **Performance**: Optimize for large outputs
5. **Platform**: Test on Windows and Linux
6. **Features**: Add plugins, themes, shortcuts

Happy testing! 🚀 