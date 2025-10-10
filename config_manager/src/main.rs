use std::fs;
use std::collections::HashMap;

// ============= ENUMS =============

#[derive(Debug, Clone)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
enum ConfigType {
    Database {
        host: String,
        port: u16,
        username: String,
        max_connections: u32,
    },
    Server {
        bind_address: String,
        port: u16,
        ssl_enabled: bool,
    },
    Application {
        name: String,
        version: String,
        debug_mode: bool,
        log_level: LogLevel,
    },
}

// ============= STRUCTS =============

struct Config {
    config_type: ConfigType,
    metadata: ConfigMetadata,
}

struct ConfigMetadata {
    filename: String,
    last_modified: Option<String>,
    is_valid: bool,
}

// ============= ERROR TYPES =============

#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),
    InvalidJson { filename: String, reason: String },
    MissingField { config_type: String, field: String },
    InvalidValue { field: String, value: String, expected: String },
    ValidationFailed(Vec<String>),
    SaveFailed { filename: String, reason: String },
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(filename) => {
                write!(f, "Configuration file not found: {}", filename)
            }
            ConfigError::InvalidJson { filename, reason } => {
                write!(f, "Invalid JSON in {}: {}", filename, reason)
            }
            ConfigError::MissingField { config_type, field } => {
                write!(f, "{} configuration is missing required field: {}", 
                       config_type, field)
            }
            ConfigError::InvalidValue { field, value, expected } => {
                write!(f, "Invalid value '{}' for field '{}': expected {}", 
                       value, field, expected)
            }
            ConfigError::ValidationFailed(errors) => {
                write!(f, "Validation failed with {} errors:\n", errors.len())?;
                for error in errors {
                    write!(f, "  - {}\n", error)?;
                }
                Ok(())
            }
            ConfigError::SaveFailed { filename, reason } => {
                write!(f, "Failed to save {}: {}", filename, reason)
            }
            ConfigError::ParseError(msg) => {
                write!(f, "Parse error: {}", msg)
            }
        }
    }
}

// =============  CONFIG IMPLEMENTATION =============

impl Config {
    fn load_from_file(filename: &str) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(filename)
            .map_err(|_| ConfigError::FileNotFound(filename.to_string()))?;
        
        let config_type = Self::parse_json(&contents)?;
        Self::validate_config(&config_type)?;
        
