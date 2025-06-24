use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::output::OutputFormat;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Default output format
    pub default_format: Option<OutputFormat>,
    
    /// Default output directory
    pub default_output_dir: Option<PathBuf>,
    
    /// Default categories to filter
    pub default_categories: Option<Vec<String>>,
    
    /// Enable verbose logging by default
    pub verbose: Option<bool>,
    
    /// Default network depth
    pub default_network_depth: Option<u32>,
    
    /// API settings
    pub api: ApiConfig,
    
    /// UI settings
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    /// Custom INSPIRE API base URL
    pub base_url: Option<String>,
    
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
    
    /// Maximum retries for failed requests
    pub max_retries: Option<u32>,
    
    /// Delay between requests (in milliseconds) to avoid rate limiting
    pub request_delay_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    /// Show progress bars
    pub show_progress: Option<bool>,
    
    /// Use colored output
    pub use_colors: Option<bool>,
    
    /// Progress bar style
    pub progress_style: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_format: Some(OutputFormat::Json),
            default_output_dir: None,
            default_categories: None,
            verbose: Some(false),
            default_network_depth: Some(1),
            api: ApiConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: Some("https://inspirehep.net/api".to_string()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            request_delay_ms: Some(100),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_progress: Some(true),
            use_colors: Some(true),
            progress_style: Some("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}".to_string()),
        }
    }
}

impl Config {
    /// Load configuration from file, creating default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        
        println!("Configuration saved to: {}", config_path.display());
        Ok(())
    }
    
    /// Get the path to the configuration file
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        Ok(config_dir.join("reference_tool").join("config.toml"))
    }
    
    /// Show current configuration
    pub fn show(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        println!("Current configuration:");
        println!("{}", content);
        Ok(())
    }
    
    /// Get effective output format (CLI arg or config default)
    pub fn effective_format(&self, cli_format: Option<OutputFormat>) -> OutputFormat {
        cli_format
            .or(self.default_format.clone())
            .unwrap_or(OutputFormat::Json)
    }
    
    /// Get effective output directory
    pub fn effective_output_dir(&self, cli_output: Option<PathBuf>) -> Option<PathBuf> {
        cli_output.or_else(|| self.default_output_dir.clone())
    }
    
    /// Get effective categories
    pub fn effective_categories(&self, cli_categories: Option<String>) -> Option<Vec<String>> {
        cli_categories
            .map(|cats| cats.split(',').map(|s| s.trim().to_string()).collect())
            .or_else(|| self.default_categories.clone())
    }
    
    /// Get effective verbosity
    pub fn effective_verbose(&self, cli_verbose: bool) -> bool {
        cli_verbose || self.verbose.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_format, Some(OutputFormat::Json));
        assert_eq!(config.verbose, Some(false));
        assert_eq!(config.default_network_depth, Some(1));
    }
    
    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(config.default_format, parsed.default_format);
        assert_eq!(config.verbose, parsed.verbose);
    }
    
    #[test]
    fn test_effective_methods() {
        let config = Config::default();
        
        // Test format
        assert_eq!(config.effective_format(Some(OutputFormat::Bibtex)), OutputFormat::Bibtex);
        assert_eq!(config.effective_format(None), OutputFormat::Json);
        
        // Test verbosity
        assert_eq!(config.effective_verbose(true), true);
        assert_eq!(config.effective_verbose(false), false);
        
        // Test categories
        let categories = config.effective_categories(Some("hep-th,hep-ph".to_string()));
        assert_eq!(categories, Some(vec!["hep-th".to_string(), "hep-ph".to_string()]));
    }
}
