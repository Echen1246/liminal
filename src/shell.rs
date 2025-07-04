use crate::errors::*;
use std::collections::HashMap;
use std::env;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ShellEvent {
    Output(Vec<u8>),
    Error(String),
    Exit(i32),
}

pub struct ShellManager {
    shell_process: Option<Child>,
    output_receiver: Option<mpsc::UnboundedReceiver<ShellEvent>>,
    input_sender: Option<mpsc::UnboundedSender<Vec<u8>>>,
    shell_command: String,
    working_directory: std::path::PathBuf,
    environment_vars: HashMap<String, String>,
}

impl ShellManager {
    pub fn new() -> Result<Self> {
        let shell_command = Self::detect_shell();
        let working_directory = env::current_dir()
            .map_err(|e| LiminalError::Shell(format!("Failed to get current directory: {}", e)))?;
        
        Ok(Self {
            shell_process: None,
            output_receiver: None,
            input_sender: None,
            shell_command,
            working_directory,
            environment_vars: HashMap::new(),
        })
    }
    
    fn detect_shell() -> String {
        // Try to detect the user's preferred shell
        if let Ok(shell) = env::var("SHELL") {
            return shell;
        }
        
        // Fallback to common shells based on platform
        #[cfg(target_os = "windows")]
        {
            "cmd".to_string()
        }
        #[cfg(not(target_os = "windows"))]
        {
            "/bin/bash".to_string()
        }
    }
    
    pub async fn start_shell(&mut self) -> Result<()> {
        if self.shell_process.is_some() {
            return Err(LiminalError::Shell("Shell process already running".to_string()));
        }
        
        let (output_tx, output_rx) = mpsc::unbounded_channel();
        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        
        // Prepare environment variables
        let mut env_vars = env::vars().collect::<HashMap<_, _>>();
        env_vars.extend(self.environment_vars.clone());
        
        // Set terminal-specific environment variables
        env_vars.insert("TERM".to_string(), "xterm-256color".to_string());
        env_vars.insert("COLUMNS".to_string(), "80".to_string());
        env_vars.insert("LINES".to_string(), "24".to_string());
        
        let mut command = Command::new(&self.shell_command);
        command
            .current_dir(&self.working_directory)
            .envs(&env_vars)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        
        // Add shell-specific arguments for interactive mode
        if self.shell_command.ends_with("bash") {
            command.arg("-i");
        } else if self.shell_command.ends_with("zsh") {
            command.arg("-i");
        }
        
        let mut child = command.spawn()
            .map_err(|e| LiminalError::Shell(format!("Failed to start shell process: {}", e)))?;
        
        // Get handles to stdin, stdout, and stderr
        let mut stdin = child.stdin.take()
            .ok_or_else(|| LiminalError::Shell("Failed to get shell stdin".to_string()))?;
        let mut stdout = child.stdout.take()
            .ok_or_else(|| LiminalError::Shell("Failed to get shell stdout".to_string()))?;
        let mut stderr = child.stderr.take()
            .ok_or_else(|| LiminalError::Shell("Failed to get shell stderr".to_string()))?;
        
        // Spawn task to handle input to shell
        tokio::spawn(async move {
            while let Some(data) = input_rx.recv().await {
                if let Err(e) = stdin.write_all(&data).await {
                    log::error!("Failed to write to shell stdin: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush().await {
                    log::error!("Failed to flush shell stdin: {}", e);
                    break;
                }
            }
        });
        
        // Spawn task to handle output from shell stdout
        let output_tx_stdout = output_tx.clone();
        tokio::spawn(async move {
            let mut buffer = [0u8; 4096];
            loop {
                match stdout.read(&mut buffer).await {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        if output_tx_stdout.send(ShellEvent::Output(buffer[..n].to_vec())).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = output_tx_stdout.send(ShellEvent::Error(format!("Stdout read error: {}", e)));
                        break;
                    }
                }
            }
        });
        
        // Spawn task to handle output from shell stderr
        let output_tx_stderr = output_tx.clone();
        tokio::spawn(async move {
            let mut buffer = [0u8; 4096];
            loop {
                match stderr.read(&mut buffer).await {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        if output_tx_stderr.send(ShellEvent::Output(buffer[..n].to_vec())).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = output_tx_stderr.send(ShellEvent::Error(format!("Stderr read error: {}", e)));
                        break;
                    }
                }
            }
        });
        
        // Spawn task to wait for child process exit
        tokio::spawn(async move {
            match child.wait().await {
                Ok(status) => {
                    let exit_code = status.code().unwrap_or(-1);
                    let _ = output_tx.send(ShellEvent::Exit(exit_code));
                }
                Err(e) => {
                    let _ = output_tx.send(ShellEvent::Error(format!("Process wait error: {}", e)));
                }
            }
        });
        
        self.output_receiver = Some(output_rx);
        self.input_sender = Some(input_tx);
        
        log::info!("Shell process started: {}", self.shell_command);
        Ok(())
    }
    
    pub async fn send_input(&mut self, data: &[u8]) -> Result<()> {
        if let Some(sender) = &self.input_sender {
            sender.send(data.to_vec())
                .map_err(|_| LiminalError::Shell("Failed to send input to shell".to_string()))?;
            Ok(())
        } else {
            Err(LiminalError::Shell("Shell process not running".to_string()))
        }
    }
    
    pub async fn send_command(&mut self, command: &str) -> Result<()> {
        let mut data = command.as_bytes().to_vec();
        data.push(b'\n');
        self.send_input(&data).await
    }
    
    pub async fn receive_output(&mut self) -> Option<ShellEvent> {
        if let Some(receiver) = &mut self.output_receiver {
            receiver.recv().await
        } else {
            None
        }
    }
    
    pub fn is_running(&self) -> bool {
        self.shell_process.is_some()
    }
    
    pub async fn resize_terminal(&mut self, cols: u16, rows: u16) -> Result<()> {
        // Update environment variables for future commands
        self.environment_vars.insert("COLUMNS".to_string(), cols.to_string());
        self.environment_vars.insert("LINES".to_string(), rows.to_string());
        
        // Send resize signal to current shell if running
        // Note: This is a simplified approach. A full implementation would use
        // PTY (pseudo-terminal) for proper terminal control
        if self.is_running() {
            let resize_command = format!("export COLUMNS={} LINES={}\n", cols, rows);
            self.send_input(resize_command.as_bytes()).await?;
        }
        
        Ok(())
    }
    
    pub fn set_working_directory(&mut self, path: std::path::PathBuf) {
        self.working_directory = path;
    }
    
    pub fn set_environment_variable(&mut self, key: String, value: String) {
        self.environment_vars.insert(key, value);
    }
    
    pub fn get_shell_command(&self) -> &str {
        &self.shell_command
    }
    
    pub fn set_shell_command(&mut self, command: String) {
        self.shell_command = command;
    }
} 