        Ok(Config {
            config_type,
            metadata: ConfigMetadata {
                filename: filename.to_string(),
                last_modified: None,
                is_valid: true,
            },
        })
    }
    
    fn parse_json(json: &str) -> Result<ConfigType, ConfigError> {
        let mut map: HashMap<String, String> = HashMap::new();
        
        for line in json.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(ConfigError::ParseError(
                    format!("Invalid line format: {}", line)
                ));
            }
            
            map.insert(
                parts[0].trim().to_string(),
                parts[1].trim().to_string(),
            );
        }
        
        let config_type = map.get("type")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("unknown"),
                field: String::from("type"),
            })?;
        
        match config_type.as_str() {
            "database" => Self::parse_database_config(&map),
            "server" => Self::parse_server_config(&map),
            "application" => Self::parse_application_config(&map),
            other => Err(ConfigError::InvalidValue {
                field: String::from("type"),
                value: other.to_string(),
                expected: String::from("database, server, or application"),
            }),
        }
    }
    
    fn parse_database_config(map: &HashMap<String, String>) -> Result<ConfigType, ConfigError> {
        let host = map.get("host")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("database"),
                field: String::from("host"),
            })?
            .clone();
        
        let port_str = map.get("port")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("database"),
                field: String::from("port"),
            })?;
        
        let port = port_str.parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue {
                field: String::from("port"),
                value: port_str.clone(),
                expected: String::from("valid port number (1-65535)"),
            })?;
        
        let username = map.get("username")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("database"),
                field: String::from("username"),
            })?
            .clone();
        
        let max_connections_str = map.get("max_connections")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("database"),
                field: String::from("max_connections"),
            })?;
        
        let max_connections = max_connections_str.parse::<u32>()
            .map_err(|_| ConfigError::InvalidValue {
                field: String::from("max_connections"),
                value: max_connections_str.clone(),
                expected: String::from("positive integer"),
            })?;
        
        Ok(ConfigType::Database {
            host,
            port,
            username,
            max_connections,
        })
    }
    
    fn parse_server_config(map: &HashMap<String, String>) -> Result<ConfigType, ConfigError> {
        let bind_address = map.get("bind_address")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("server"),
                field: String::from("bind_address"),
            })?
            .clone();
        
        let port_str = map.get("port")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("server"),
                field: String::from("port"),
            })?;
        
        let port = port_str.parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue {
                field: String::from("port"),
                value: port_str.clone(),
                expected: String::from("valid port number (1-65535)"),
            })?;
        
        let ssl_str = map.get("ssl_enabled")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("server"),
                field: String::from("ssl_enabled"),
            })?;
        
        let ssl_enabled = match ssl_str.to_lowercase().as_str() {
            "true" | "yes" | "1" => true,
            "false" | "no" | "0" => false,
            _ => return Err(ConfigError::InvalidValue {
                field: String::from("ssl_enabled"),
                value: ssl_str.clone(),
                expected: String::from("true or false"),
            }),
        };
        
        Ok(ConfigType::Server {
            bind_address,
            port,
            ssl_enabled,
        })
    }
    
    fn parse_application_config(map: &HashMap<String, String>) -> Result<ConfigType, ConfigError> {
        let name = map.get("name")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("application"),
                field: String::from("name"),
            })?
            .clone();
        
        let version = map.get("version")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("application"),
                field: String::from("version"),
            })?
            .clone();
        
        let debug_str = map.get("debug_mode")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("application"),
                field: String::from("debug_mode"),
            })?;
        
        let debug_mode = match debug_str.to_lowercase().as_str() {
            "true" | "yes" | "1" => true,
            "false" | "no" | "0" => false,
            _ => return Err(ConfigError::InvalidValue {
                field: String::from("debug_mode"),
                value: debug_str.clone(),
                expected: String::from("true or false"),
            }),
        };
        
        let log_level_str = map.get("log_level")
            .ok_or_else(|| ConfigError::MissingField {
                config_type: String::from("application"),
                field: String::from("log_level"),
            })?;
        
        let log_level = match log_level_str.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warning" => LogLevel::Warning,
            "error" => LogLevel::Error,
            _ => return Err(ConfigError::InvalidValue {
                field: String::from("log_level"),
                value: log_level_str.clone(),
                expected: String::from("debug, info, warning, or error"),
            }),
        };
        
        Ok(ConfigType::Application {
            name,
            version,
            debug_mode,
            log_level,
        })
    }
    
    fn validate_config(config_type: &ConfigType) -> Result<(), ConfigError> {
        let mut errors = Vec::new();
        
        match config_type {
            ConfigType::Database { host, port, username, max_connections } => {
                if host.is_empty() {
                    errors.push(String::from("Host cannot be empty"));
                }
                
                if *port == 0 {
                    errors.push(String::from("Port must be between 1 and 65535"));
                }
                
                if username.is_empty() {
                    errors.push(String::from("Username cannot be empty"));
                }
                
                if *max_connections == 0 {
                    errors.push(String::from("Max connections must be at least 1"));
                }
                
                if *max_connections > 10000 {
                    errors.push(String::from("Max connections seems unreasonably high (>10000)"));
                }
            }
            
            ConfigType::Server { bind_address, port, .. } => {
                if bind_address.is_empty() {
                    errors.push(String::from("Bind address cannot be empty"));
                }
                
                if !bind_address.contains('.') && bind_address != "localhost" {
                    errors.push(format!("'{}' doesn't look like a valid address", bind_address));
                }
                
                if *port == 0 {
                    errors.push(String::from("Port must be between 1 and 65535"));
                }
            }
            
            ConfigType::Application { name, version, .. } => {
                if name.is_empty() {
                    errors.push(String::from("Application name cannot be empty"));
                }
                
                if version.is_empty() {
                    errors.push(String::from("Version cannot be empty"));
                }
                
                let parts: Vec<&str> = version.split('.').collect();
                if parts.len() != 3 {
                    errors.push(format!("Version '{}' should be in format x.y.z", version));
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::ValidationFailed(errors))
        }
    }
    
    fn save_to_file(&self, filename: &str) -> Result<(), ConfigError> {
        let content = self.to_string();
        
        fs::write(filename, content)
            .map_err(|e| ConfigError::SaveFailed {
                filename: filename.to_string(),
                reason: e.to_string(),
            })?;
        
        println!("✓ Configuration saved to: {}", filename);
        Ok(())
    }
    
    fn to_string(&self) -> String {
        let mut output = String::new();
        
        match &self.config_type {
            ConfigType::Database { host, port, username, max_connections } => {
                output.push_str("type: database\n");
                output.push_str(&format!("host: {}\n", host));
                output.push_str(&format!("port: {}\n", port));
                output.push_str(&format!("username: {}\n", username));
                output.push_str(&format!("max_connections: {}\n", max_connections));
            }
            
            ConfigType::Server { bind_address, port, ssl_enabled } => {
                output.push_str("type: server\n");
                output.push_str(&format!("bind_address: {}\n", bind_address));
                output.push_str(&format!("port: {}\n", port));
                output.push_str(&format!("ssl_enabled: {}\n", ssl_enabled));
            }
            
            ConfigType::Application { name, version, debug_mode, log_level } => {
                output.push_str("type: application\n");
                output.push_str(&format!("name: {}\n", name));
                output.push_str(&format!("version: {}\n", version));
                output.push_str(&format!("debug_mode: {}\n", debug_mode));
                
                let level_str = match log_level {
                    LogLevel::Debug => "debug",
                    LogLevel::Info => "info",
                    LogLevel::Warning => "warning",
                    LogLevel::Error => "error",
                };
                output.push_str(&format!("log_level: {}\n", level_str));
            }
        }
        
        output
    }
    
    fn display(&self) {
        println!("\n=== Configuration ===");
        println!("File: {}", self.metadata.filename);
        println!("Valid: {}", self.metadata.is_valid);
        println!();
        
        match &self.config_type {
            ConfigType::Database { host, port, username, max_connections } => {
                println!("Type: Database");
                println!("  Host: {}", host);
                println!("  Port: {}", port);
                println!("  Username: {}", username);
                println!("  Max Connections: {}", max_connections);
            }
            
            ConfigType::Server { bind_address, port, ssl_enabled } => {
                println!("Type: Server");
                println!("  Bind Address: {}", bind_address);
                println!("  Port: {}", port);
                println!("  SSL Enabled: {}", ssl_enabled);
            }
            
            ConfigType::Application { name, version, debug_mode, log_level } => {
                println!("Type: Application");
                println!("  Name: {}", name);
                println!("  Version: {}", version);
                println!("  Debug Mode: {}", debug_mode);
                println!("  Log Level: {:?}", log_level);
            }
        }
        println!("====================\n");
    }
}

