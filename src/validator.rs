use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExcludeOptions {
    pub case_sensitive: bool,
    pub whole_command: bool,
    pub allow_override: bool,
}

impl Default for ExcludeOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_command: false,
            allow_override: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExcludePatterns {
    pub patterns: Vec<String>,
    #[serde(default)]
    pub options: ExcludeOptions,
}

static PATTERNS: Lazy<RwLock<Option<ExcludePatterns>>> = Lazy::new(|| RwLock::new(None));
static COMPILED_REGEX: Lazy<RwLock<Vec<Regex>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Loads exclusion patterns from the specified YAML file
pub fn load_exclusion_patterns(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Exclusion file not found: {}", file_path));
    }

    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let patterns: ExcludePatterns = serde_yaml::from_str(&contents)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    // Compile regex patterns for faster matching
    let mut regexes = Vec::new();
    for pattern in &patterns.patterns {
        if pattern.starts_with("regex:") {
            let regex_str = &pattern[6..]; // Remove "regex:" prefix
            match Regex::new(regex_str) {
                Ok(re) => regexes.push(re),
                Err(e) => return Err(format!("Invalid regex pattern '{}': {}", regex_str, e)),
            }
        }
    }

    // Store the patterns and compiled regexes
    let mut patterns_write = PATTERNS.write().unwrap();
    *patterns_write = Some(patterns);
    
    let mut regexes_write = COMPILED_REGEX.write().unwrap();
    *regexes_write = regexes;

    Ok(())
}

/// Validates a command against the exclusion patterns
pub fn validate_command(command: &str) -> Result<(), String> {
    let patterns_read = PATTERNS.read().unwrap();
    let patterns = match &*patterns_read {
        Some(p) => p,
        None => return Ok(()), // No patterns loaded, allow all commands
    };

    // SECURITY FIX: Normalize the command using the same parsing logic as the executor
    // This prevents whitespace injection attacks where attackers use tabs/newlines
    // instead of spaces to bypass denylist patterns
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }
    let canonical_command = parts.join(" ");

    let command_to_check = if patterns.options.case_sensitive {
        canonical_command.clone()
    } else {
        canonical_command.to_lowercase()
    };

    // Check plain text patterns
    for pattern in &patterns.patterns {
        if pattern.starts_with("regex:") {
            continue; // Skip regex patterns, they'll be checked separately
        }

        let pattern_to_check = if patterns.options.case_sensitive {
            pattern.clone()
        } else {
            pattern.to_lowercase()
        };

        if patterns.options.whole_command {
            if command_to_check == pattern_to_check {
                // Log security violation for monitoring
                eprintln!("SECURITY ALERT: Command blocked by exact match - Original: '{}', Canonical: '{}', Pattern: '{}'", 
                         command, canonical_command, pattern);
                return Err(format!("Command '{}' is blocked by pattern '{}'", command, pattern));
            }
        } else if command_to_check.contains(&pattern_to_check) {
            // Log security violation for monitoring
            eprintln!("SECURITY ALERT: Command blocked by substring match - Original: '{}', Canonical: '{}', Pattern: '{}'", 
                     command, canonical_command, pattern);
            return Err(format!("Command '{}' is blocked by pattern '{}'", command, pattern));
        }
    }

    // Check regex patterns
    let regexes_read = COMPILED_REGEX.read().unwrap();
    for regex in &*regexes_read {
        if regex.is_match(&command_to_check) {
            // Log security violation for monitoring
            eprintln!("SECURITY ALERT: Command blocked by regex - Original: '{}', Canonical: '{}', Regex: '{}'", 
                     command, canonical_command, regex.as_str());
            return Err(format!(
                "Command '{}' is blocked by regex pattern '{}'",
                command,
                regex.as_str()
            ));
        }
    }

    Ok(())
}

/// Attempts to load exclusion patterns from multiple possible locations
pub fn try_load_exclusion_patterns() -> Result<(), String> {
    let possible_paths = vec![
        "exclude.yaml",
        "/app/exclude.yaml",
        "../exclude.yaml",
        "/exclude.yaml",
    ];

    for path in possible_paths {
        if Path::new(path).exists() {
            return load_exclusion_patterns(path);
        }
    }

    // If no file is found, log a warning but don't fail
    eprintln!("Warning: No exclude.yaml file found. Running without command restrictions.");
    Ok(())
}