// =============  MAIN FUNCTION =============

fn main() {
    println!("=== JSON Config Manager ===\n");
    
    // Create and save a database configuration
    println!("--- Example 1: Database Configuration ---\n");
    
    let db_config = Config {
        config_type: ConfigType::Database {
            host: String::from("localhost"),
            port: 5432,
            username: String::from("admin"),
            max_connections: 100,
        },
        metadata: ConfigMetadata {
            filename: String::from("database.conf"),
            last_modified: None,
            is_valid: true,
        },
    };
    
    db_config.display();
    
    if let Err(e) = db_config.save_to_file("database.conf") {
        println!("Failed to save config: {}", e);
    }
    
    // Create and save a server configuration
    println!("\n--- Example 2: Server Configuration ---\n");
    
    let server_config = Config {
        config_type: ConfigType::Server {
            bind_address: String::from("0.0.0.0"),
            port: 8080,
            ssl_enabled: true,
        },
        metadata: ConfigMetadata {
            filename: String::from("server.conf"),
            last_modified: None,
            is_valid: true,
        },
    };
    
    server_config.display();
    
    if let Err(e) = server_config.save_to_file("server.conf") {
        println!("Failed to save: {}", e);
    }
    
    // Create and save an application configuration
    println!("\n--- Example 3: Application Configuration ---\n");
    
    let app_config = Config {
        config_type: ConfigType::Application {
            name: String::from("MyAwesomeApp"),
            version: String::from("1.0.0"),
            debug_mode: true,
            log_level: LogLevel::Debug,
        },
        metadata: ConfigMetadata {
            filename: String::from("app.conf"),
            last_modified: None,
            is_valid: true,
        },
    };
    
    app_config.display();
    
    if let Err(e) = app_config.save_to_file("app.conf") {
        println!("Failed to save: {}", e);
    }
    
    // Load a configuration from file
    println!("\n--- Example 4: Loading Configuration ---\n");
    
    match Config::load_from_file("database.conf") {
        Ok(loaded_config) => {
            println!("✓ Successfully loaded configuration:");
            loaded_config.display();
        }
        Err(e) => {
            println!("✗ Error loading config: {}", e);
        }
    }
    
    // Demonstrating validation errors
    println!("\n--- Example 5: Validation Errors ---\n");
    
    let invalid_config = ConfigType::Database {
        host: String::from(""),
        port: 0,
        username: String::from(""),
        max_connections: 0,
    };
    
    match Config::validate_config(&invalid_config) {
        Ok(()) => println!("Config is valid"),
        Err(e) => println!("Validation errors:\n{}", e),
    }
    
    println!("\n=== All Examples Complete! ===");
}